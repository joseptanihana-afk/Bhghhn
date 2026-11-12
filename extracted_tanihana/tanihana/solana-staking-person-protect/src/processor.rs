//! Program instruction processor for Solana Staking Person Protect Protocol (Online Live Chain)

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instructions::StakeInstruction;

/// Process instruction entry point for live online chain execution
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StakeInstruction::unpack(instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?;

    msg!("Processing online live chain person protection instruction: {:?}", instruction);

    match instruction {
        StakeInstruction::Initialize { owner, amount_lamports } => {
            msg!("Online live chain person: Initializing stake account for owner: {}, amount: {} lamports", owner, amount_lamports);
            Ok(())
        }
        StakeInstruction::Delegate { validator_pubkey, amount_lamports } => {
            msg!("Online live chain person: Delegating stake to validator: {}, amount: {} lamports", validator_pubkey, amount_lamports);
            Ok(())
        }
        StakeInstruction::Deactivate { amount_lamports } => {
            msg!("Online live chain person: Deactivating stake: {} lamports", amount_lamports);
            Ok(())
        }
        StakeInstruction::Withdraw { amount_lamports } => {
            msg!("Online live chain person: Withdrawing stake: {} lamports", amount_lamports);
            Ok(())
        }
        StakeInstruction::ClaimRewards => {
            msg!("Online live chain person: Claiming staking rewards");
            Ok(())
        }
        StakeInstruction::ActivateProtection { tier } => {
            msg!("Online live chain person: Activating partial protection tier: {:?}", tier);
            Ok(())
        }
        StakeInstruction::DeactivateProtection => {
            msg!("Online live chain person: Deactivating loss protection");
            Ok(())
        }
    }
}
