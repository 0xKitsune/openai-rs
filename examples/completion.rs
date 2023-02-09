use std::error::Error;

use openai::{
    client,
    completions::{CompletionRequest, CompletionResponse},
    models::{self},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let open_ai_client = client::OpenAIClient::new(
        std::env::var("OPENAI_API_KEY").expect("Could not get api token"),
    );

    let prompt = "What is a good name for a Rust library that wraps the openai api?";
    println!("{:?}", prompt);

    let model = models::TEXT_DAVINCI_003;
    let response = open_ai_client.complete(&model, prompt).await?;

    println!("{} \n\n", response.choices[0].text);

    let prompt = "Can you write me a poem about the Rust programming language?";
    println!("{:?}", prompt);

    let request = CompletionRequest::new("text-davinci-003", prompt);

    let response = open_ai_client
        .send_request::<CompletionRequest, CompletionResponse>(request)
        .await?;

    println!("{}", response.choices[0].text);

    Ok(())
}
