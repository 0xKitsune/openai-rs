use std::error::Error;

use openai::{
    client,
    completions::{self, CompletionRequest, CompletionResponse},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let open_ai_client = client::OpenAIClient::new(
        std::env::var("OPENAI_API_KEY").expect("Could not get api token"),
    );

    let request = CompletionRequest::new("text-davinci-003", "Can you write me a poem?");

    open_ai_client
        .send_request::<CompletionRequest, CompletionResponse>(request)
        .await?;

    Ok(())
}
