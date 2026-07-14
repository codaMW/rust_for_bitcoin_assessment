mod config;
mod error;
mod rpc;
mod cli;
mod commands;

use clap::Parser;
use config::Config;
use rpc::BitcoinRpcClient;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    // 1. Process local system configuration
    let mut config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration Error: {}", e);
            std::process::exit(1);
        }
    };

    // 2. Parse arguments passed from Terminal
    let args = Cli::parse();

    // Support override URL from command line flags
    if let Some(custom_url) = args.url {
        config.rpc_url = custom_url;
    }

    // Support connecting to specific sub-wallets 
    // Bitcoin Core isolates multi-wallet RPC interfaces at "http://node-ip:port/wallet/<wallet-name>"
    if let Some(wallet_name) = args.wallet {
        config.rpc_url = format!("{}/wallet/{}", config.rpc_url.trim_end_matches('/'), wallet_name);
    }

    // 3. Initialize RPC Engine
    let rpc_client = match BitcoinRpcClient::new(&config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Client Initialization Error: {}", e);
            std::process::exit(1);
        }
    };

    // 4. Command Router Execution
    let exec_result = match args.command {
        Commands::BlockchainInfo => commands::blockchain::handle_info(&rpc_client).await,
        Commands::WalletInfo => commands::wallet::handle_wallet_info(&rpc_client).await,
        Commands::Balance => commands::wallet::handle_balance(&rpc_client).await,
        Commands::NewAddress => commands::wallet::handle_new_address(&rpc_client).await,
        Commands::Rpc { method, params } => {
            commands::wallet::handle_generic_rpc(&rpc_client, method, params).await
        }
    };

    // 5. Graceful Error printer
    if let Err(e) = exec_result {
        eprintln!("Operational Error: {}", e);
        std::process::exit(1);
    }
}
