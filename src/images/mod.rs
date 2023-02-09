use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{OpenAIRequest, OpenAIResponse};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateImageRequest {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

const CREATE_IMAGE_ENDPOINT: &str = "https://api.openai.com/v1/images/generations";

impl OpenAIRequest for CreateImageRequest {
    fn endpoint(&self) -> &str {
        CREATE_IMAGE_ENDPOINT
    }
}

impl CreateImageRequest {
    pub fn new(prompt: &str) -> Self {
        CreateImageRequest {
            prompt: prompt.to_owned(),
            ..Default::default()
        }
    }

    pub fn n(mut self, n: i32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn size(mut self, size: String) -> Self {
        self.size = Some(size);
        self
    }

    pub fn response_format(mut self, response_format: String) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateImageResponse {
    pub created: usize,
    pub data: Vec<HashMap<String, String>>,
}

pub struct ImageUrl {
    pub url: String,
}

impl OpenAIResponse for CreateImageResponse {}
