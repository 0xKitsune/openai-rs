//https://platform.openai.com/docs/models
pub struct Model {
    pub name: &'static str,
    pub max_tokens: usize,
}

#[allow(non_upper_case_globals)]
pub const GPT_4o: Model = Model {
    name: "gpt-4o",
    max_tokens: 128000,
};

#[allow(non_upper_case_globals)]
pub const GPT_4o_Mini: Model = Model {
    name: "gpt-4o-mini",
    max_tokens: 128000,
};

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
