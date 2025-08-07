# Kosher Chain

Welcome to the Kosher Chain, a Layer 2 blockchain built on the XRP Ledger, designed to operate under the principles of Ashkenazi Jewish law. This project provides the node software required to run a validator and participate in the network.

## Project Overview

The Kosher Chain is a Proof-of-Authority (PoA) sidechain. A council of trusted and vetted validators, approved under the project's governance model, are responsible for creating new blocks and ensuring the integrity of the network.

**Key Components:**
* **Node Application:** The core Rust application that runs the blockchain.
* **P2P Networking:** Uses `libp2p` to synchronize the ledger between nodes.
* **HTTP API:** An `axum`-based API for submitting transactions.
* **XRPL Witness:** A service that monitors the XRP Ledger for deposits to the L2 chain.

---

## Getting Started

### Prerequisites
* [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
* A text editor (e.g., VSCode)

### 1. Configuration

Before running a node, you must set up your configuration files.

1.  **Main Configuration (`config.toml`)**:
    * Copy the provided `config.toml.example` to `config.toml`.
    * Set the `door_account` under `[witness]` to the public address of the federation's multisignature account on the XRPL.
    * Review and adjust the API and P2P listen addresses as needed.

2.  **Validator Set (`validators.json`)**:
    * This file contains the list of public keys for all trusted validators.
    * Ensure this file is present and correctly formatted with the public keys of the current Rabbinic Council-approved validators.

### 2. Running the Node

Once configured, you can run the node from the project's root directory:

```sh
cargo run --release
```

The `--release` flag is crucial for running with optimizations, which is necessary for a production environment. Upon starting, the node will initialize all services:
* The API server will start listening for transactions.
* The P2P service will begin discovering and connecting to peers.
* The XRPL Witness service will start monitoring the door account for deposits.

---

## Governance

The Kosher Chain is governed by a council of Halachic authorities. All changes to the validator set and network rules are subject to this governance process. For more details, see `GOVERNANCE.md`.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.
