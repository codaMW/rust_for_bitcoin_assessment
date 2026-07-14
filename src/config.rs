use std::env;
use crate::error::AppError;

pub struct Config {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_pass: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        let rpc_url = env::var("BTC_RPC_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:18443".to_string());
        
        let rpc_user = env::var("BTC_RPC_USER")
            .map_err(|_| AppError::Config("BTC_RPC_USER env var is missing".into()))?;
            
        let rpc_pass = env::var("BTC_RPC_PASS")
            .map_err(|_| AppError::Config("BTC_RPC_PASS env var is missing".into()))?;

        Ok(Self { rpc_url, rpc_user, rpc_pass })
    }
}
