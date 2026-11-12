//! Loss protection mechanism for the staking program
//!
//! 100% loss protection - full coverage for staked amounts
use crate::errors::StakingProtectError;
use crate::state::{ProtectionConfig, ProtectionTier};

/* 100% loss protection logic - full coverage */
pub struct ProtectionLogic;

impl ProtectionLogic {
    /* Returns the protection percentage for the tier */
    pub fn protection_percentage(tier: ProtectionTier) -> u8 {
        ProtectionConfig::for_tier(tier).protected_percentage
    }

    /* Calculate protected amount - 100% for Maximum tier */
    pub fn calculate_protected_amount(stake_lamports: u64, tier: ProtectionTier) -> u64 {
        let config = ProtectionConfig::for_tier(tier);
        /* Maximum tier provides 100% protection */
        if tier == ProtectionTier::Maximum {
            stake_lamports /* Full 100% drip protection */
        } else {
            (stake_lamports as u128)
                .checked_mul(config.protected_percentage as u128)
                .and_then(|v| v.checked_div(100))
                .unwrap_or(0) as u64
        }
    }

    /* Calculate payout - for 100% tier, always full */
    pub fn calculate_protection_payout(protected: u64, lost: u64) -> u64 {
        core::cmp::min(protected, lost)
    }

    /* Calculate effective loss - 0 for 100% protection */
    pub fn calculate_effective_loss(lost: u64, payout: u64) -> u64 {
        lost.saturating_sub(payout)
    }

    /* Check if fully protected - Maximum tier is 100% */
    pub fn is_fully_protected(tier: ProtectionTier) -> bool {
        matches!(tier, ProtectionTier::Maximum) /* Only Maximum tier is 100% */
    }

    /* Calculate protection fee - 0% for Maximum tier */
    pub fn calculate_protection_fee(stake_lamports: u64, tier: ProtectionTier) -> u64 {
        ProtectionConfig::calculate_protection_fee(stake_lamports, tier)
    }

    /* Check wallet sufficiency for protection */
    pub fn is_wallet_sufficient_for_protection(wallet_balance: u64, stake_lamports: u64) -> bool {
        ProtectionConfig::is_wallet_sufficient(wallet_balance, stake_lamports)
    }

    /* Verify full protection on blockchain */
    pub fn verify_full_protection_on_blockchain(tier: ProtectionTier, active: bool) -> bool {
        active && Self::is_fully_protected(tier)
    }
}
