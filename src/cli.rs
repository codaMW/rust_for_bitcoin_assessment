// src/cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "btc-cli")]
#[command(about = "Command-Line Bitcoin node dashboard", long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true, help = "Override RPC Node URL")]
    pub url: Option<String>,

    #[arg(short, long, global = true, help = "Optional Wallet Name")]
    pub wallet: Option<String>,

    #[command(subcommand)] // This specifies that `command` contains the subcommand enums
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)] // Derived Subcommand and Clone here!
pub enum Commands {
    #[command(about = "Get basic blockchain metrics")]
    BlockchainInfo,

    #[command(about = "Get loaded wallet stats")]
    WalletInfo,

    #[command(about = "Display wallet balance in BTC")]
    Balance,

    #[command(about = "Generate a new wallet receiving address")]
    NewAddress,

    #[command(about = "Run arbitrary RPC methods dynamically")]
    Rpc {
        method: String,
        params: Vec<String>,
    },
}
