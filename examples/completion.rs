use std::error::Error;

use openairs::{
    client,
    completions::Message,
    messages,
    models::{self},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("Could not get api token");
    let open_ai_client = client::OpenAIClient::new(api_key);

    // Create a new model
    let model = models::GPT_4o_Mini;

    // Create a new prompt
    let msg = messages!("Can you write me a simple Rust server that can handle a 'Ping' request, returning a 'Pong' message?");
    println!("{}", msg[0].content);

    // Send the chat completion request to OpenAI
    let response = open_ai_client.chat_completion(&model, msg).await?;
    println!("{} \n\n", response.choices[0].message.content);

    Ok(())
}
