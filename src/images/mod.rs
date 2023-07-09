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
pub struct ImageResponse {
    pub created: usize,
    pub data: Vec<ImageUrl>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImageUrl {
    pub url: String,
}

impl OpenAIResponse for ImageResponse {}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImageEditRequest {
    pub image: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

const IMAGE_EDIT_ENDPOINT: &str = "https://api.openai.com/v1/images/edits";

impl OpenAIRequest for ImageEditRequest {
    fn endpoint(&self) -> &str {
        IMAGE_EDIT_ENDPOINT
    }
}

impl ImageEditRequest {
    pub fn new(image: &str, prompt: &str) -> Self {
        ImageEditRequest {
            image: image.to_owned(),
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

    pub fn mask(mut self, mask: String) -> Self {
        self.mask = Some(mask);
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
pub struct ImageVariationRequest {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

const IMAGE_VARIATION_ENDPOINT: &str = "https://api.openai.com/v1/images/variations";

impl OpenAIRequest for ImageVariationRequest {
    fn endpoint(&self) -> &str {
        IMAGE_VARIATION_ENDPOINT
    }
}

impl ImageVariationRequest {
    pub fn new(image: &str) -> Self {
        ImageVariationRequest {
            image: image.to_owned(),
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
