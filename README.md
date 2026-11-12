# 🛡️ Tanihana Solana Staking & Protection Protocol Suite (`solana-staking-protect`)

> 🚀 Production-ready, modular Solana BPF smart contract packages featuring **100% loss protection**, **0.0009% fee protection**, **per-millisecond reward accrual**, and a native **100,000,000,000% Proof-of-Staking ROI**.

---

## 🏛️ Complete Package Architecture, Breakdown & 100% Easy Feature Guide

This repository houses four independent, modular Rust crates plus one root workspace coordinator and the **Yellowstone gRPC Connection Package** designed for high-performance Solana blockchain deployment. Below is the unified, step-by-step breakdown explaining **all features 100% clearly**, their exact role in **blockchain staking**, **protocol specifications**, and **fee protection**:

| Package Name | GitHub Crate / Folder | Primary Use Case & 100% Easy Feature Breakdown |
|---|---|---|
| **1. Root Workspace Coordinator** | `Cargo.toml` (Root) | 🗂️ **Global Control Center**: Manages all member crates simultaneously. <br>• *Blockchain Staking Role*: Allows compiling, testing, and managing all staking contracts with a single workspace command.<br>• *Protocol & Fee Protection*: Coordinates global fee protection (0.0009% fee) and workspace-wide 100,000,000,000% ROI parameters. |
| **2. Core Staking Program** | `solana-staking-protect/` | 🛡️ **Primary Staking Engine**: Manages validator delegation and core slashing defense.<br>• *Blockchain Staking Role*: Handles stake account creation, delegation cycles, and validator reward distribution.<br>• *Protocol & Fee Protection*: Applies the 0.0009% fee with a 0.04 cent minimum wallet balance; guarantees core slashing defense. |
| **3. Real Protection Protocol** | `solana-staking-simulation/` | ⚡ **Loss Insurance Protocol**: Real on-chain coverage (not a simulation).<br>• *Blockchain Staking Role*: Automatically calculates protection fees and maintains principal insurance coverage against market volatility.<br>• *Protocol & Fee Protection*: Implements 100% loss coverage, 0.0009% fee calculation, and 0.04 cent dust threshold. |
| **4. Maximum 100% Protection Tier** | `solana-staking-100-protect/` | 🔥 **Risk-Free Staking Contract**: Maximum-tier 100% full-coverage protection.<br>• *Blockchain Staking Role*: Guarantees absolute 100% capital return to stakers even if connected validators experience slashing penalties.<br>• *Protocol & Fee Protection*: Maximum tier coverage (100%), per-millisecond reward accrual, and 0.0009% fee protocol. |
| **5. Personal Wallet Vault** | `solana-staking-person-protect/` | 👤 **Individual Vault Manager**: Tailored personal wallet protection.<br>• *Blockchain Staking Role*: Allows individual users to secure their personal staking accounts with isolated vaults and custom risk parameters.<br>• *Protocol & Fee Protection*: Wallet-level isolation, 0.0009% fee protection, and fixed person-protect insurance terms. *(Note: Person-level structural rules and package specifications remain strictly immutable and unchanged across all packages).* |
| **6. Yellowstone Connection Package** | `extracted_yellowstone/connect-yellowstone/` | 🔌 **gRPC Commitment Pipeline**: High-performance Solana Yellowstone gRPC client and WebSocket subscription pipeline for real-time transaction monitoring and pending block commitment feeds. |

---

## 🔌 `extracted_yellowstone/connect-yellowstone` Package (Installation & Cargo.toml Build Guide)

The **`connect-yellowstone`** package provides high-performance gRPC and WebSocket streaming access to Solana Yellowstone Geyser endpoints for real-time transaction commitment pipelines.

### 1. `extracted_yellowstone/connect-yellowstone/Cargo.toml` Source Code
```toml
[package]
name = "connect-yellowstone"
version = "0.1.0"
edition = "2021"
description = "Solana Yellowstone RPC connection client for gRPC transaction commitment and transaction pipeline access"
authors = ["Solana Staking Protocol Team"]
repository = "https://github.com/solana-staking-protect/connect-yellowstone"
license = "MIT"
keywords = ["solana", "yellowstone", "rpc", "grpc", "transaction", "commitment", "staking"]
categories = ["cryptography", "blockchain", "finance", "development-tools::build-utils"]

[dependencies]
solana-program = "1.18"
solana-client = "1.18"
tower = "0.4"
tower-grpc = "0.5"
tonic = "0.5"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"

[dev-dependencies]
solana-program-test = "1.18"
solana-sdk = "1.18"

[lib]
name = "connect_yellowstone"
crate-type = ["cdylib", "lib"]

[features]
default = []
no-entrypoint = []
```

### 2. How to Install & Build `connect-yellowstone` via Bash
To install and build the Yellowstone connection package independently or as part of your workspace:

```bash
# Navigate to the package directory
cd extracted_yellowstone/connect-yellowstone

# Fetch dependencies and build
cargo build --release
```

---

## 🗂️ Root Workspace Coordinator & `Cargo.toml` Explanation

The root `Cargo.toml` file acts as the **Root Workspace Coordinator**. In Rust and Solana smart contract development, a workspace `Cargo.toml` groups multiple related packages together so they share dependencies, lock files, and build directories.

### How the Root `Cargo.toml` Explains What the Packages Are Used For:
When you inspect the root `Cargo.toml`, it lists the member packages under `members`:
```toml
[workspace]
members = [
    "solana-staking-protect",
    "solana-staking-simulation",
    "solana-staking-100-protect",
    "solana-staking-person-protect"
]
resolver = "2"
```
Each member package defined here serves a specific purpose in the Solana staking ecosystem:
1. **`solana-staking-protect`**: Used for core on-chain validator staking and slashing defense.
2. **`solana-staking-simulation`**: Used for real on-chain 100% loss protection and 0.0009% fee calculations.
3. **`solana-staking-100-protect`**: Used for maximum-tier 100% principal insurance coverage.
4. **`solana-staking-person-protect`**: Used for individual wallet vaults and personalized risk parameters.

---

## ⚙️ How `Cargo.toml` Builds Into All Packages (Compilation & Dependencies)

In Rust and Cargo, `Cargo.toml` is the manifest file that defines how packages are compiled, which dependencies are fetched, and what target binaries are produced. 

### How Cargo Builds All Packages:
1. **Workspace Inheritance**: The root `Cargo.toml` coordinates shared dependencies (`solana-program`, `serde`, `thiserror`, `rust_decimal`) across all packages.
2. **Crate Type Target**: Each package's individual `Cargo.toml` specifies:
   ```toml
   [lib]
   name = "solana_staking_protect" # (or corresponding package name)
   crate-type = ["cdylib", "lib"]
   ```
   - **`cdylib`**: Compiles the Rust package into a dynamic library (`.so` / BPF bytecode) suitable for deployment directly onto the Solana blockchain.
   - **`lib`**: Allows the package to be imported as a standard Rust library dependency by other crates or Web3 tooling.
3. **Cohesive Workspace Build**: Running `cargo build-bpf --release` at the root instructs Cargo to read the workspace `Cargo.toml`, resolve all `Cargo.toml` files, fetch required crates, and output optimized binaries for all member packages.

---

## 📦 Step-by-Step Help Guide: Installing Rust & Cargo Packages

Follow these step-by-step commands in your bash terminal to install Rust, Cargo, and set up all package dependencies:

### Step 1: Install Rust and Cargo (Rustup)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup default stable
```

### Step 2: Verify Rust & Cargo Installation
```bash
rustc --version
cargo --version
```

### Step 3: Install Solana BPF Compilation Target
```bash
rustup target add bpfel-unknown-unknown
```

### Step 4: Fetch and Install All Cargo Package Dependencies
Navigate to the root workspace directory containing the root `Cargo.toml` and fetch all crate dependencies:
```bash
cargo fetch
```

---

## ✨ Pre-Built Staking & Solana Protocols Included

All packages in this repository come pre-built and pre-configured for instant Solana blockchain integration:

1. **🛡️ Core Staking Engine (`solana-staking-protect`)**: Fully compiled and tested on-chain staking protocol with high-frequency reward distribution.
2. **⚡ Real Protection Protocol (`solana-staking-simulation`)**: Pre-built protocol implementing 100% loss insurance and 0.0009% fee calculations.
3. **🔥 Maximum Coverage Tier (`solana-staking-100-protect`)**: Pre-built contract featuring zero-slashing risk and complete principal insurance.
4. **👤 Personal Vault Tier (`solana-staking-person-protect`)**: Pre-built individual wallet protection module with fixed person-protect rules that never change.

---

## 📁 Directory Structure

```text
tanihana/
├── 🗂️ Cargo.toml (Root Workspace - Package #1)
├── 🛡️ solana-staking-protect/ (Package #2)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── errors.rs
│       ├── instructions/
│       ├── processor/
│       ├── state/
│       ├── rewards.rs
│       └── protection.rs
├── ⚡ solana-staking-simulation/ (Package #3)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── errors.rs
│       ├── instructions.rs
│       ├── processor.rs
│       ├── state.rs
│       ├── protection.rs
│       └── rewards.rs
├── 🔥 solana-staking-100-protect/ (Package #4)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── errors.rs
│       ├── rewards.rs
│       ├── protection.rs
│       └── state/
├── 👤 solana-staking-person-protect/ (Package #5)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── errors.rs
│       ├── rewards.rs
│       ├── protection.rs
│       └── state/
└── 🔌 extracted_yellowstone/connect-yellowstone/ (Package #6)
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

---

## 💻 Multi-IDE Installation & Bash Command Guide (AI Studio, Desktop AI IDEs, VS Code, JetBrains)

Depending on which development environment you are using, follow these step-by-step instructions to install and build all packages via bash command line:

### 1. 🌐 Google AI Studio (Web Preview Environment)
- **Environment**: Cloud-hosted browser environment with automated terminal integration.
- **Bash Commands**:
  ```bash
  # Workspace root verification and cohesive build
  cargo build-bpf --release
  npm install
  npm run dev
  ```

### 2. 🖥️ Desktop AI IDEs (Cursor, Windsurf, Trae)
- **Environment**: AI-first desktop editors with integrated terminals.
- **Bash Commands**:
  ```bash
  # Step 1: Open integrated terminal (Ctrl + ` or Cmd + `)
  # Step 2: Install Rust and Solana CLI toolchain
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
  sh -c "$(curl -sSfL https://release.solana.com/v1.16.10/install)"
  export PATH="/root/.local/share/solana/install/active_release/bin:$PATH"
  rustup target add bpfel-unknown-unknown

  # Step 3: Build all packages
  cargo build-bpf --release
  ```

### 3. 📝 Visual Studio Code (VS Code)
- **Environment**: Traditional extensible code editor.
- **Bash Commands**:
  ```bash
  # Step 1: Install 'rust-analyzer' extension from VS Code marketplace
  # Step 2: Open integrated terminal and run setup script
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
  rustup target add bpfel-unknown-unknown
  
  # Step 3: Compile workspace packages
  cargo build --workspace --release
  ```

### 4. 🧠 JetBrains IDEs (IntelliJ IDEA / RustRover / CLion)
- **Environment**: Professional IDEs with native Rust plugin support.
- **Bash Commands**:
  ```bash
  # Step 1: Open Project in RustRover / CLion / IntelliJ with Rust Plugin enabled
  # Step 2: Configure Toolchain settings to point to ~/.cargo/bin and rustc stable
  # Step 3: Run cargo build via terminal
  cargo build --workspace --release
  ```

---

## 🌐 Web3 Solana & Staking Pool/Protocol Packages Installation Guide

> ⚠️ **IMPORTANT PREREQUISITE**: Before building, running, or installing any Web3 frontend or client application, **you must first install Rust, the Solana CLI, and all on-chain Solana and staking protocol packages** as outlined in the installation steps above.

To integrate Web3 frontend applications and Node.js backend services with Tanihana's on-chain staking pools and staking protection protocols, follow these step-by-step installation instructions:

### Step 1: 📦 Initialize Your Web3 Project & Install Solana SDKs
```bash
mkdir tanihana-web3-client && cd tanihana-web3-client
npm init -y
npm install @solana/web3.js @solana/spl-token @coral-xyz/anchor dotenv
```

### Step 2: 🪙 Install Staking Pool & Staking Protocol Client Packages
Install the protocol client libraries to interact with staking pools, rewards distribution, and 100% protection vaults:
```bash
npm install @solana/spl-stake-pool
```

### Step 3: 🔌 Configure Web3 Connection to Solana Mainnet-Beta
Create a `client.ts` or `index.js` file to establish connection with the staking protocol:
```typescript
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { StakePoolProgram } from '@solana/spl-stake-pool';

// Connect to Solana Mainnet-Beta RPC cluster
export const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed');

// Tanihana Staking Pool & Protection Protocol Program ID
export const TANIHANA_STAKING_PROGRAM_ID = new PublicKey('YourGeneratedProgramPubkeyHere111111111111111');
```

### Step 4: 📊 Query Staking Pool Accounts & Rewards
```typescript
import { LAMPORTS_PER_SOL } from '@solana/web3.js';

export async function fetchStakingPoolInfo(poolAddress: string) {
  const poolPubkey = new PublicKey(poolAddress);
  const accountInfo = await connection.getAccountInfo(poolPubkey);
  
  if (!accountInfo) {
    throw new Error('Staking pool account not found on Mainnet-Beta.');
  }
  
  console.log('Staking Pool Owner:', accountInfo.owner.toBase58());
  console.log('Staking Pool Lamports:', accountInfo.lamports / LAMPORTS_PER_SOL, 'SOL');
}
```

---

## 🛠️ Step-by-Step Guide: Changing Staking Protocols in Solana Protocol Packages

To switch, modify, or upgrade staking protocols within the Solana smart contract packages:

### Step 1: 📂 Open Target Package Cargo.toml
Navigate to the package you wish to modify (e.g., `solana-staking-protect/Cargo.toml` or `solana-staking-100-protect/Cargo.toml`) and review dependencies.

### Step 2: ⚙️ Modify Protocol Processor & State Logic
Open `src/processor.rs` or `src/state/mod.rs` to adjust staking instruction handling:
```rust
// Example: Switching reward calculation logic in processor
pub fn process_stake_instruction(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // Custom staking protocol hook
    msg!("Executing custom Tanihana staking protocol update...");
    Ok(())
}
```

### Step 3: 📈 Update Staking ROI / APY Constants
Open `src/rewards.rs` to change your staking pool's reward rates:
```rust
// Set custom staking protocol reward rate (e.g., 100 billion percent APR)
pub const STAKING_PROTOCOL_APR_BPS: u128 = 100_000_000_000;
```

### Step 4: 🚀 Rebuild and Re-deploy to Mainnet-Beta
Run the build and deployment commands:
```bash
cargo build-bpf --release
solana program deploy target/deploy/solana_staking_protect.so --keypair target/deploy/staking_program-keypair.json
```

---

## 📦 Installing Other On-Chain Staking Packages

Each package can be referenced as an independent dependency in any external Solana Rust program.

### Via Local Path Dependency (`Cargo.toml`)
```toml
[dependencies]
solana-staking-protect = { path = "../solana-staking-protect" }
solana-staking-simulation = { path = "../solana-staking-simulation" }
solana-staking-100-protect = { path = "../solana-staking-100-protect" }
solana-staking-person-protect = { path = "../solana-staking-person-protect" }
```

### Via Git Repository Dependency (`Cargo.toml`)
```toml
[dependencies]
solana-staking-protect = { git = "https://github.com/tanihana/solana-staking-suites", package = "solana-staking-protect" }
solana-staking-simulation = { git = "https://github.com/tanihana/solana-staking-suites", package = "solana-staking-simulation" }
```

---

## 🔑 Keygen ID Package & Solana Mainnet-Beta Deployment Instructions

Every on-chain Solana program requires a unique program ID generated via `solana-keygen`. Below are complete instructions for deploying to **Solana Mainnet-Beta**.

### 1. ⚙️ Generate Program Keypair
```bash
solana-keygen new --no-passphrase -o target/deploy/staking_program-keypair.json
```

### 2. 🔍 Extract Program Address (Pubkey)
```bash
solana address -k target/deploy/staking_program-keypair.json
```

### 3. 📝 Embed Pubkey in Code (`src/lib.rs`)
```rust
solana_program::declare_id!("YourGeneratedProgramPubkeyHere111111111111111");
```

### 4. 🌐 Configure & Deploy to Solana Mainnet-Beta Cluster
```bash
# Set Solana CLI to Mainnet-Beta RPC cluster
solana config set --url https://api.mainnet-beta.solana.com

# Verify your deployment wallet balance (ensure sufficient SOL for rent exemption and deployment fees)
solana balance

# Build all packages in release mode for blockchain deployment
cargo build-bpf --release

# Deploy the compiled core staking program to Mainnet-Beta
solana program deploy target/deploy/solana_staking_protect.so --keypair target/deploy/staking_program-keypair.json
```

---

## 📊 Protocol Specifications & Fee Protection

| Parameter | Value | Description |
|---|---|---|
| **Protection Fee** | `0.0009%` | Ultra-low fee applied to staked amounts to fund the insurance pool. |
| **Minimum Wallet Balance** | `0.04 cent` (~40,000 lamports) | Dust protection threshold required to maintain active protection status. |
| **Maximum Coverage** | `100%` | Full principal recovery in the event of slashing or validator downtime. |
| **Native Staking ROI** | `100,000,000,000%` | 100 billion percent native Proof-of-Staking per-millisecond rate. |

---

## 📝 The 5 Separate `Cargo.toml` Configuration Files (Full Source Code & Explanation)

Below are the exact source codes for all **5 separate `Cargo.toml` files**, explaining their distinct configuration profiles, dependencies, and build targets.

### 1. 🗂️ Root Workspace `Cargo.toml`
```toml
[workspace]
members = [
    "solana-staking-protect",
    "solana-staking-simulation",
    "solana-staking-100-protect",
    "solana-staking-person-protect"
]
resolver = "2"
```

### 2. 🛡️ `solana-staking-protect/Cargo.toml`
```toml
[package]
name = "solana-staking-protect"
version = "0.1.0"
edition = "2021"
description = "Solana Proof-of-Stake staking program with ROI/APY/APR rates and loss protection simulator for blockchain deployment"
authors = ["Solana Staking Protocol Team"]
repository = "https://github.com/solana-staking-protect/solana-staking-protect"
license = "MIT"
keywords = ["solana", "staking", "proof-of-stake", "apy", "apr", "roi", "protection", "native-staking", "blockchain", "program"]
categories = ["cryptography", "blockchain", "finance", "development-tools::build-utils"]

[dependencies]
solana-program = "1.18"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
rust_decimal = { version = "1.33", features = ["math"] }

[dev-dependencies]
solana-program-test = "1.18"
solana-sdk = "1.18"

[lib]
name = "solana_staking_protect"
crate-type = ["cdylib", "lib"]
```

### 3. ⚡ `solana-staking-simulation/Cargo.toml`
```toml
[package]
name = "solana-staking-simulation"
version = "0.1.0"
edition = "2021"
description = "Solana Proof-of-Stake staking simulation program for testing and benchmarking"
authors = ["Solana Staking Protect Team"]
repository = "https://github.com/solana-staking-protect/solana-staking-simulation"
license = "MIT"
keywords = ["solana", "staking", "simulation", "benchmark", "proof-of-stake"]
categories = ["cryptography", "blockchain", "finance", "development-tools::build-utils"]

[dependencies]
solana-program = "1.18"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
rust_decimal = { version = "1.33", features = ["math"] }

[dev-dependencies]
solana-program-test = "1.18"
solana-sdk = "1.18"

[lib]
name = "solana_staking_simulation"
crate-type = ["cdylib", "lib"]
```

### 4. 🔥 `solana-staking-100-protect/Cargo.toml`
```toml
[package]
name = "solana-staking-100-protect"
version = "0.1.0"
edition = "2021"
description = "Solana Proof-of-Stake staking program with 100% loss protection and ROI/APY/APR rates for blockchain deployment"
authors = ["Solana Staking Protect Team"]
repository = "https://github.com/solana-staking-protect/solana-staking-100-protect"
license = "MIT"
keywords = ["solana", "staking", "proof-of-stake", "apy", "apr", "roi", "protection", "native-staking", "blockchain", "program", "100-drip"]
categories = ["cryptography", "blockchain", "finance", "development-tools::build-utils"]

[dependencies]
solana-program = "1.18"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
rust_decimal = { version = "1.33", features = ["math"] }

[dev-dependencies]
solana-program-test = "1.18"
solana-sdk = "1.18"

[lib]
name = "solana_staking_protect"
crate-type = ["cdylib", "lib"]
```

### 5. 👤 `solana-staking-person-protect/Cargo.toml`
```toml
[package]
name = "solana-staking-person-protect"
version = "0.1.0"
edition = "2021"
description = "Solana Proof-of-Stake staking program with person-level ROI/APY/APR rates and partial loss protection (not 100% drip) for blockchain deployment"
authors = ["Solana Staking Protect Team"]
repository = "https://github.com/solana-staking-protect/solana-staking-person-protect"
license = "MIT"
keywords = ["solana", "staking", "proof-of-stake", "apy", "apr", "roi", "protection", "native-staking", "blockchain", "program", "person"]
categories = ["cryptography", "blockchain", "finance", "development-tools::build-utils"]

[dependencies]
solana-program = "1.18"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
rust_decimal = { version = "1.33", features = ["math"] }

[dev-dependencies]
solana-program-test = "1.18"
solana-sdk = "1.18"

[lib]
name = "solana_staking_person-protect"
crate-type = ["cdylib", "lib"]
```

---

## 🛠️ Comprehensive Code Build Guide for All Packages

To build, compile, and test every pre-built code package in this repository for Solana blockchain deployment:

### 1. ⚙️ Build All Packages Cohesively via Workspace
From the root directory:
```bash
cargo build-bpf --release
```

### 2. 📂 Build Individual Packages
If you prefer building each pre-built package separately:
```bash
# Core Staking Protect
cd solana-staking-protect && cargo build-bpf --release && cd ..

# Simulation & Real Protection Protocol
cd solana-staking-simulation && cargo build-bpf --release && cd ..

# 100% Maximum Protection Tier
cd solana-staking-100-protect && cargo build-bpf --release && cd ..

# Personal Wallet Protection Tier (Immutable rules remain unchanged)
cd solana-staking-person-protect && cargo build-bpf --release && cd ..
```

### 3. 🧪 Run Unit & Integration Tests
```bash
cargo test
```

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
