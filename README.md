# Bitcoin Core RPC CLI Client

A robust, production-grade command-line interface (CLI) tool built in Rust to interact with a local Bitcoin Core Regtest node via JSON-RPC.

This application is designed with **strong cryptographic typing**, defensive error handling (no panics), modular architecture, and dynamic argument parsing for raw RPC relaying.

---

## Architecture & Design Decisions

To meet the high standard of production Bitcoin software, the application is broken down into structured, isolated domains:

* **`src/main.rs`**: The runtime orchestrator that configures environment inputs and routes CLI execution.
* **`src/config.rs`**: Safe environment variable parsing and configuration injection.
* **`src/error.rs`**: A centralized error domain built using `thiserror`. It gracefully captures network timeouts, authentication failures, serialization errors, and node RPC issues without ever crashing the application.
* **`src/rpc.rs`**: A custom `reqwest`-based JSON-RPC 2.0 client implementing Basic Auth (Base64 credential encoding) to securely query the node.
* **`src/cli.rs`**: Built using `clap` (v4 with the `derive` feature) for type-safe terminal routing.
* **`src/commands/`**: Contains the business logic split cleanly between `blockchain` metrics and `wallet` operations.

---

## Setup Instructions

### 1. Run a Local Regtest Node with Polar
1. Download and run [Polar](https://lightningpolar.com/).
2. Create a network containing at least one Bitcoin Core node (e.g., `alice`).
3. Start the network.
4. From the Polar dashboard under **Connect**, locate your node's:
    * **RPC URL** (e.g., `http://127.0.0.1:18443`)
    * **RPC Username** (e.g., `polaruser`)
    * **RPC Password** (e.g., `polarpass`)

* **Note: You need to download docker desktop

### 2. Configure Your Environment Variables
To ensure no sensitive credentials are hardcoded into the source, configure your shell with the following environment variables:

```bash
export BTC_RPC_URL="[http://127.0.0.1:18443](http://127.0.0.1:18443)" # Check your node's actual port!
export BTC_RPC_USER="polaruser"
export BTC_RPC_PASS="polarpass"
