use std::time::Duration;
use reqwest::{ Client as ReqwestClient, Method, Url };

use serde::de::DeserializeOwned;
use serde_json::Value;
use anyhow::{ Result, anyhow };

use crate::backoff::{ Backoff, ExponentialBackoff };

const ENV_AUTH_TOKEN: &str = "REPLICATE_API_TOKEN";
const DEFAULT_BASE_URL: &str = "https://api.replicate.com/v1";
const DEFAULT_MAX_RETRIES: u32 = 5;

pub struct Client {
    pub(crate) auth_token: String,
    pub(crate) client: ReqwestClient,
    pub(crate) base_url: String,
    max_retries: u32,
    backoff: Box<dyn Backoff>,
}

impl Client {
    pub fn new(auth_token: Option<String>) -> Result<Self> {
        let auth_token = auth_token
            .or_else(|| std::env::var(ENV_AUTH_TOKEN).ok())
            .ok_or_else(|| anyhow!("No auth token provided"))?;

        Ok(Self {
            auth_token,
            client: ReqwestClient::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
            max_retries: DEFAULT_MAX_RETRIES,
            backoff: Box::new(ExponentialBackoff {
                base: Duration::from_millis(500),
                multiplier: 2.0,
                jitter: Duration::from_millis(50),
            }),
        })
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_backoff(mut self, backoff: Box<dyn Backoff>) -> Self {
        self.backoff = backoff;
        self
    }

    pub async fn fetch<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>
    ) -> Result<T> {
        let url = Url::parse(&format!("{}{}", self.base_url, path))?;
        let mut request = self.client
            .request(method.clone(), url.clone())
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.auth_token));
        // .header("User-Agent", DEFAULT_USER_AGENT);

        if let Some(body) = body {
            request = request.json(&body);
            log::debug!("Request body: {}", serde_json::to_string_pretty(&body).unwrap());
        }

        log::info!("Sending {} request to {}", method, url);

        let mut attempts = 0;
        loop {
            log::debug!("Attempt {} of {}", attempts + 1, self.max_retries + 1);

            let response = request.try_clone().unwrap().send().await?;

            log::debug!("Response status: {}", response.status());

            if response.status().is_success() {
                let json = response.json().await?;
                log::debug!("Successful response received");

                return Ok(json);
            } else {
                log::warn!("Request failed");
            }

            if !self.should_retry(&response, &method) || attempts >= self.max_retries {
                let error_text = response.text().await?;
                log::error!("Request failed: {}", error_text);
                return Err(anyhow!("Request failed: {}", error_text));
            }

            let delay = self.backoff.next_delay(attempts);
            log::info!("Retrying after {} ms", delay.as_millis());
            tokio::time::sleep(delay).await;
            attempts += 1;
        }
    }

    fn should_retry(&self, response: &reqwest::Response, method: &Method) -> bool {
        if method == Method::GET {
            response.status() == 429 ||
                (response.status().as_u16() >= 500 && response.status().as_u16() < 600)
        } else {
            response.status() == 429
        }
    }
}
