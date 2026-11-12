//! Loss protection mechanism for the staking program (partial protection, not 100%)

use crate::errors::StakingProtectError;
use crate::state::{ProtectionConfig, ProtectionTier};

/* Loss protection logic for person staking (not 100% drip) */
pub struct ProtectionLogic;

impl ProtectionLogic {
    pub fn protection_percentage(tier: ProtectionTier) -> u8 {
        ProtectionConfig::for_tier(tier).protected_percentage
    }

    pub fn calculate_protected_amount(stake_lamports: u64, tier: ProtectionTier) -> u64 {
        let config = ProtectionConfig::for_tier(tier);
        (stake_lamports as u128)
            .checked_mul(config.protected_percentage as u128)
            .and_then(|v| v.checked_div(100))
            .unwrap_or(0) as u64
    }

    pub fn calculate_protection_payout(protected: u64, lost: u64) -> u64 {
        core::cmp::min(protected, lost)
    }

    pub fn calculate_effective_loss(lost: u64, payout: u64) -> u64 {
        lost.saturating_sub(payout)
    }

    pub fn is_fully_protected(tier: ProtectionTier) -> bool {
        // Person protection is NOT 100% - only partial
        matches!(tier, ProtectionTier::Premium) // Premium is 50%, not full
    }

    pub fn calculate_protection_fee(stake_lamports: u64, tier: ProtectionTier) -> u64 {
        ProtectionConfig::calculate_protection_fee(stake_lamports, tier)
    }

    pub fn is_wallet_sufficient_for_protection(wallet_balance: u64, stake_lamports: u64) -> bool {
        ProtectionConfig::is_wallet_sufficient(wallet_balance, stake_lamports)
    }

    pub fn verify_full_protection_on_blockchain(tier: ProtectionTier, active: bool) -> bool {
        active && Self::is_fully_protected(tier)
    }
}
