use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    pub suffix: Option<String>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<i32>,
    pub stream: Option<bool>,
    pub lob_probs: Option<i32>,
    pub echo: Option<bool>,
    pub stop: Option<bool>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub best_of: Option<i32>,
    pub logit_bias: Option<String>,
    pub user: Option<String>,
}

impl CompletionRequest {
    pub fn new(model: String, prompt: String) -> Self {
        CompletionRequest {
            model,
            prompt,
            ..Default::default()
        }
    }

    pub fn suffix(&mut self, suffix: String) {
        self.suffix = Some(suffix);
    }

    pub fn max_tokens(&mut self, max_tokens: i32) {
        self.max_tokens = Some(max_tokens);
    }
    pub fn temperature(&mut self, temperature: f64) {
        self.temperature = Some(temperature);
    }
    pub fn top_p(&mut self, top_p: f64) {
        self.top_p = Some(top_p);
    }
    pub fn n(&mut self, n: i32) {
        self.n = Some(n);
    }
    pub fn stream(&mut self, stream: bool) {
        self.stream = Some(stream);
    }

    pub fn lob_probs(&mut self, lob_probs: i32) {
        self.lob_probs = Some(lob_probs);
    }

    pub fn echo(&mut self, echo: bool) {
        self.echo = Some(echo);
    }
    pub fn stop(&mut self, stop: bool) {
        self.stop = Some(stop);
    }

    pub fn presence_penalty(&mut self, presence_penalty: f64) {
        self.presence_penalty = Some(presence_penalty);
    }
    pub fn frequency_penalty(&mut self, frequency_penalty: f64) {
        self.frequency_penalty = Some(frequency_penalty);
    }

    pub fn best_of(&mut self, best_of: i32) {
        self.best_of = Some(best_of);
    }
    pub fn logit_bias(&mut self, logit_bias: String) {
        self.logit_bias = Some(logit_bias);
    }

    pub fn user(&mut self, user: String) {
        self.user = Some(user);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Choice {
    text: String,
    index: usize,
    log_probs: Option<i32>,
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}
