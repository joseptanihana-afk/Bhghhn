//! Protection configuration for 100% loss protection

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/* 100% protection - full coverage */
pub const PROTECTION_FEE_PERCENT: Decimal = Decimal::new(0, 0); /* 0% fee for maximum protection */

/* Protection tier configuration - includes 100% Maximum tier */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtectionTier {
    Basic = 1,
    Standard = 2,
    Premium = 3,
    Maximum = 4, /* 100% protection tier */
}

/* Protection config with percentages */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectionConfig {
    pub tier: ProtectionTier,
    pub protected_percentage: u8,
    pub lockup_epochs: u64,
    pub fee_percentage: Decimal,
}

impl ProtectionConfig {
    pub fn for_tier(tier: ProtectionTier) -> Self {
        match tier {
            ProtectionTier::Basic => ProtectionConfig {
                tier: ProtectionTier::Basic,
                protected_percentage: 10,
                lockup_epochs: 1,
                fee_percentage: Decimal::new(9, 4),
            },
            ProtectionTier::Standard => ProtectionConfig {
                tier: ProtectionTier::Standard,
                protected_percentage: 25,
                lockup_epochs: 2,
                fee_percentage: Decimal::new(9, 4),
            },
            ProtectionTier::Premium => ProtectionConfig {
                tier: ProtectionTier::Premium,
                protected_percentage: 50,
                lockup_epochs: 4,
                fee_percentage: Decimal::new(9, 4),
            },
            ProtectionTier::Maximum => ProtectionConfig {
                tier: ProtectionTier::Maximum,
                protected_percentage: 100, /* 100% drip - full protection */
                lockup_epochs: 1,
                fee_percentage: Decimal::new(0, 0), /* 0% fee for maximum protection */
            },
        }
    }

    pub fn from_u8(value: u8) -> Option<ProtectionTier> {
        match value {
            1 => Some(ProtectionTier::Basic),
            2 => Some(ProtectionTier::Standard),
            3 => Some(ProtectionTier::Premium),
            4 => Some(ProtectionTier::Maximum),
            _ => None,
        }
    }

    /* Calculate 100% protected amount - full coverage */
    pub fn calculate_protected_amount(stake_lamports: u64, tier: ProtectionTier) -> u64 {
        let config = Self::for_tier(tier);
        /* For Maximum tier, protect 100% of stake */
        if tier == ProtectionTier::Maximum {
            stake_lamports /* Full 100% protection */
        } else {
            (stake_lamports as u128)
                .checked_mul(config.protected_percentage as u128)
                .and_then(|v| v.checked_div(100))
                .unwrap_or(0) as u64
        }
    }

    pub fn calculate_protection_fee(stake_lamports: u64, tier: ProtectionTier) -> u64 {
        let config = Self::for_tier(tier);
        let fee = (stake_lamports as u128)
            .checked_mul(config.fee_percentage.mantissa() as u128)
            .and_then(|v| v.checked_div(10u128.pow(config.fee_percentage.scale() as u32) * 100))
            .unwrap_or(0) as u64;
        fee
    }
}

impl From<ProtectionTier> for u8 {
    fn from(tier: ProtectionTier) -> u8 {
        tier as u8
    }
}
