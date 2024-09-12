use serde::{ Deserialize, Serialize };
use anyhow::Result;
use crate::client::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub results: Vec<T>,
    pub next: Option<String>,
    pub previous: Option<String>,
}

impl<T> Page<T> {
    pub fn new(results: Vec<T>) -> Self {
        Page {
            results,
            next: None,
            previous: None,
        }
    }
}

impl Client {
    pub async fn paginate<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<Page<T>> {
        self.fetch(reqwest::Method::GET, path, None).await
    }

    pub async fn paginate_next<T: for<'de> Deserialize<'de>>(
        &self,
        page: &Page<T>
    ) -> Result<Option<Page<T>>> {
        if let Some(next_url) = &page.next {
            Ok(Some(self.fetch(reqwest::Method::GET, next_url, None).await?))
        } else {
            Ok(None)
        }
    }
}
