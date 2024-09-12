use crate::client::Client;
use crate::model::{ Model, ModelVersion, CreateModelOptions };
use crate::paginate::Page;
use anyhow::Result;
use reqwest::Method;
use serde_json::json;

impl Client {
    pub async fn list_models(&self) -> Result<Page<Model>> {
        self.fetch(Method::GET, "/models", None).await
    }

    pub async fn search_models(&self, query: &str) -> Result<Page<Model>> {
        self.fetch(Method::GET, "/models", Some(json!(query))).await
    }

    pub async fn get_model(&self, model_owner: &str, model_name: &str) -> Result<Model> {
        self.fetch(Method::GET, &format!("/models/{}/{}", model_owner, model_name), None).await
    }

    pub async fn create_model(
        &self,
        model_owner: &str,
        model_name: &str,
        options: CreateModelOptions
    ) -> Result<Model> {
        let body =
            json!({
            "owner": model_owner,
            "name": model_name,
            "visibility": options.visibility,
            "hardware": options.hardware,
            "description": options.description,
            "github_url": options.github_url,
            "paper_url": options.paper_url,
            "license_url": options.license_url,
            "cover_image_url": options.cover_image_url,
        });

        self.fetch(Method::POST, "/models", Some(body)).await
    }

    pub async fn delete_model(&self, model_owner: &str, model_name: &str) -> Result<()> {
        self.fetch(Method::DELETE, &format!("/models/{}/{}", model_owner, model_name), None).await
    }

    pub async fn list_model_versions(
        &self,
        model_owner: &str,
        model_name: &str
    ) -> Result<Page<ModelVersion>> {
        self.fetch(
            Method::GET,
            &format!("/models/{}/{}/versions", model_owner, model_name),
            None
        ).await
    }

    pub async fn get_model_version(
        &self,
        model_owner: &str,
        model_name: &str,
        version_id: &str
    ) -> Result<ModelVersion> {
        self.fetch(
            Method::GET,
            &format!("/models/{}/{}/versions/{}", model_owner, model_name, version_id),
            None
        ).await
    }

    pub async fn delete_model_version(
        &self,
        model_owner: &str,
        model_name: &str,
        version_id: &str
    ) -> Result<()> {
        self.fetch(
            Method::DELETE,
            &format!("/models/{}/{}/versions/{}", model_owner, model_name, version_id),
            None
        ).await
    }

    // pub async fn create_prediction_with_model(
    //     &self,
    //     model_owner: &str,
    //     model_name: &str,
    //     input: serde_json::Value,
    //     webhook: Option<&Webhook>,
    //     stream: bool
    // ) -> Result<Prediction> {
    //     let mut data = json!({
    //         "input": input,
    //     });

    //     if let Some(webhook) = webhook {
    //         data["webhook"] = json!(webhook.url);
    //         if !webhook.events.is_empty() {
    //             data["webhook_events_filter"] = json!(webhook.events);
    //         }
    //     }

    //     if stream {
    //         data["stream"] = json!(true);
    //     }

    //     self.fetch(
    //         Method::POST,
    //         &format!("/models/{}/{}/predictions", model_owner, model_name),
    //         Some(data)
    //     ).await
    // }
}
