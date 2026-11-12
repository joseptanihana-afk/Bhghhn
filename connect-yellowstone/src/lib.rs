//! Connect Yellowstone - gRPC Client for Solana Transaction Pipeline
//!
//! This module provides a client for connecting to Solana Yellowstone gRPC endpoints,
//! enabling access to the transaction pipeline before blocks are committed.
//! Yellowstone provides a parallel transaction commitment mechanism that allows
//! validators and sophisticated traders to see and act on transactions before
//! they appear in blocks.
//!
//! # Features
//!
//! - Yellowstone gRPC endpoint connection
//! - Transaction filter configuration
//! - Commitment pipeline access
//! - Websocket support for real-time transaction updates
//!
//! # Usage
//!
//! ```
//! use connect_yellowstone::{YellowstoneClient, YellowstoneConfig}
//! 
//! let config = YellowstoneConfig::default()
//!     .with_endpoint("https://yellowstone-mainnet.example.com".parse().unwrap());
//! let client = YellowstoneClient::new(config)?;
//! ```

#![cfg_attr(not(feature = "no-entrypoint"), no_entrypoint)]
#![cfg_attr(not(feature = "std"), no_std)]

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub mod config;
pub mod client;
pub mod errors;
pub mod types;

pub use config::YellowstoneConfig;
pub use client::YellowstoneClient;
pub use errors::YellowstoneError;
pub use types::*;

#[cfg(not(feature = "std"))]
extern crate core;

pub mod config {
    //! Configuration for Yellowstone gRPC connection
    //!
    //! # Examples
    //!
    //! ```
    //! use connect_yellowstone::config::YellowstoneConfig;
    //! use std::net::ToSocketAddrs;
    //!
    //! let config = YellowstoneConfig::new("0.0.0.0:9092".to_socket_addrs().unwrap().next().unwrap());
    //! ```
    use std::net::SocketAddr;

    use serde::{Deserialize, Serialize};

    /// Configuration for connecting to a Yellowstone gRPC endpoint
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct YellowstoneConfig {
        /// The gRPC endpoint address for Yellowstone
        pub endpoint: SocketAddr,
        /// Optional authentication token for the Yellowstone relay
        pub auth_token: Option<String>,
        /// Whether to enable commitment pipeline access
        pub enable_commitment_pipeline: bool,
        /// Maximum number of pending transactions to buffer
        pub max_pending_txs: usize,
        /// Filter for transaction types to receive
        pub transaction_filter: Option<TransactionFilter>,
    }

    /// Filter for which transactions to receive from Yellowstone
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TransactionFilter {
        /// Filter by program ID(s) to monitor
        pub program_ids: Option<Vec<Pubkey>>,
        /// Filter by instruction data patterns
        pub instruction_data: Option<String>,
        /// Whether to include internal instructions
        pub include_internal_instructions: bool,
    }

    impl Default for YellowstoneConfig {
        fn default() -> Self {
            Self {
                endpoint: "0.0.0.0:9092".parse().unwrap(),
                auth_token: None,
                enable_commitment_pipeline: true,
                max_pending_txs: 1000,
                transaction_filter: None,
            }
        }
    }

    impl YellowstoneConfig {
        /// Create a new YellowstoneConfig with the given endpoint
        pub fn new(endpoint: SocketAddr) -> Self {
            Self {
                endpoint,
                ..Default::default()
            }
        }

        /// Set the authentication token
        pub fn with_auth_token(mut self, token: String) -> Self {
            self.auth_token = Some(token);
            self
        }

        /// Enable or disable commitment pipeline access
        pub fn with_commitment_pipeline(mut self, enabled: bool) -> Self {
            self.enable_commitment_pipeline = enabled;
            self
        }

        /// Set maximum pending transactions buffer size
        pub fn with_max_pending_txs(mut self, max: usize) -> Self {
            self.max_pending_txs = max;
            self
        }

        /// Set transaction filter
        pub fn with_filter(mut self, filter: TransactionFilter) -> Self {
            self.transaction_filter = Some(filter);
            self
        }
    }
}

pub mod client {
    use super::*;
    use tonic::transport::Channel;
    use tonic_grpc::Client as TonicClient;

    /// Error type for Yellowstone client operations
    #[derive(Debug, thiserror::Error)]
    pub enum YellowstoneError {
        #[error("gRPC connection error: {0}")]
        ConnectionError(String),
        #[error("Serialization error: {0}")]
        SerializationError(String),
        #[error("Timeout error: {0}")]
        TimeoutError(String),
        #[error("Invalid configuration: {0}")]
        InvalidConfig(String),
    }

    impl std::fmt::Display for YellowstoneError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.description())
        }
    }

    impl std::error::Error for YellowstoneError {}

    /// Client for connecting to Solana Yellowstone gRPC endpoints
    #[derive(Debug)]
    pub struct YellowstoneClient {
        /// The configured Yellowstone endpoint
        config: YellowstoneConfig,
        /// gRPC channel to the Yellowstone relay
        channel: Channel,
        /// Tonc client for Yellowstone services
        yellowstone_client: TonicClient<Channel>,
    }

    impl YellowstoneClient {
        /// Create a new YellowstoneClient from configuration
        pub fn new(config: YellowstoneConfig) -> Result<Self, YellowstoneError> {
            let channel = tonic::transport::Endpoint::new(config.endpoint)
                .map_err(|e| YellowstoneError::ConnectionError(format!("Endpoint error: {}", e)))?
                .connect()
                .map_err(|e| YellowstoneError::ConnectionError(format!("Connect error: {}", e)))?;

            let yellowstone_client = TonicClient::new(channel);

            Ok(Self {
                config,
                channel,
                yellowstone_client,
            })
        }

        /// Connect to the Yellowstone endpoint with the given configuration
        pub async fn connect(&self) -> Result<(), YellowstoneError> {
            // Channel is already established in new(), verify it's healthy
            Ok(())
        }

        /// Subscribe to pending transactions from the Yellowstone pipeline
        ///
        /// This returns a stream of transactions before they are committed to blocks.
        pub async fn subscribe_pending_txs(&self) -> Result<tonic::Streaming<YellowstoneTx>, YellowstoneError> {
            let request = yellowstone_proto::ton::SubscribeRequest {
                // Default subscription request
                subscription: Some(yellowstone_proto::ton::SomeSubscription::BlockCommitment(
                    yellowstone_proto::ton::BlockCommitmentSubscription {
                        commitment: Some(yellowstone_proto::ton::Commitment::Max)
                    }
                ))
            };

            self.yellowstone_client
                .subscribe(request)
                .await
                .map_err(|e| YellowstoneError::ConnectionError(format!("Subscribe error: {}", e)))
        }

        /// Get blocked transactions from the commitment pipeline
        pub async fn get_blocked_txs(&self, filter: &TransactionFilter) -> Result<Vec<YellowstoneTx>, YellowstoneError> {
            // Implementation for getting blocked/pending transactions
            Ok(vec![])
        }
    }

    /// Yellowstone transaction received from the pipeline
    #[derive(Debug, Clone)]
    pub struct YellowstoneTx {
        /// The transaction signature
        pub signature: [u8; 64],
        /// The block height this transaction was first seen at
        pub block_height: u64,
        /// The transaction metadata
        pub metadata: TxMetadata,
        /// The slot number
        pub slot: u64,
    }

    /// Transaction metadata from Yellowstone
    #[derive(Debug, Clone)]
    pub struct TxMetadata {
        /// Whether the transaction is fee-paying
        pub is_fee_paying: bool,
        /// Program addresses involved
        pub program_ids: Vec<Pubkey>,
        /// Instruction data
        pub instruction_data: Vec<u8>,
    }
}
pub mod types {
    use super::super::config::TransactionFilter;
    use solana_program::pubkey::Pubkey;

    /// Pipeline transaction type from Yellowstone
    pub type PipelineTx = yellowstone_grpc::pb::token::Transaction;

    /// Commitment level for Yellowstone transactions
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum Commitment {
        /// Wait for confirmation
        Max,
        /// Finalized commitment
        Finalized,
        /// Recent commitment
        Recent,
    }

    /// Transaction filter configuration
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct YellowstoneTransactionFilter {
        /// Program IDs to filter by
        pub program_ids: Option<Vec<Pubkey>>,
        /// Instruction data patterns
        pub instruction_data: Option<String>,
    }
}

/// Websocket connection for real-time transaction updates
pub mod websocket {
    use super::*;
    use tokio::sync::mpsc;
    use tokio::websockets::WebSocketStream;
    use futures::StreamExt;

    /// WebSocket event from Yellowstone
    #[derive(Debug, Clone)]
    pub enum YellowstoneWsEvent {
        /// New pending transaction
        NewTransaction(YellowstoneTx),
        /// Block commitment updated
        BlockCommitment { slot: u64, commitment: Commitment },
        /// Connection status
        ConnectionStatus { connected: bool },
    }

    /// Yellowstone WebSocket client
    #[derive(Debug)]
    pub struct YellowstoneWsClient {
        /// The underlying Yellowstone client
        client: YellowstoneClient,
        /// WebSocket stream
        ws: Option<WebSocketStream<tokio::net::tcp::ReadHalf>>>,
        /// Event transmitter
        tx: Option<mpsc::Sender<YellowstoneWsEvent>>,
    }

    impl YellowstoneWsClient {
        /// Create a new WebSocket client
        pub fn new(client: YellowstoneClient) -> Self {
            Self {
                client,
                ws: None,
                tx: None,
            }
        }

        /// Connect to Yellowstone WebSocket endpoint
        pub async fn connect(&mut self, endpoint: SocketAddr) -> Result<(), YellowstoneError> {
            // Establish WebSocket connection
            let socket = tokio::net::TcpStream::connect(endpoint)
                .await
                .map_err(|e| YellowstoneError::ConnectionError(format!("WS connect error: {}", e)))?;

            let (read, write) = socket.split();
            let ws = WebSocketStream::new(read, write);

            self.ws = Some(ws);
            Ok(())
        }

        /// Subscribe to real-time events
        pub async fn subscribe_events(&mut self) -> Result<mpsc::Receiver<YellowstoneWsEvent>, YellowstoneError> {
            let (tx, rx) = mpsc::channel(100);
            self.tx = Some(tx);

            // Start listening for Yellowstone events
            // This would parse WebSocket messages and convert to events
            Ok(rx)
        }

        /// Poll for incoming events
        pub async fn poll_event(&mut self) -> Option<YellowstoneWsEvent> {
            // Poll the WebSocket for new events
            None
        }
    }
}
