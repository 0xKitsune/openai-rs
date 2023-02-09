use serde::{Deserialize, Serialize};

use crate::client::{OpenAIRequest, OpenAIResponse};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EditRequest {
    model: String,
    instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,
}

const EDIT_ENDPOINT: &str = "https://api.openai.com/v1/edits";

impl OpenAIRequest for EditRequest {
    fn endpoint(&self) -> &str {
        EDIT_ENDPOINT
    }
}

impl EditRequest {
    pub fn new(model: &str, instruction: &str) -> Self {
        EditRequest {
            model: model.to_owned(),
            instruction: instruction.to_owned(),
            ..Default::default()
        }
    }

    pub fn input(mut self, input: &str) -> Self {
        self.input = Some(input.to_owned());
        self
    }

    pub fn n(mut self, n: i32) -> Self {
        self.n = Some(n.to_owned());
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature.to_owned());
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p.to_owned());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EditResponse {
    pub object: String,
    pub created: usize,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

impl OpenAIResponse for EditResponse {}

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
