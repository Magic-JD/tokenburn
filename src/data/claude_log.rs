use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TokenLog {
    pub(crate) message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub(crate) id: String,
    pub(crate) model: String,
    pub(crate) usage: Usage,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Usage {
    pub(crate) input_tokens: isize,
    pub(crate) output_tokens: isize,
    pub(crate) cache_read_input_tokens: isize,
    pub(crate) cache_creation: CacheCreation,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CacheCreation {
    pub(crate) ephemeral_1h_input_tokens: isize,
    pub(crate) ephemeral_5m_input_tokens: isize,
}
