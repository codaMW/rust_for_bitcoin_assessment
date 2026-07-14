// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Environment configuration error: {0}")]
    Config(String),

    #[error("Network connection failed: {0}")]
    Network(#[from] reqwest::Error),

    #[error("RPC error returned from node: (Code {code}) {message}")]
    BitcoinRpc { code: i32, message: String },

    #[error("Serialization / Deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),

}
