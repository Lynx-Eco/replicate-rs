use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use anyhow::{ anyhow, Result };
use futures::{ Stream, StreamExt };
use reqwest::header::{ HeaderMap, HeaderValue };
use serde::{ Deserialize, Serialize };
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::prediction::{ Prediction, PredictionInput, CreatePredictionParams };
use crate::webhook::Webhook;
use crate::identifier::Identifier;
use crate::Client;

#[derive(Debug)]
pub struct InvalidUTF8DataError;

impl fmt::Display for InvalidUTF8DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid UTF-8 data")
    }
}

impl Error for InvalidUTF8DataError {}

const SSE_TYPE_DONE: &str = "done";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSEEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub id: String,
    pub data: String,
}

impl SSEEvent {
    fn decode(input: &str) -> Result<Self> {
        let mut event = SSEEvent {
            event_type: String::new(),
            id: String::new(),
            data: String::new(),
        };

        let mut data = Vec::new();

        for line in input.lines() {
            if let Some((field, value)) = line.split_once(':') {
                let value = value.trim_start();
                match field {
                    "id" => {
                        event.id = value.to_string();
                    }
                    "event" => {
                        event.event_type = value.to_string();
                    }
                    "data" => data.push(value),
                    _ => {} // ignore other fields
                }
            }
        }

        event.data = data.join("\n");

        if !event.data.is_empty() && !event.data.is_ascii() {
            return Err(anyhow!(InvalidUTF8DataError));
        }

        Ok(event)
    }
}

impl fmt::Display for SSEEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.event_type == "output" { write!(f, "{}", self.data) } else { write!(f, "") }
    }
}

impl Client {
    pub async fn stream(
        &self,
        identifier: &str,
        input: PredictionInput,
        webhook: Option<&Webhook>
    ) -> Result<(impl Stream<Item = SSEEvent>, impl Stream<Item = anyhow::Error>)> {
        let id = Identifier::parse(identifier)?;

        let params = CreatePredictionParams {
            webhook: webhook.map(|w| w.url.clone()),
            webhook_events_filter: webhook.map(|w| w.events.clone()),
            stream: Some(true),
            webhook_completed: None,
        };

        let prediction = match id.version {
            Some(version) => {
                self.create_prediction(None, Some(&version), None, Some(input), Some(params)).await?
            }
            None => {
                self.create_prediction(
                    Some(&format!("{}/{}", id.owner, id.name)),
                    None,
                    None,
                    Some(input),
                    Some(params)
                ).await?
            }
        };

        self.stream_prediction(prediction, None).await
    }

    pub async fn stream_prediction(
        &self,
        prediction: Prediction,
        last_event: Option<SSEEvent>
    ) -> Result<(impl Stream<Item = SSEEvent>, impl Stream<Item = anyhow::Error>)> {
        let (sse_tx, sse_rx) = mpsc::channel(64);
        let (err_tx, err_rx) = mpsc::channel(64);

        let url = prediction.urls
            .as_ref()
            .and_then(|urls| urls.get("stream"))
            .ok_or_else(|| {
                anyhow!("streaming not supported or not enabled for this prediction")
            })?;

        let client = Arc::new(self.client.clone());
        let url = url.to_string();

        tokio::spawn(async move {
            let mut headers = HeaderMap::new();
            headers.insert("Accept", HeaderValue::from_static("text/event-stream"));
            headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
            headers.insert("Connection", HeaderValue::from_static("keep-alive"));

            if let Some(event) = last_event {
                headers.insert("Last-Event-ID", HeaderValue::from_str(&event.id).unwrap());
            }

            loop {
                let resp = match client.get(&url).headers(headers.clone()).send().await {
                    Ok(resp) => resp,
                    Err(e) => {
                        let _ = err_tx.send(anyhow!("Failed to send request: {}", e)).await;
                        return;
                    }
                };

                if !resp.status().is_success() {
                    let _ = err_tx.send(
                        anyhow!("Received invalid status code: {}", resp.status())
                    ).await;
                    return;
                }

                let mut stream = resp.bytes_stream();
                let mut buffer = String::new();

                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(chunk) => {
                            buffer.push_str(&String::from_utf8_lossy(&chunk));
                            while let Some(pos) = buffer.find("\n\n") {
                                let event_str = buffer[..pos].to_string();
                                buffer = buffer[pos + 2..].to_string();

                                match SSEEvent::decode(&event_str) {
                                    Ok(event) => {
                                        if let Err(e) = sse_tx.send(event.clone()).await {
                                            let _ = err_tx.send(
                                                anyhow!("Failed to send SSE event: {}", e)
                                            ).await;
                                            return;
                                        }
                                        if event.event_type == SSE_TYPE_DONE {
                                            return;
                                        }
                                    }
                                    Err(e) => {
                                        let _ = err_tx.send(e).await;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let _ = err_tx.send(anyhow!("Error reading stream: {}", e)).await;
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            break;
                        }
                    }
                }
            }
        });

        Ok((ReceiverStream::new(sse_rx), ReceiverStream::new(err_rx)))
    }
}
