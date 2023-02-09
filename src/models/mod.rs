//https://platform.openai.com/docs/models
pub struct CompletionModel {
    pub name: &'static str,
    pub max_tokens: usize,
}

pub const TEXT_DAVINCI_003: CompletionModel = CompletionModel {
    name: "text-davinci-003",
    max_tokens: 4097,
};

pub const TEXT_CURIE_001: CompletionModel = CompletionModel {
    name: "text-curie-001",
    max_tokens: 2048,
};

pub const TEXT_BABBAGE_001: CompletionModel = CompletionModel {
    name: "text-babbage-001",
    max_tokens: 2048,
};

pub const TEXT_ADA_001: CompletionModel = CompletionModel {
    name: "text-ada-001",
    max_tokens: 2048,
};

pub const CODE_DAVINCI_002: CompletionModel = CompletionModel {
    name: "code-davinci-002",
    max_tokens: 8000,
};

pub const CODE_CUSHMAN_001: CompletionModel = CompletionModel {
    name: "code-cushman-001",
    max_tokens: 8000,
};

pub struct EditModel {
    pub name: &'static str,
}

pub const TEXT_DAVINCI_EDIT_001: EditModel = EditModel {
    name: "text-davinci-edit-001",
};

pub const CODE_DAVINCI_EDIT_001: EditModel = EditModel {
    name: "code-davinci-edit-001",
};
