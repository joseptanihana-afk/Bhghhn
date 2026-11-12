//! Solana Staking Production - 100% Real Protection Protocol on Blockchain
//!
//! This is a real on-chain Solana program that provides 100% loss protection
//! on the Solana blockchain.
//!
//! Fee protection protocol: 0.0009% fee with 0.04 cent minimum wallet balance.
//! Maximum tier provides 100% protection on-chain.

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

pub mod errors;
pub mod instructions;
pub mod processor;
pub mod rewards;
pub mod state;
pub mod protection;

pub use errors::StakingProductionError;
pub use instructions::ProductionInstruction;
pub use processor::process_instruction;

#[cfg(not(feature = "std"))]
extern crate core;

/// Program entry point for on-chain execution
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, instruction_data)
}
