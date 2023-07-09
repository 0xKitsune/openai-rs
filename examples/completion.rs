use std::error::Error;

use openairs::{
    client,
    completions::Message,
    models::{self},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("Could not get api token");
    let open_ai_client = client::OpenAIClient::new(api_key);

    // Create a new prompt
    let messages = vec![Message::new(
        "Can you write me a simple Rust server that can handle a 'Ping' request, returning a 'Pong' message?",
    )];
    println!("{}", messages[0].content);
    // Create a new model
    let model = models::GPT_4;

    //use the client.complete() method to send a prompt to the completion endpoint
    let response = open_ai_client.chat_completion(&model, messages).await?;
    println!("{} \n\n", response.choices[0].message.content);

    Ok(())
}
