use std::time::Duration;
use anyhow::{ Result, anyhow };
use tokio::time::sleep;
use crate::client::Client;
use crate::prediction::Prediction;

const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(1);
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour

impl Client {
    pub async fn wait(&self, prediction: &Prediction) -> Result<()> {
        self.wait_with_options(prediction, DEFAULT_POLL_INTERVAL, DEFAULT_TIMEOUT).await
    }

    pub async fn wait_with_options(
        &self,
        prediction: &Prediction,
        poll_interval: Duration,
        timeout: Duration
    ) -> Result<()> {
        let start = std::time::Instant::now();
        let mut current_prediction = prediction.clone();

        loop {
            if start.elapsed() > timeout {
                return Err(anyhow!("Timeout waiting for prediction to complete"));
            }

            if current_prediction.status.is_terminated() {
                return Ok(());
            }

            sleep(poll_interval).await;
            current_prediction = self.get_prediction(&current_prediction.id).await?;
        }
    }

    // pub(crate) async fn get_prediction(&self, prediction_id: &str) -> Result<Prediction> {
    //     self.fetch(reqwest::Method::GET, &format!("/predictions/{}", prediction_id), None).await
    // }
}
