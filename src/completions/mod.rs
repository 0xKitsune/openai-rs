use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{OpenAIRequest, OpenAIResponse};

const CHAT_COMPLETION_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Default, Debug, Serialize, Clone, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Function>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lob_probs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<String>,
}

impl Message {
    pub fn new(content: &str) -> Message {
        Message {
            content: content.to_owned(),
            ..Default::default()
        }
    }

    pub fn role(mut self, role: Role) -> Message {
        self.role = role;
        self
    }

    pub fn name(mut self, name: String) -> Message {
        self.name = Some(name);
        self
    }

    pub fn function_call(mut self, function_call: String) -> Message {
        self.function_call = Some(function_call);
        self
    }
}

#[derive(Default, Debug, Serialize, Clone, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    #[default]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "function")]
    Function,
}

impl OpenAIRequest for ChatCompletionRequest {
    fn endpoint(&self) -> &str {
        CHAT_COMPLETION_ENDPOINT
    }
}

impl ChatCompletionRequest {
    pub fn new(model: &str, messages: Vec<Message>) -> Self {
        ChatCompletionRequest {
            model: model.to_owned(),
            messages,
            ..Default::default()
        }
    }

    pub fn max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }
    pub fn n(mut self, n: i32) -> Self {
        self.n = Some(n);
        self
    }
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn lob_probs(mut self, lob_probs: i32) -> Self {
        self.lob_probs = Some(lob_probs);
        self
    }

    pub fn stop(mut self, stop: bool) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }
    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn logit_bias(mut self, logit_bias: HashMap<String, i32>) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: usize,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

impl OpenAIResponse for ChatCompletionResponse {}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Choice {
    pub index: usize,
    pub message: Message,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}
