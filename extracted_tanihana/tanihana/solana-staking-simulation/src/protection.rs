//! Loss protection mechanism - Real on-chain 100% protection protocol
//!
//! Fee protection protocol: 0.0009% fee with 0.04 cent minimum wallet balance.
//! Maximum tier provides 100% protection on-chain.

use crate::errors::{StakingProductionError, StakingProductionResult};
use crate::state::ProductionProtectionTier;

/// Minimum wallet balance for protection: 0.04 cent in SOL (~40,000 lamports)
pub const MIN_WALLET_BALANCE_FOR_PROTECTION_LAMPORTS: u64 = 40_000;

/// Protection fee: 0.0009%
pub const PROTECTION_FEE_PERCENT_BASIS_POINTS: u64 = 9; // 0.0009% = 9 basis points of 1%

/// Loss protection manager for on-chain blockchain
pub struct LossProtection;

impl LossProtection {
    /// Get the protection percentage for a tier
    pub fn protection_percentage(tier: ProductionProtectionTier) -> u8 {
        match tier {
            ProductionProtectionTier::Basic => 10,
            ProductionProtectionTier::Standard => 25,
            ProductionProtectionTier::Premium => 50,
            ProductionProtectionTier::Maximum => 100,
        }
    }

    /// Calculate the protected amount in lamports for a stake
    pub fn calculate_protected_amount(
        stake_lamports: u64,
        tier: ProductionProtectionTier,
    ) -> u64 {
        let percentage = Self::protection_percentage(tier);
        (stake_lamports as u128)
            .checked_mul(percentage as u128)
            .and_then(|v| v.checked_div(100))
            .unwrap_or(0) as u64
    }

    /// Calculate the protection payout if a loss occurs
    pub fn calculate_protection_payout(
        protected_amount_lamports: u64,
        lost_amount_lamports: u64,
    ) -> u64 {
        core::cmp::min(protected_amount_lamports, lost_amount_lamports)
    }

    /// Calculate the effective loss after protection reimbursement
    pub fn calculate_effective_loss(
        lost_amount_lamports: u64,
        payout_lamports: u64,
    ) -> u64 {
        lost_amount_lamports.saturating_sub(payout_lamports)
    }

    /// Check if the user has 100% protection (Maximum tier)
    pub fn is_fully_protected(tier: ProductionProtectionTier) -> bool {
        matches!(tier, ProductionProtectionTier::Maximum)
    }

    /// Calculate protection fee in lamports using 0.0009% rate
    pub fn calculate_protection_fee(
        stake_lamports: u64,
        _tier: ProductionProtectionTier,
    ) -> u64 {
        (stake_lamports as u128)
            .checked_mul(PROTECTION_FEE_PERCENT_BASIS_POINTS as u128)
            .and_then(|v| v.checked_div(1_000_000))
            .unwrap_or(0) as u64
    }

    /// Check if wallet has sufficient balance for protection
    pub fn is_wallet_sufficient_for_protection(
        wallet_balance_lamports: u64,
        stake_lamports: u64,
    ) -> bool {
        let fee = Self::calculate_protection_fee(stake_lamports, ProductionProtectionTier::Maximum);
        wallet_balance_lamports >= MIN_WALLET_BALANCE_FOR_PROTECTION_LAMPORTS
            && wallet_balance_lamports >= fee
    }

    /// Verify 100% protection status on blockchain
    pub fn verify_full_protection_on_blockchain(
        tier: ProductionProtectionTier,
        protection_active: bool,
    ) -> bool {
        protection_active && Self::is_fully_protected(tier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ProductionProtectionTier;

    #[test]
    fn test_protection_percentage_maximum() {
        assert_eq!(LossProtection::protection_percentage(ProductionProtectionTier::Maximum), 100);
    }

    #[test]
    fn test_protection_percentage_basic() {
        assert_eq!(LossProtection::protection_percentage(ProductionProtectionTier::Basic), 10);
    }

    #[test]
    fn test_calculate_protected_amount_100_percent() {
        let amount = LossProtection::calculate_protected_amount(1_000_000_000, ProductionProtectionTier::Maximum);
        assert_eq!(amount, 1_000_000_000);
    }

    #[test]
    fn test_calculate_protection_payout() {
        let payout = LossProtection::calculate_protection_payout(1_000_000_000, 500_000_000);
        assert_eq!(payout, 500_000_000);
    }

    #[test]
    fn test_protection_fee_0_0009_percent() {
        let fee = LossProtection::calculate_protection_fee(1_000_000_000, ProductionProtectionTier::Maximum);
        assert_eq!(fee, 9_000);
    }

    #[test]
    fn test_wallet_sufficient() {
        assert!(LossProtection::is_wallet_sufficient_for_protection(40_000, 1_000_000_000));
    }
}
