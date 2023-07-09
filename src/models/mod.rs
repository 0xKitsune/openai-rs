//https://platform.openai.com/docs/models
pub struct Model {
    pub name: &'static str,
    pub max_tokens: usize,
}

pub const GPT_4: Model = Model {
    name: "gpt-4",
    max_tokens: 8192,
};

pub const GPT_4_32K: Model = Model {
    name: "gpt-4-32k",
    max_tokens: 32768,
};

pub const GPT_3_5_TURBO: Model = Model {
    name: "gpt-3.5-turbo",
    max_tokens: 4096,
};

pub const GPT_3_5_TURBO_16K: Model = Model {
    name: "gpt-3.5-turbo-16k",
    max_tokens: 16384,
};
