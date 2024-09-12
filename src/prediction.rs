use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use anyhow::Result;
use reqwest::Method;
use regex::Regex;
use anyhow::anyhow;
use crate::status::Status;
use crate::webhook::{ Webhook, WebhookEventType };
use crate::client::Client;
use crate::paginate::Page;
pub type PredictionInput = HashMap<String, serde_json::Value>;
pub type PredictionOutput = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    Web,
    Api,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionMetrics {
    pub predict_time: Option<f64>,
    pub total_time: Option<f64>,
    pub input_token_count: Option<i32>,
    pub output_token_count: Option<i32>,
    pub time_to_first_token: Option<f64>,
    pub tokens_per_second: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub id: String,
    pub status: Status,
    pub model: String,
    pub version: String,
    pub input: PredictionInput,
    pub output: Option<PredictionOutput>,
    #[serde(default)] // This makes the field optional during deserialization
    pub source: Option<Source>,
    pub error: Option<serde_json::Value>,
    pub logs: Option<String>,
    pub metrics: Option<PredictionMetrics>,
    pub webhook: Option<String>,
    pub webhook_events_filter: Option<Vec<WebhookEventType>>,
    pub urls: Option<HashMap<String, String>>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PredictionProgress {
    pub percentage: f64,
    pub current: i32,
    pub total: i32,
}

impl Prediction {
    pub fn progress(&self) -> Option<PredictionProgress> {
        if let Some(logs) = &self.logs {
            if logs.is_empty() {
                return None;
            }

            let re = Regex::new(
                r"^\s*(?P<percentage>\d+)%\s*\|.+?\|\s*(?P<current>\d+)\/(?P<total>\d+)"
            ).unwrap();
            let lines: Vec<&str> = logs.lines().collect();

            for line in lines.iter().rev() {
                let line = line.trim();
                if let Some(captures) = re.captures(line) {
                    let percentage: f64 = captures["percentage"].parse().unwrap();
                    let current: i32 = captures["current"].parse().unwrap();
                    let total: i32 = captures["total"].parse().unwrap();

                    return Some(PredictionProgress {
                        percentage: percentage / 100.0,
                        current,
                        total,
                    });
                }
            }
        }

        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePredictionParams {
    pub webhook: Option<String>,
    pub webhook_completed: Option<String>,
    pub webhook_events_filter: Option<Vec<String>>,
    pub stream: Option<bool>,
}

impl Client {
    pub async fn create_prediction(
        &self,
        model: Option<&str>,
        version: Option<&str>,
        deployment: Option<&str>,
        input: Option<PredictionInput>,
        params: Option<CreatePredictionParams>
    ) -> Result<Prediction> {
        if
            [model.is_some(), version.is_some(), deployment.is_some()]
                .iter()
                .filter(|&&x| x)
                .count() != 1
        {
            return Err(
                anyhow!("Exactly one of 'model', 'version', or 'deployment' must be specified.")
            );
        }

        let mut body = serde_json::json!({
            "input": input.unwrap_or_default(),
        });

        if let Some(version) = version {
            body["version"] = serde_json::json!(version);
        }

        if let Some(params) = params {
            if let Some(webhook) = params.webhook {
                body["webhook"] = serde_json::json!(webhook);
            }
            if let Some(webhook_completed) = params.webhook_completed {
                body["webhook_completed"] = serde_json::json!(webhook_completed);
            }
            if let Some(webhook_events_filter) = params.webhook_events_filter {
                body["webhook_events_filter"] = serde_json::json!(webhook_events_filter);
            }
            if let Some(stream) = params.stream {
                body["stream"] = serde_json::json!(stream);
            }
        }

        let endpoint = if let Some(model) = model {
            format!("/models/{}/predictions", model)
        } else if let Some(deployment) = deployment {
            format!("/deployments/{}/predictions", deployment)
        } else {
            "/predictions".to_string()
        };

        let prediction: Prediction = self.fetch(Method::POST, &endpoint, Some(body)).await?;

        Ok(prediction)
    }

    pub async fn list_predictions(&self) -> Result<Page<Prediction>> {
        self.fetch(Method::GET, "/predictions", None).await
    }

    pub async fn get_prediction(&self, id: &str) -> Result<Prediction> {
        self.fetch(Method::GET, &format!("/predictions/{}", id), None).await
    }

    pub async fn cancel_prediction(&self, id: &str) -> Result<Prediction> {
        self.fetch(Method::POST, &format!("/predictions/{}/cancel", id), None).await
    }

    pub async fn create_prediction_with_model(
        &self,
        owner: &str,
        name: &str,
        input: PredictionInput,
        webhook: Option<&Webhook>,
        stream: bool
    ) -> Result<Prediction> {
        let mut data = serde_json::json!({
            "input": input,
        });

        if let Some(webhook) = webhook {
            data["webhook"] = serde_json::json!(webhook.url);
            if !webhook.events.is_empty() {
                data["webhook_events_filter"] = serde_json::json!(webhook.events);
            }
        }

        if stream {
            data["stream"] = serde_json::json!(true);
        }

        self.fetch(
            Method::POST,
            &format!("/models/{}/{}/predictions", owner, name),
            Some(data)
        ).await
    }
}
