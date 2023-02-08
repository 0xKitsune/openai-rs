use std::error::Error;

use openai::{
    client,
    completions::{self, CompletionRequest, CompletionResponse},
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let open_ai_client = client::OpenAIClient::new(
        std::env::var("OPENAI_API_KEY").expect("Could not get api token"),
    );

    let request = CompletionRequest::new("text-davinci-003", "Can you write me a poem?");

    dbg!(json!(request.clone()));

    let response = open_ai_client
        .send_request::<CompletionRequest, CompletionResponse>(request)
        .await?;

    Ok(())
}
