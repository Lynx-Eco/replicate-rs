use serde::{ Deserialize, Serialize };
use crate::prediction::Prediction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub url: String,
    pub owner: String,
    pub name: String,
    pub description: String,
    pub visibility: String,
    pub github_url: String,
    pub paper_url: String,
    pub license_url: String,
    pub run_count: i32,
    pub cover_image_url: String,
    pub default_example: Option<Prediction>,
    pub latest_version: Option<ModelVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub id: String,
    pub created_at: String,
    pub cog_version: String,
    pub openapi_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelOptions {
    pub visibility: String,
    pub hardware: String,
    pub description: Option<String>,
    pub github_url: Option<String>,
    pub paper_url: Option<String>,
    pub license_url: Option<String>,
    pub cover_image_url: Option<String>,
}
