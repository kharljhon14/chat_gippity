use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// Call LLM GPT 4 turbo
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API key
    let api_key = env::var("OPEN_AI_KEY").expect("Key not found in enviroment");
    let api_org = env::var("OPEN_AI_ORG").expect("Org not found in enviroment");

    let url = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers = HeaderMap::new();

    // Create API key header
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );
    // Create Open AI org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) });

    // Create chat completion
    let chat_completion = ChatCompletion {
        model: "gpt-4-turbo".to_string(),
        temperature: 0.1,
        messages,
    };

    todo!()

    // Troubleshooting
    // let response_raw = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await
    //     .unwrap();

    // dbg!(response_raw.text().await.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Give me a short response".to_string(),
        };

        let messages = vec![message];

        call_gpt(messages).await;
    }
}
