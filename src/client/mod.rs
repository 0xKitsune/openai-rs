use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use crate::{
    completions::{ChatCompletionRequest, ChatCompletionResponse, Message},
    error::OpenAIError,
    images::{CreateImageRequest, ImageResponse},
    models::Model,
};

pub trait OpenAIRequest {
    fn endpoint(&self) -> &str;
}

pub trait OpenAIResponse {}

pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
}

pub const MODELS_ENDPOINT: &str = "https://api.openai.com/v1/models";

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        OpenAIClient {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn list_models(&self) -> Result<serde_json::Value, reqwest::Error> {
        let request = self.client.get(MODELS_ENDPOINT).bearer_auth(&self.api_key);

        let resp = request.send().await?;
        resp.json::<serde_json::Value>().await
    }

    pub async fn chat_completion(
        &self,
        model: &Model,
        messages: Vec<Message>,
    ) -> Result<ChatCompletionResponse, OpenAIError> {
        let total_chars: usize = messages.iter().map(|m| m.content.chars().count()).sum();

        //TODO: Add error handling for when the model max tokens < prompt length
        self.send_request::<ChatCompletionRequest, ChatCompletionResponse>(
            ChatCompletionRequest::new(model.name, messages)
                .max_tokens(model.max_tokens - total_chars),
        )
        .await
    }

    pub async fn create_image(&self, prompt: &str) -> Result<ImageResponse, OpenAIError> {
        self.send_request::<CreateImageRequest, ImageResponse>(CreateImageRequest::new(prompt))
            .await
    }

    pub async fn send_request<
        Req: OpenAIRequest + Serialize,
        Res: OpenAIResponse + DeserializeOwned,
    >(
        &self,
        request: Req,
    ) -> Result<Res, OpenAIError> {
        let body = json!(request);

        let request_builder = self
            .client
            .post(request.endpoint())
            .json(&body)
            .bearer_auth(&self.api_key);

        let response = request_builder.send().await?.json::<Value>().await?;

        if response.get("error").is_some() {
            let err_json = json!(response.get("error").unwrap());

            match err_json
                .get("type")
                .expect("No 'type' sent with error message")
                .as_str()
                .unwrap()
            {
                "null" => match err_json
                    .get("type")
                    .expect("No 'type' sent with error message")
                    .as_str()
                    .unwrap()
                {
                    "rate_limit_exceeded" => {
                        Err(OpenAIError::RateLimitExceeded(err_json.to_string()))
                    }
                    _ => Err(OpenAIError::UnrecognizedError(err_json.to_string())),
                },
                "billing_not_active" => Err(OpenAIError::BillingNotActive(err_json.to_string())),
                "invalid_request_error" => Err(OpenAIError::InvalidRequest(err_json.to_string())),

                _ => Err(OpenAIError::UnrecognizedError(err_json.to_string())),
            }
        } else {
            //Handle the error
            Ok(Res::deserialize(response).expect("need to handle this error"))
        }
    }
}
