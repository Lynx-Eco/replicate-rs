use anyhow::{ Result, anyhow, Context };
use crate::client::Client;
use crate::prediction::{ PredictionInput, PredictionOutput, CreatePredictionParams };
use crate::webhook::Webhook;
use crate::identifier::Identifier;
use tokio::time::{ sleep, Duration };
use crate::Status;
impl Client {
    pub async fn run(
        &self,
        identifier: &str,
        input: PredictionInput,
        webhook: Option<&Webhook>
    ) -> Result<PredictionOutput> {
        let id = Identifier::parse(identifier)?;

        let params = CreatePredictionParams {
            webhook: webhook.map(|w| w.url.clone()),
            webhook_events_filter: webhook.map(|w| w.events.clone()),
            stream: Some(false),
            webhook_completed: None,
        };

        let mut prediction = match id.version {
            Some(version) => {
                self
                    .create_prediction(None, Some(&version), None, Some(input), Some(params)).await
                    .context("Failed to create prediction with version")?
            }
            None => {
                self
                    .create_prediction(
                        Some(&format!("{}/{}", id.owner, id.name)),
                        None,
                        None,
                        Some(input),
                        Some(params)
                    ).await
                    .context("Failed to create prediction with model")?
            }
        };

        let timeout = Duration::from_secs(600); // 10 minutes timeout
        let start_time = std::time::Instant::now();

        while prediction.status == Status::Starting || prediction.status == Status::Processing {
            if start_time.elapsed() > timeout {
                return Err(anyhow!("Prediction timed out after {:?}", timeout));
            }
            sleep(Duration::from_secs(5)).await;
            prediction = self.get_prediction(&prediction.id).await?;
        }

        if prediction.status == Status::Succeeded {
            prediction.output.ok_or_else(|| anyhow!("Prediction succeeded but no output available"))
        } else if prediction.status == Status::Failed {
            Err(anyhow!("Prediction failed: {:?}", prediction.error))
        } else {
            Err(
                anyhow!(
                    "Unexpected prediction status: {:?}. Logs: {:?}",
                    prediction.status,
                    prediction.logs
                )
            )
        }
    }
}
