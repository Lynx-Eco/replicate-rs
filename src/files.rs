use std::collections::HashMap;
use std::path::Path;
use std::fs::File as FsFile;
use std::io::Read;
use anyhow::{ Result, anyhow };
use reqwest::multipart::{ Form, Part };
use serde::{ Deserialize, Serialize };
use mime_guess::from_path;
use crate::client::Client;
use crate::paginate::Page;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub name: String,
    pub content_type: String,
    pub size: i64,
    pub etag: String,
    pub checksums: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub urls: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFileOptions {
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

impl Client {
    pub async fn create_file_from_path(
        &self,
        file_path: &Path,
        options: Option<CreateFileOptions>
    ) -> Result<File> {
        let mut file = FsFile::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let options = options.unwrap_or_default();
        let filename = options.filename.unwrap_or_else(||
            file_path.file_name().unwrap().to_string_lossy().into_owned()
        );
        let content_type = options.content_type.unwrap_or_else(||
            from_path(file_path).first_or_octet_stream().essence_str().to_string()
        );

        self.create_file(&buffer, filename, content_type, options.metadata).await
    }

    pub async fn create_file_from_bytes(
        &self,
        data: &[u8],
        options: Option<CreateFileOptions>
    ) -> Result<File> {
        let options = options.unwrap_or_default();
        let filename = options.filename.unwrap_or_else(|| "file".to_string());
        let content_type = options.content_type.unwrap_or_else(||
            "application/octet-stream".to_string()
        );

        self.create_file(data, filename, content_type, options.metadata).await
    }

    async fn create_file(
        &self,
        data: &[u8],
        filename: String,
        content_type: String,
        metadata: Option<HashMap<String, String>>
    ) -> Result<File> {
        let mut form = Form::new().part(
            "content",
            Part::bytes(data.to_vec()).file_name(filename).mime_str(&content_type)?
        );

        if let Some(metadata) = metadata {
            form = form.part("metadata", Part::text(serde_json::to_string(&metadata)?));
        }

        let response = self.client
            .post(&format!("{}/files", self.base_url))
            .multipart(form)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to create file: {}", response.text().await?));
        }

        let file: File = response.json().await?;
        Ok(file)
    }

    pub async fn list_files(&self) -> Result<Page<File>> {
        self.fetch(reqwest::Method::GET, "/files", None).await
    }

    pub async fn get_file(&self, file_id: &str) -> Result<File> {
        self.fetch(reqwest::Method::GET, &format!("/files/{}", file_id), None).await
    }

    pub async fn delete_file(&self, file_id: &str) -> Result<()> {
        self.fetch(reqwest::Method::DELETE, &format!("/files/{}", file_id), None).await
    }
}
