//! Instruction types for the staking program

pub use crate::instructions::stake_instruction::StakeInstruction;

pub mod stake_processor;

pub use stake_processor::process_instruction;