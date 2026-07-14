use crate::rpc::BitcoinRpcClient;
use crate::error::AppError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BlockchainInfoResult {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verificationprogress: f64,
}

pub async fn handle_info(client: &BitcoinRpcClient) -> Result<(), AppError> {
    let raw_val = client.call("getblockchaininfo", vec![]).await?;
    let info: BlockchainInfoResult = serde_json::from_value(raw_val)?;

    println!("=========================================");
    println!("             BLOCKCHAIN METRICS          ");
    println!("=========================================");
    println!("Network Chain:        {}", info.chain);
    println!("Validated Blocks:     {}", info.blocks);
    println!("Best Headers:         {}", info.headers);
    println!("Mining Difficulty:    {}", info.difficulty);
    println!("Sync Progress:        {:.4}%", info.verificationprogress * 100.0);
    println!("=========================================");

    Ok(())
}
