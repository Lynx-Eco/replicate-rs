use serde::{ Deserialize, Serialize };
use anyhow::Result;
use crate::client::Client;
use crate::paginate::Page;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub url: String,
    pub events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub created_at: String,
    pub destination: String,
    pub event_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEventType {
    Start,
    Output,
    Logs,
    Completed,
    // Add any other event types as needed
}

impl Client {
    pub async fn list_webhook_events(&self) -> Result<Page<WebhookEvent>> {
        self.fetch(reqwest::Method::GET, "/webhook-events", None).await
    }

    pub async fn get_webhook_event(&self, event_id: &str) -> Result<WebhookEvent> {
        self.fetch(reqwest::Method::GET, &format!("/webhook-events/{}", event_id), None).await
    }
}
