#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::model::Model;
    use crate::prediction::Prediction;
    use crate::webhook::Webhook;
    use mockito::{mock, server_url};
    use serde_json::json;

    #[tokio::test]
    async fn test_new_client_no_auth() {
        let result = Client::new(None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No auth token provided");
    }

    #[tokio::test]
    async fn test_new_client_blank_auth_token_from_env() {
        std::env::set_var("REPLICATE_API_TOKEN", "");
        let result = Client::new_from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("REPLICATE_API_TOKEN"));
    }

    #[tokio::test]
    async fn test_new_client_auth_token_from_env() {
        std::env::set_var("REPLICATE_API_TOKEN", "test-token");
        let result = Client::new_from_env();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_collections() {
        let mock_server = mock("GET", "/collections")
            .with_status(200)
            .with_body(r#"
                {
                    "results": [
                        {"slug": "collection-1", "name": "Collection 1", "description": ""},
                        {"slug": "collection-2", "name": "Collection 2", "description": ""}
                    ]
                }
            "#)
            .create();

        let client = Client::new(Some("test-token".to_string())).unwrap();
        client.set_base_url(&server_url());

        let collections = client.list_collections().await.unwrap();

        assert_eq!(collections.len(), 2);
        assert_eq!(collections[0].slug, "collection-1");
        assert_eq!(collections[0].name, "Collection 1");
        assert_eq!(collections[1].slug, "collection-2");
        assert_eq!(collections[1].name, "Collection 2");

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_get_collection() {
        let mock_server = mock("GET", "/collections/super-resolution")
            .with_status(200)
            .with_body(r#"
                {
                    "name": "Super resolution",
                    "slug": "super-resolution",
                    "description": "Upscaling models that create high-quality images from low-quality images.",
                    "models": []
                }
            "#)
            .create();

        let client = Client::new(Some("test-token".to_string())).unwrap();
        client.set_base_url(&server_url());

        let collection = client.get_collection("super-resolution").await.unwrap();

        assert_eq!(collection.name, "Super resolution");
        assert_eq!(collection.slug, "super-resolution");
        assert_eq!(collection.description, "Upscaling models that create high-quality images from low-quality images.");
        assert!(collection.models.is_empty());

        mock_server.assert();
    }

    // Add more tests here...
}