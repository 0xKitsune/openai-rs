use std::error::Error;

use openairs::{
    client,
    models::{self},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let open_ai_client = client::OpenAIClient::new(
        std::env::var("OPENAI_API_KEY").expect("Could not get api token"),
    );

    //Create a new prompt
    let input = "What day of the wek is it?";
    let instruction = "Fix the spelling mistakes";

    println!("Input: {}", input);

    //Create a new text-edit-001 model
    let model = models::TEXT_DAVINCI_EDIT_001;

    //use the client.edit() method to send a prompt to the completion endpoint
    let response = open_ai_client.edit(&model, input, instruction).await?;
    println!("{}", response.choices[0].text);

    Ok(())
}
