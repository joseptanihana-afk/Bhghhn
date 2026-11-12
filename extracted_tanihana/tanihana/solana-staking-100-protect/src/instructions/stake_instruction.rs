//! Instruction types for the staking program

use solana_program::program_error::ProgramError;

/* Stake instructions for the 100-protect program */
#[derive(Debug, Clone)]
pub enum StakeInstruction {
    /// Initialize a new stake account
    Initialize { owner: solana_program::pubkey::Pubkey, amount_lamports: u64 },

    /// Delegate stake to a validator
    Delegate { validator_pubkey: solana_program::pubkey::Pubkey, amount_lamports: u64 },

    /// Deactivate stake
    Deactivate { amount_lamports: u64 },

    /// Withdraw stake
    Withdraw { amount_lamports: u64 },

    /// Claim rewards
    ClaimRewards,

    /// Activate protection (100% tier)
    ActivateProtection,

    /// Deactivate protection
    DeactivateProtection,
}

/* Pack/unpack support for instructions */
pub trait StakeInstructionPack {
    fn pack(&self) -> Vec<u8>;
    fn unpack(data: &[u8]) -> Result<Self, ProgramError>;
}

impl StakeInstructionPack for StakeInstruction {
    fn pack(&self) -> Vec<u8> {
        match self {
            StakeInstruction::Initialize { owner: _, amount_lamports } => {
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
            StakeInstruction::ActivateProtection => {
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
            0 => Ok(StakeInstruction::Initialize { owner: solana_program::pubkey::Pubkey::new_unique(), amount_lamports: ((data[2] as u64) << 8) | (data[1] as u64) }),
            1 => Ok(StakeInstruction::Delegate { validator_pubkey: solana_program::pubkey::Pubkey::new_unique(), amount_lamports: ((data[2] as u64) << 8) | (data[1] as u64) }),
            2 => Ok(StakeInstruction::Deactivate { amount_lamports: ((data[2] as u64) << 8) | (data[1] as u64) }),
            3 => Ok(StakeInstruction::Withdraw { amount_lamports: ((data[2] as u64) << 8) | (data[1] as u64) }),
            4 => Ok(StakeInstruction::ClaimRewards),
            5 => Ok(StakeInstruction::ActivateProtection),
            6 => Ok(StakeInstruction::DeactivateProtection),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
