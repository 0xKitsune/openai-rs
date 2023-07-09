use std::error::Error;

use openairs::{
    client,
    completions::{ChatCompletionRequest, ChatCompletionResponse, Message},
    models::{self},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("Could not get api token");
    let open_ai_client = client::OpenAIClient::new(api_key);

    // Create a new prompt
    let messages = vec![Message::new(
        "Can you write me a rust program that prints hello world?",
    )];
    println!("{}", messages[0].content);

    // Create a new text-davinci-003 model
    let model = models::GPT_4;

    //use the client.complete() method to send a prompt to the completion endpoint
    let response = open_ai_client.chat_completion(&model, messages).await?;
    println!("{} \n\n", response.choices[0].message.content);

    // Create a new prompt
    let messages = vec![Message::new(
        "Can you write me a poem about the Rust programming language?",
    )];

    println!("{}", messages[0].content);

    // Create a new completion request with a custom temperature
    let request = ChatCompletionRequest::new(model.name, messages).temperature(0.3);

    // Send the completion request
    let response = open_ai_client
        .send_request::<ChatCompletionRequest, ChatCompletionResponse>(request)
        .await?;

    println!("{} \n\n", response.choices[0].message.content);

    Ok(())
}
