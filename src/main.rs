mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

fn main() {
    let user_request = get_user_response("What webser are we building today?");

    println!("{}", user_request);
}
