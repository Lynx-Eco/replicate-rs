use serde::{ Deserialize, Serialize };
use anyhow::Result;
use crate::client::Client;
use crate::model::Model;
use crate::paginate::Page;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub models: Option<Vec<Model>>,
}

impl Client {
    pub async fn list_collections(&self) -> Result<Page<Collection>> {
        self.fetch(reqwest::Method::GET, "/collections", None).await
    }

    pub async fn get_collection(&self, slug: &str) -> Result<Collection> {
        self.fetch(reqwest::Method::GET, &format!("/collections/{}", slug), None).await
    }
}
