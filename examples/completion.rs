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

    //Create a new prompt
    let prompt = "Can you write me a rust program that prints hello world?";
    println!("{}", prompt);

    //Create a new text-davinci-003 model
    let model = models::TEXT_DAVINCI_003;

    //use the client.complete() method to send a prompt to the completion endpoint
    let response = open_ai_client.complete(&model, prompt).await?;
    println!("{} \n\n", response.choices[0].text);

    //Create a new prompt
    let prompt = "Can you write me a poem about the Rust programming language?";
    println!("{}", prompt);

    //Create a new completion request with a custom temperature
    let request = CompletionRequest::new("text-davinci-003", prompt).temperature(0.3);

    //Send the completion request
    let response = open_ai_client
        .send_request::<CompletionRequest, CompletionResponse>(request)
        .await?;

    println!("{}", response.choices[0].text);

    Ok(())
}
