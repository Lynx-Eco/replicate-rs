use crate::Client;
use anyhow::{ Result, anyhow };
use serde_json::json;
use std::{ collections::HashMap, env };
use futures::StreamExt;

#[allow(dead_code)]
fn get_test_client(auth_token: Option<String>) -> Result<Client> {
    let auth_token = auth_token
        .or_else(|| env::var("REPLICATE_API_TOKEN").ok())
        .ok_or_else(|| anyhow!("No auth token provided"))?;
    Client::new(Some(auth_token))
}

#[allow(dead_code)]
pub async fn run_example(auth_token: Option<String>) -> Result<()> {
    let client = get_test_client(auth_token)?;

    let model = "black-forest-labs/flux-schnell";

    let input: HashMap<String, serde_json::Value> = serde_json
        ::from_value(
            json!({
        "prompt": "a beautiful landscape with a river and mountains, landscape photography, dynamic shot",
        "num_outputs": 1,
        "aspect_ratio": "1:1",
        "output_format": "jpg",
        "output_quality": 80
    })
        )
        .unwrap();

    println!("Starting prediction...");
    match client.run(model, input, None).await {
        Ok(output) => {
            println!("Prediction completed successfully!");
            println!("Raw output: {:?}", output);
            if let Some(images) = output.as_array() {
                println!("Generated {} image(s)", images.len());
                for (i, image) in images.iter().enumerate() {
                    println!("Image {}: {:?}", i + 1, image);
                }
            } else {
                println!("Unexpected output format: {:?}", output);
            }
        }
        Err(e) => {
            eprintln!("Run failed: {}", e);
            if let Some(source) = e.source() {
                eprintln!("Caused by: {}", source);
            }
            let mut current_error = e.source();
            while let Some(error) = current_error {
                eprintln!("  Caused by: {}", error);
                current_error = error.source();
            }
        }
    }

    Ok(())
}
#[allow(dead_code)]
pub async fn search_models_example(auth_token: Option<String>) -> Result<()> {
    let client = get_test_client(auth_token)?;

    let query = "llama";
    let models_page = client.search_models(query).await?;

    for model in &models_page.results {
        if model.owner == "meta" && model.name.starts_with("meta-llama-3") {
            println!("Found Meta Llama 3 model");
            break;
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn streaming_example(auth_token: Option<String>) -> Result<()> {
    let client = get_test_client(auth_token)?;

    let model = "meta/meta-llama-3-8b-instruct";

    let input: HashMap<String, serde_json::Value> = serde_json
        ::from_value(
            json!({
       "top_k": 0,
        "top_p": 0.95,
        "prompt": "Johnny has 8 billion parameters. His friend Tommy has 70 billion parameters. What does this mean when it comes to speed?",
        "max_tokens": 512,
        "temperature": 0.7,
        "system_prompt": "You are a helpful assistant",
        "length_penalty": 1,
        "max_new_tokens": 512,
        "stop_sequences": "<|end_of_text|>,<|eot_id|>",
        "prompt_template": "<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n{system_prompt}<|eot_id|><|start_header_id|>user<|end_header_id|>\n\n{prompt}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n",
        "presence_penalty": 0,
        "log_performance_metrics": false
    })
        )
        .unwrap();

    println!("Starting streaming prediction...");
    let (mut sse_stream, mut err_stream) = client.stream(model, input, None).await?;

    tokio::select! {
        _ = async {
            while let Some(event) = sse_stream.next().await {
                match event.event_type.as_str() {
                    "output" => print!("{} ", event.data),
                    "logs" => println!("Logs: {}", event.data),
                    "done" => {
                        println!("\nStreaming completed");
                        break;
                    },
                    _ => println!("Received event: {:?}", event),
                }
            }
        } => {}
        error = err_stream.next() => {
            if let Some(err) = error {
                eprintln!("Error in stream: {}", err);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_token() -> Option<String> {
        env::var("REPLICATE_API_TOKEN").ok()
    }

    #[tokio::test]
    async fn test_run_example() {
        env_logger::init();
        let auth_token = get_test_token();
        if let Err(e) = run_example(auth_token).await {
            eprintln!("Error running example: {}", e);
            panic!("Test failed");
        }
    }

    #[tokio::test]
    async fn test_search_models_example() {
        env_logger::init();
        let auth_token = get_test_token();
        if let Err(e) = search_models_example(auth_token).await {
            eprintln!("Error searching models: {}", e);
            panic!("Test failed");
        }
    }

    #[tokio::test]
    async fn test_streaming_example() {
        env_logger::init();
        let auth_token = get_test_token();
        if let Err(e) = streaming_example(auth_token).await {
            eprintln!("Error in streaming example: {}", e);
            panic!("Test failed");
        }
    }
}
