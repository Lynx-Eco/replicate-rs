use serde::{ Deserialize, Serialize };
use anyhow::Result;
use crate::client::Client;
use crate::account::Account;
use crate::prediction::{ Prediction, PredictionInput };
use crate::webhook::Webhook;
use crate::paginate::Page;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    pub owner: String,
    pub name: String,
    pub current_release: DeploymentRelease,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRelease {
    pub number: i32,
    pub model: String,
    pub version: String,
    pub created_at: String,
    pub created_by: Account,
    pub configuration: DeploymentConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfiguration {
    pub hardware: String,
    pub min_instances: i32,
    pub max_instances: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeploymentOptions {
    pub name: String,
    pub model: String,
    pub version: String,
    pub hardware: String,
    pub min_instances: i32,
    pub max_instances: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeploymentOptions {
    pub model: Option<String>,
    pub version: Option<String>,
    pub hardware: Option<String>,
    pub min_instances: Option<i32>,
    pub max_instances: Option<i32>,
}

impl Client {
    pub async fn create_prediction_with_deployment(
        &self,
        deployment_owner: &str,
        deployment_name: &str,
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

        let path = format!("/deployments/{}/{}/predictions", deployment_owner, deployment_name);
        self.fetch(reqwest::Method::POST, &path, Some(data)).await
    }

    pub async fn get_deployment(
        &self,
        deployment_owner: &str,
        deployment_name: &str
    ) -> Result<Deployment> {
        let path = format!("/deployments/{}/{}", deployment_owner, deployment_name);
        self.fetch(reqwest::Method::GET, &path, None).await
    }

    pub async fn list_deployments(&self) -> Result<Page<Deployment>> {
        self.fetch(reqwest::Method::GET, "/deployments", None).await
    }

    pub async fn create_deployment(&self, options: CreateDeploymentOptions) -> Result<Deployment> {
        self.fetch(
            reqwest::Method::POST,
            "/deployments",
            Some(serde_json::to_value(options)?)
        ).await
    }

    pub async fn update_deployment(
        &self,
        deployment_owner: &str,
        deployment_name: &str,
        options: UpdateDeploymentOptions
    ) -> Result<Deployment> {
        let path = format!("/deployments/{}/{}", deployment_owner, deployment_name);
        self.fetch(reqwest::Method::PATCH, &path, Some(serde_json::to_value(options)?)).await
    }

    pub async fn delete_deployment(
        &self,
        deployment_owner: &str,
        deployment_name: &str
    ) -> Result<()> {
        let path = format!("/deployments/{}/{}", deployment_owner, deployment_name);
        self.fetch(reqwest::Method::DELETE, &path, None).await
    }
}
