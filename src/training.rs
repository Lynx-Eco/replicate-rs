use anyhow::Result;
use serde::{ Deserialize, Serialize };
use crate::client::Client;
use crate::model::Model;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Training {
    pub id: String,
    pub version: String,
    pub status: String,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub logs: Option<String>,
    pub webhook_completed: Option<String>,
}

impl Client {
    pub async fn create_training(
        &self,
        model: &Model,
        version: &str,
        input: serde_json::Value
    ) -> Result<Training> {
        let path = format!("/models/{}/{}/versions/{}/trainings", model.owner, model.name, version);
        let body = serde_json::json!({
            "input": input,
        });
        self.fetch(reqwest::Method::POST, &path, Some(body)).await
    }

    pub async fn get_training(
        &self,
        model: &Model,
        version: &str,
        training_id: &str
    ) -> Result<Training> {
        let path = format!(
            "/models/{}/{}/versions/{}/trainings/{}",
            model.owner,
            model.name,
            version,
            training_id
        );
        self.fetch(reqwest::Method::GET, &path, None).await
    }

    pub async fn cancel_training(
        &self,
        model: &Model,
        version: &str,
        training_id: &str
    ) -> Result<Training> {
        let path = format!(
            "/models/{}/{}/versions/{}/trainings/{}/cancel",
            model.owner,
            model.name,
            version,
            training_id
        );
        self.fetch(reqwest::Method::POST, &path, None).await
    }
}
