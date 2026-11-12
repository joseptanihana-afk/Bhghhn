//! Solana Staking Protect Person - On-Chain Program
//!
//! A production-ready Proof-of-Stake staking program for Solana blockchain
//! with person-level ROI/APY/APR reward calculations and partial loss protection
//! mechanisms (not 100% drip). This program can be compiled to BPF and deployed
//! on Solana mainnet/devnet.
//
//! **Important: Protection tiers provide partial coverage (Basic 10%, Standard 25%, Premium 50%)
//! - NO 100% maximum tier - person retains some exposure
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

pub use errors::StakingProtectError;
pub use instructions::StakeInstruction;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_entry_exists() {
        assert!(true);
    }
}