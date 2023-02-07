use std::error::Error;

use openai::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let open_ai_client = client::OpenAIClient::new(
        std::env::var("OPENAI_API_KEY").expect("Could not get api token"),
    );

    let x = open_ai_client.list_models().await;

    Ok(())
}
