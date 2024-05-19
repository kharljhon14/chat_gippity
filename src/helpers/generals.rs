use std::fs;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{apis::call_request::call_gpt, models::general::llm::Message};

use super::command_line::PrintCommand;

const CODE_TEMPLATE_PATH: &str = r#"D:\rust\web_server\src\code_template.rs"#;
const EXEC_TEMPLATE_PATH: &str = r#"D:\rust\web_server\src\main.rs"#;
const API_SCHEMA_PATH: &str = "/d/rust/web_server/src/schemas/api_schema.json";

// Extend ai function to encourage specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    // Extend the string to encourage only printing  the output
    let msg: String = format!(
        "FUNCTION: {} 
    INSTRUCTION: You are a function printer. 
    You ONLY print the results of functions. Nothing else. No commentary. 
    Here is the input to the function: {}. Print out what the function will return.",
        ai_function_str, func_input
    );

    // Return message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Performs call to GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    callback_fn: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI function
    let extended_msg: Message = extend_ai_function(callback_fn, &msg_context);

    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get GPT response
    let gpt_response_result: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // Handle Success
    match gpt_response_result {
        Ok(gpt_response) => gpt_response,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call GPT"),
    }
}

// Performs call to GPT - Decoded
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    callback_fn: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, callback_fn).await;

    let decoded_response: T = serde_json::from_str(&llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");

    decoded_response
}

// Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;

    Ok(response.status().as_u16())
}

// Get Code Template
pub fn read_code_template_contents() -> String {
    let path = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

pub fn read_exec_main_contents() -> String {
    let path = String::from(EXEC_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// Save nNew Backend Code
pub fn save_backend_code(content: &String) {
    let path = String::from(EXEC_TEMPLATE_PATH);
    fs::write(path, content).expect("Failed to write main.rs file");
}

// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &String) {
    let path = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API endpoints to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy");

        assert_eq!(extended_msg.role, "system".to_string())
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_param = "build be a web server for making stock price api requests".to_string();

        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}
