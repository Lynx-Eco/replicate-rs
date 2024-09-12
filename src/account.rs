use serde::{ Deserialize, Serialize };
use crate::client::Client;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "type")]
    pub account_type: String,
    pub username: String,
    pub name: String,
    pub github_url: String,
}

impl Account {
    pub fn new(account_type: String, username: String, name: String, github_url: String) -> Self {
        Self {
            account_type,
            username,
            name,
            github_url,
        }
    }
}

impl Client {
    pub async fn get_current_account(&self) -> Result<Account> {
        let account: Account = self.fetch(
            reqwest::Method::GET,
            "/account",
            None::<serde_json::Value>
        ).await?;
        Ok(account)
    }
}
