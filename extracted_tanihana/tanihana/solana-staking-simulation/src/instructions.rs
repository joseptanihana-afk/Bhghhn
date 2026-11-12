//! Production instruction definitions - Real on-chain protection protocol

use solana_program::pubkey::Pubkey;
use serde::{Deserialize, Serialize};
use crate::errors::StakingProductionError;

/// All instructions supported by the production protection program
/// This is a REAL on-chain protocol for Solana mainnet/devnet deployment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProductionInstruction {
    InitializeStake { owner: Pubkey, amount_lamports: u64 },
    DelegateStake { validator_pubkey: Pubkey, amount_lamports: u64 },
    DeactivateStake { amount_lamports: u64 },
    WithdrawStake { amount_lamports: u64 },
    ClaimRewards,
    ActivateHundredPercentProtection { wallet_balance_lamports: u64 },
    DeactivateProtection,
    CalculateProtectionFee { stake_lamports: u64 },
    VerifyFullProtection,
}

impl ProductionInstruction {
    /// Unpack instruction data from bytes
    pub fn unpack(input: &[u8]) -> Result<Self, StakingProductionError> {
        if input.is_empty() {
            return Err(StakingProductionError::InvalidInstructionData);
        }

        let tag = input[0];
        let rest = &input[1..];

        match tag {
            0 => {
                if rest.len() < 32 + 8 {
                    return Err(StakingProductionError::InvalidInstructionData);
                }
                let owner = Pubkey::try_from(rest[..32].try_into().map_err(|_| StakingProductionError::InvalidInstructionData)?)
                    .map_err(|_| StakingProductionError::InvalidInstructionData)?;
                let amount = u64::from_le_bytes(rest[32..40].try_into().map_err(|_| StakingProductionError::InvalidInstructionData)?);
                Ok(ProductionInstruction::InitializeStake { owner, amount_lamports: amount })
            }
            1 => {
                if rest.len() < 32 + 8 {
                    return Err(StakingProductionError::InvalidInstructionData);
                }
                let validator = Pubkey::try_from(rest[..32].try_into().map_err(|_| StakingProductionError::InvalidInstructionData)?)
                    .map_err(|_| StakingProductionError::InvalidInstructionData)?;
                let amount = u64::from_le_bytes(rest[32..40].try_into().map_err(|_| StakingProductionError::InvalidInstructionData)?);
                Ok(ProductionInstruction::DelegateStake { validator_pubkey: validator, amount_lamports: amount })
            }
            2 => {
                if rest.len() < 8 {
                    return Err(StakingProductionError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[..8].try_into().map_err(|_| StakingProductionError::InvalidInstructionData)?);
                Ok(ProductionInstruction::DeactivateStake { amount_lamports: amount })
            }
            3 => {
                if rest.len() < 8 {
                    return Err(StakingProductionError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[..8].try_into().map_err(|_| StakingProductionError::InvalidInstructionData)?);
                Ok(ProductionInstruction::WithdrawStake { amount_lamports: amount })
            }
            4 => Ok(ProductionInstruction::ClaimRewards),
            5 => {
                if rest.len() < 8 {
                    return Err(StakingProductionError::InvalidInstructionData);
                }
                let wallet_balance = u64::from_le_bytes(rest[..8].try_into().map_err(|_| StakingProductionError::InvalidInstructionData)?);
                Ok(ProductionInstruction::ActivateHundredPercentProtection { wallet_balance_lamports: wallet_balance })
            }
            6 => Ok(ProductionInstruction::DeactivateProtection),
            7 => {
                if rest.len() < 8 {
                    return Err(StakingProductionError::InvalidInstructionData);
                }
                let stake = u64::from_le_bytes(rest[..8].try_into().map_err(|_| StakingProductionError::InvalidInstructionData)?);
                Ok(ProductionInstruction::CalculateProtectionFee { stake_lamports: stake })
            }
            8 => Ok(ProductionInstruction::VerifyFullProtection),
            _ => Err(StakingProductionError::InvalidInstructionData),
        }
    }

    /// Pack instruction into bytes
    pub fn pack(&self) -> Vec<u8> {
        let mut data = Vec::new();
        match self {
            ProductionInstruction::InitializeStake { owner, amount_lamports } => {
                data.push(0);
                data.extend_from_slice(owner.as_ref());
                data.extend_from_slice(&amount_lamports.to_le_bytes());
            }
            ProductionInstruction::DelegateStake { validator_pubkey, amount_lamports } => {
                data.push(1);
                data.extend_from_slice(validator_pubkey.as_ref());
                data.extend_from_slice(&amount_lamports.to_le_bytes());
            }
            ProductionInstruction::DeactivateStake { amount_lamports } => {
                data.push(2);
                data.extend_from_slice(&amount_lamports.to_le_bytes());
            }
            ProductionInstruction::WithdrawStake { amount_lamports } => {
                data.push(3);
                data.extend_from_slice(&amount_lamports.to_le_bytes());
            }
            ProductionInstruction::ClaimRewards => {
                data.push(4);
            }
            ProductionInstruction::ActivateHundredPercentProtection { wallet_balance_lamports } => {
                data.push(5);
                data.extend_from_slice(&wallet_balance_lamports.to_le_bytes());
            }
            ProductionInstruction::DeactivateProtection => {
                data.push(6);
            }
            ProductionInstruction::CalculateProtectionFee { stake_lamports } => {
                data.push(7);
                data.extend_from_slice(&stake_lamports.to_le_bytes());
            }
            ProductionInstruction::VerifyFullProtection => {
                data.push(8);
            }
        }
        data
    }
}
