use std::error::Error;

use openairs::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("Could not get api token");
    let open_ai_client = client::OpenAIClient::new(api_key);

    //Create a new prompt
    let prompt = "Spiderman, but as a Wes Anderson movie";

    println!("Prompt: {}", prompt);

    //use the client.create_image() method to send a prompt to the completion endpoint
    let response = open_ai_client.create_image(prompt).await?;
    println!("{}", response.data[0].url);

    Ok(())
}
