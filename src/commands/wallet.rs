use crate::rpc::BitcoinRpcClient;
use crate::error::AppError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WalletInfoResult {
    walletname: String,
    // Using Option keeps the parser from failing if the node hides these fields
    balance: Option<f64>,
    unconfirmed_balance: Option<f64>,
    txcount: Option<u64>,
}

pub async fn handle_wallet_info(client: &BitcoinRpcClient) -> Result<(), AppError> {
    let raw_val = client.call("getwalletinfo", vec![]).await?;
    let info: WalletInfoResult = serde_json::from_value(raw_val)?;

    println!("=========================================");
    println!("             WALLET INFORMATION          ");
    println!("=========================================");
    println!("Wallet Name:          {}", info.walletname);
    println!("Confirmed Balance:    {} BTC", info.balance.unwrap_or(0.0));
    println!("Unconfirmed Balance:  {} BTC", info.unconfirmed_balance.unwrap_or(0.0));
    println!("Transaction Count:    {}", info.txcount.unwrap_or(0));
    println!("=========================================");

    Ok(())
}

pub async fn handle_balance(client: &BitcoinRpcClient) -> Result<(), AppError> {
    let raw_val = client.call("getbalance", vec![]).await?;
    let balance: f64 = serde_json::from_value(raw_val)?;
    println!("Wallet Balance: {} BTC", balance);
    Ok(())
}

pub async fn handle_new_address(client: &BitcoinRpcClient) -> Result<(), AppError> {
    let raw_val = client.call("getnewaddress", vec![]).await?;
    let address: String = serde_json::from_value(raw_val)?;
    println!("Generated Address: {}", address);
    Ok(())
}

pub async fn handle_generic_rpc(
    client: &BitcoinRpcClient,
    method: String,
    raw_params: Vec<String>,
) -> Result<(), AppError> {
    let parsed_params: Vec<serde_json::Value> = raw_params
        .into_iter()
        .map(|p| {
            serde_json::from_str(&p)
                .unwrap_or_else(|_| serde_json::Value::String(p))
        })
        .collect();

    let result = client.call(&method, parsed_params).await?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
