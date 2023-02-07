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

    pub fn max_tokens(mut self, max_tokens: i32) -> Self {
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
    pub fn logit_bias(mut self, logit_bias: String) -> Self {
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
