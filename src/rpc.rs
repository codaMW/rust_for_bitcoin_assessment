use crate::error::AppError;
use crate::config::Config;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: &'static str,
    id: &'static str,
    method: String,
    params: Vec<Value>,
}

#[derive(Deserialize)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<RpcErrorPayload>,
}

#[derive(Deserialize, Debug)]
struct RpcErrorPayload {
    code: i32,
    message: String,
}

pub struct BitcoinRpcClient {
    client: reqwest::Client,
    url: String,
}

impl BitcoinRpcClient {
    pub fn new(config: &Config) -> Result<Self, AppError> {
        // Build the basic authorization header manually
        let auth_str = format!("{}:{}", config.rpc_user, config.rpc_pass);
        let b64_auth = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, auth_str);
        
        let mut headers = HeaderMap::new();
        let mut auth_header = HeaderValue::from_str(&format!("Basic {}", b64_auth))
            .map_err(|_| AppError::Config("Invalid character in RPC credentials".into()))?;
        auth_header.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_header);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client, url: config.rpc_url.clone() })
    }

    /// Sends a type-safe or dynamic request to the Bitcoin node
    pub async fn call(&self, method: &str, params: Vec<Value>) -> Result<Value, AppError> {
        let payload = RpcRequest {
            jsonrpc: "2.0",
            id: "btc-cli",
            method: method.to_string(),
            params,
        };

        let res = self.client.post(&self.url)
            .json(&payload)
            .send()
            .await?;

        // Catch non-200 authentication/network errors before parsing JSON
        if res.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::BitcoinRpc {
                code: -401,
                message: "Unauthorized: Invalid RPC Username or Password".to_string(),
            });
        }

        let rpc_response: RpcResponse = res.json().await?;

        if let Some(err) = rpc_response.error {
            return Err(AppError::BitcoinRpc {
                code: err.code,
                message: err.message,
            });
        }

        Ok(rpc_response.result.unwrap_or(Value::Null))
    }
}
