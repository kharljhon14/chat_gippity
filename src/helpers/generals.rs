use crate::{apis::call_request::call_gpt, models::general::llm::Message};

use super::command_line::PrintCommand;

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
