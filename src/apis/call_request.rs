use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// Call LLM GPT 3.5 turbo
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    // Extract API key
    let api_key = env::var("OPEN_AI_KEY").expect("Key not found in enviroment");
    let api_org = env::var("OPEN_AI_ORG").expect("Org not found in enviroment");

    let url = "https://api/openapi.com/v1/chat/completions";

    // Create headers
    let mut headers = HeaderMap::new();

    // Create API key header
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    // Create Open AI org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Create client
    let client = Client::builder().default_headers(headers).build().unwrap();

    // Creat chat completion
    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        temperature: 0.1,
        messages,
    };

    // Troubleshooting
    let response_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(response_raw.text().await.unwrap());
}
