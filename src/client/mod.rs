pub trait OpenAIRequest {}

pub trait OpenAIResponse {}

pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        OpenAIClient {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn list_models(&self) -> Result<serde_json::Value, reqwest::Error> {
        let request = self
            .client
            .get("https://api.openai.com/v1/models")
            .bearer_auth(&self.api_key);

        let resp = request.send().await?;
        Ok(resp.json::<serde_json::Value>().await?)
    }
}

pub struct OpenAIRequestBuilder {}
