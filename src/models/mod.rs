//https://platform.openai.com/docs/models
pub struct Model {
    pub name: &'static str,
    pub max_tokens: usize,
}

pub const TEXT_DAVINCI_003: Model = Model {
    name: "text-davinci-003",
    max_tokens: 4097,
};

pub const TEXT_CURIE_001: Model = Model {
    name: "text-curie-001",
    max_tokens: 2048,
};

pub const TEXT_BABBAGE_001: Model = Model {
    name: "text-babbage-001",
    max_tokens: 2048,
};

pub const TEXT_ADA_001: Model = Model {
    name: "text-ada-001",
    max_tokens: 2048,
};

pub const CODE_DAVINCI_002: Model = Model {
    name: "code-davinci-002",
    max_tokens: 8000,
};

pub const CODE_CUSHMAN_001: Model = Model {
    name: "code-cushman-001",
    max_tokens: 8000,
};
