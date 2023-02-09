use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{OpenAIRequest, OpenAIResponse};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
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
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

const COMPLETION_ENDPOINT: &str = "https://api.openai.com/v1/completions";

impl OpenAIRequest for CompletionRequest {
    fn endpoint(&self) -> &str {
        COMPLETION_ENDPOINT
    }
}

impl CompletionRequest {
    pub fn new(model: &str, prompt: &str) -> Self {
        CompletionRequest {
            model: model.to_owned(),
            prompt: prompt.to_owned(),
            ..Default::default()
        }
    }

    pub fn suffix(mut self, suffix: String) -> Self {
        self.suffix = Some(suffix);
        self
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

    pub fn echo(mut self, echo: bool) -> Self {
        self.echo = Some(echo);
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

    pub fn best_of(mut self, best_of: i32) -> Self {
        self.best_of = Some(best_of);
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
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: usize,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

impl OpenAIResponse for CompletionResponse {}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Choice {
    pub text: String,
    pub index: usize,
    pub log_probs: Option<i32>,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}
