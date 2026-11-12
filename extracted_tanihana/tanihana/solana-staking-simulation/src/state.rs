//! Production stake account state - Real on-chain state structures

use solana_program::pubkey::Pubkey;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{Pack, IsInitialized};
use serde::{Deserialize, Serialize};

/// State of a production stake account
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductionStakeState {
    Uninitialized,
    Staking,
    Deactivating,
    Deactivated,
    Closed,
}

/// Protection tier for production
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductionProtectionTier {
    Basic = 1,
    Standard = 2,
    Premium = 3,
    Maximum = 4,
}

/// Production stake account struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionStakeAccount {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub validator: Option<Pubkey>,
    pub stake_lamports: u64,
    pub rewards_lamports: u64,
    pub state: u8,
    pub activation_epoch: Option<u64>,
    pub deactivation_epoch: Option<u64>,
    pub lockup_epochs: u64,
    pub protection_active: bool,
    pub protection_tier: Option<u8>,
    pub protected_amount_lamports: u64,
    pub protection_fee_paid: u64,
}

impl ProductionStakeAccount {
    pub fn new(owner: Pubkey, amount_lamports: u64) -> Self {
        Self {
            is_initialized: true,
            owner,
            validator: None,
            stake_lamports: amount_lamports,
            rewards_lamports: 0,
            state: ProductionStakeState::Uninitialized as u8,
            activation_epoch: None,
            deactivation_epoch: None,
            lockup_epochs: 1,
            protection_active: false,
            protection_tier: None,
            protected_amount_lamports: 0,
            protection_fee_paid: 0,
        }
    }

    pub fn is_fully_protected(&self) -> bool {
        self.protection_active && self.protection_tier == Some(ProductionProtectionTier::Maximum as u8)
    }

    pub fn protection_percentage(&self) -> u8 {
        match self.protection_tier {
            Some(1) => 10,
            Some(2) => 25,
            Some(3) => 50,
            Some(4) => 100,
            _ => 0,
        }
    }
}

impl Pack for ProductionStakeAccount {
    const LEN: usize = 1 + 32 + 33 + 8 + 8 + 1 + 9 + 9 + 8 + 1 + 2 + 8 + 8;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut offset = 0;
        dst[offset] = self.is_initialized as u8;
        offset += 1;
        dst[offset..offset + 32].copy_from_slice(self.owner.as_ref());
        offset += 32;
        match self.validator {
            Some(v) => {
                dst[offset] = 1;
                offset += 1;
                dst[offset..offset + 32].copy_from_slice(v.as_ref());
                offset += 32;
            }
            None => {
                dst[offset] = 0;
                offset += 1;
            }
        }
        dst[offset..offset + 8].copy_from_slice(&self.stake_lamports.to_le_bytes());
        offset += 8;
        dst[offset..offset + 8].copy_from_slice(&self.rewards_lamports.to_le_bytes());
        offset += 8;
        dst[offset] = self.state;
        offset += 1;
        match self.activation_epoch {
            Some(e) => {
                dst[offset] = 1;
                offset += 1;
                dst[offset..offset + 8].copy_from_slice(&e.to_le_bytes());
                offset += 8;
            }
            None => {
                dst[offset] = 0;
                offset += 1;
            }
        }
        match self.deactivation_epoch {
            Some(e) => {
                dst[offset] = 1;
                offset += 1;
                dst[offset..offset + 8].copy_from_slice(&e.to_le_bytes());
                offset += 8;
            }
            None => {
                dst[offset] = 0;
                offset += 1;
            }
        }
        dst[offset..offset + 8].copy_from_slice(&self.lockup_epochs.to_le_bytes());
        offset += 8;
        dst[offset] = self.protection_active as u8;
        offset += 1;
        dst[offset] = self.protection_tier.map_or(0, |t| t as u8);
        offset += 1;
        dst[offset..offset + 8].copy_from_slice(&self.protected_amount_lamports.to_le_bytes());
        offset += 8;
        dst[offset..offset + 8].copy_from_slice(&self.protection_fee_paid.to_le_bytes());
    }
}

impl ProductionStakeAccount {
    pub fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        if src.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let mut offset = 0;
        let is_initialized = src[offset] != 0;
        offset += 1;
        let owner = Pubkey::try_from(&src[offset..offset + 32]).map_err(|_| ProgramError::InvalidAccountData)?;
        offset += 32;
        let validator_flag = src[offset];
        offset += 1;
        let validator = if validator_flag == 1 {
            Some(Pubkey::try_from(&src[offset..offset + 32]).map_err(|_| ProgramError::InvalidAccountData)?)
        } else {
            None
        };
        offset += 32;
        let stake_lamports = u64::from_le_bytes(src[offset..offset + 8].try_into().map_err(|_| ProgramError::InvalidAccountData)?);
        offset += 8;
        let rewards_lamports = u64::from_le_bytes(src[offset..offset + 8].try_into().map_err(|_| ProgramError::InvalidAccountData)?);
        offset += 8;
        let state = src[offset];
        offset += 1;
        let activation_flag = src[offset];
        offset += 1;
        let activation_epoch = if activation_flag == 1 {
            Some(u64::from_le_bytes(src[offset..offset + 8].try_into().map_err(|_| ProgramError::InvalidAccountData)?))
        } else {
            None
        };
        offset += 8;
        let deactivation_flag = src[offset];
        offset += 1;
        let deactivation_epoch = if deactivation_flag == 1 {
            Some(u64::from_le_bytes(src[offset..offset + 8].try_into().map_err(|_| ProgramError::InvalidAccountData)?))
        } else {
            None
        };
        offset += 8;
        let lockup_epochs = u64::from_le_bytes(src[offset..offset + 8].try_into().map_err(|_| ProgramError::InvalidAccountData)?);
        offset += 8;
        let protection_active = src[offset] != 0;
        offset += 1;
        let protection_tier = Some(src[offset]);
        offset += 1;
        let protected_amount_lamports = u64::from_le_bytes(src[offset..offset + 8].try_into().map_err(|_| ProgramError::InvalidAccountData)?);
        let protection_fee_paid = u64::from_le_bytes(src[offset..offset + 8].try_into().map_err(|_| ProgramError::InvalidAccountData)?);

        Ok(Self {
            is_initialized,
            owner,
            validator,
            stake_lamports,
            rewards_lamports,
            state,
            activation_epoch,
            deactivation_epoch,
            lockup_epochs,
            protection_active,
            protection_tier,
            protected_amount_lamports,
            protection_fee_paid,
        })
    }
}

impl IsInitialized for ProductionStakeAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
