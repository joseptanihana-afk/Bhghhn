//! Instruction types for the staking program

use rust_decimal::Decimal;
use solana_program::program_error::ProgramError;

/// Stake instructions for the person-protect program
#[derive(Debug, Clone)]
pub enum StakeInstruction {
    /// Initialize a new stake account
    ///
    /// Accounts:
    /// - [writable] Stake account
    /// - [signer] Owner
    /// - System program
    Initialize { owner: Pubkey, amount_lamports: u64 },

    /// Delegate stake to a validator
    ///
    /// Accounts:
    /// - [writable] Stake account
    /// - System program
    Delegate { validator_pubkey: Pubkey, amount_lamports: u64 },

    /// Deactivate stake
    ///
    /// Accounts:
    /// - [writable] Stake account
    /// - System program
    Deactivate { amount_lamports: u64 },

    /// Withdraw stake
    ///
    /// Accounts:
    /// - [writable] Stake account
    /// - Destination account
    Withdraw { amount_lamports: u64 },

    /// Claim rewards
    ///
    /// Accounts:
    /// - [writable] Stake account
    /// - Rewards account
    ClaimRewards,

    /// Activate protection tier (not 100% - partial protection)
    ///
    /// Accounts:
    /// - [writable] Stake account
    /// - System program
    ActivateProtection { tier: ProtectionTier },

    /// Deactivate protection
    ///
    /// Accounts:
    /// - [writable] Stake account
    DeactivateProtection,
}

/// Pack/unpack support for instructions
pub trait StakeInstructionPack {
    fn pack(&self) -> Vec<u8>;
    fn unpack(data: &[u8]) -> Result<Self, ProgramError>;
}

impl StakeInstructionPack for StakeInstruction {
    fn pack(&self) -> Vec<u8> {
        match self {
            StakeInstruction::Initialize { owner: _, amount_lamports } => {
                // Pack implementation
                vec![0u8, *amount_lamports as u8, (amount_lamports >> 8) as u8]
            }
            StakeInstruction::Delegate { validator_pubkey: _, amount_lamports } => {
                vec![1u8, *amount_lamports as u8, (amount_lamports >> 8) as u8]
            }
            StakeInstruction::Deactivate { amount_lamports } => {
                vec![2u8, *amount_lamports as u8, (amount_lamports >> 8) as u8]
            }
            StakeInstruction::Withdraw { amount_lamports } => {
                vec![3u8, *amount_lamports as u8, (amount_lamports >> 8) as u8]
            }
            StakeInstruction::ClaimRewards => {
                vec![4u8]
            }
            StakeInstruction::ActivateProtection { tier: _ } => {
                vec![5u8]
            }
            StakeInstruction::DeactivateProtection => {
                vec![6u8]
            }
        }
    }

    fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        if data.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        match data[0] {
            0 => {
                let amount_lamports = ((data[2] as u64) << 8) | (data[1] as u64);
                Ok(StakeInstruction::Initialize { owner: Pubkey::new_unique(), amount_lamports })
            }
            1 => {
                let amount_lamports = ((data[2] as u64) << 8) | (data[1] as u64);
                Ok(StakeInstruction::Delegate { validator_pubkey: Pubkey::new_unique(), amount_lamports })
            }
            2 => {
                let amount_lamports = ((data[2] as u64) << 8) | (data[1] as u64);
                Ok(StakeInstruction::Deactivate { amount_lamports })
            }
            3 => {
                let amount_lamports = ((data[2] as u64) << 8) | (data[1] as u64);
                Ok(StakeInstruction::Withdraw { amount_lamports })
            }
            4 => Ok(StakeInstruction::ClaimRewards),
            5 => Ok(StakeInstruction::ActivateProtection { tier: ProtectionTier::Basic }),
            6 => Ok(StakeInstruction::DeactivateProtection),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}