use thiserror::Error;
use serde::{ Serialize, Deserialize };
use std::fmt;

#[derive(Error, Debug, Serialize, Deserialize)]
pub struct APIError {
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub title: Option<String>,
    pub status: Option<i32>,
    pub detail: Option<String>,
    pub instance: Option<String>,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut components = Vec::new();
        if let Some(ref error_type) = self.error_type {
            components.push(error_type.clone());
        }
        if let Some(ref title) = self.title {
            components.push(title.clone());
        }
        if let Some(ref detail) = self.detail {
            components.push(detail.clone());
        }

        let output = if !components.is_empty() {
            components.join(": ")
        } else {
            "unknown error".to_string()
        };

        if let Some(ref instance) = self.instance {
            write!(f, "{} ({})", output, instance)
        } else {
            write!(f, "{}", output)
        }
    }
}

impl APIError {
    pub fn from_response(response: &reqwest::Response, data: &[u8]) -> Self {
        let mut api_error: APIError = serde_json::from_slice(data).unwrap_or_else(|_| APIError {
            error_type: None,
            title: None,
            status: None,
            detail: Some(format!("Unknown error: {:?}", String::from_utf8_lossy(data))),
            instance: None,
        });

        if api_error.status.is_none() {
            api_error.status = Some(response.status().as_u16() as i32);
        }

        api_error
    }
}

#[derive(Error, Debug)]
pub struct ModelError {
    pub prediction: crate::prediction::Prediction,
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "model error: {}",
            self.prediction.error
                .as_ref()
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error")
        )
    }
}
