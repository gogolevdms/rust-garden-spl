use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("2WXpY8havGjfRxme9LUxtjFHTh1EfU3ur4v6wiK4KdNC")]
#[discriminator([211u8, 202u8, 103u8, 41u8, 183u8, 147u8, 59u8, 251u8])]
pub struct InstantRefundInstruction {
    pub accounts: InstantRefundInstructionAccounts,
    pub data: InstantRefundInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InstantRefundInstructionData)]
#[storage(FuzzAccounts)]
pub struct InstantRefundInstructionAccounts {
    pub identity_pda: TridentAccount,

    #[account(mut)]
    pub swap_data: TridentAccount,

    #[account(mut)]
    pub token_vault: TridentAccount,

    #[account(mut)]
    pub refundee_token_account: TridentAccount,

    #[account(mut, signer)]
    pub redeemer: TridentAccount,

    #[account(mut)]
    pub rent_sponsor: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InstantRefundInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InstantRefundInstruction {
    type IxAccounts = FuzzAccounts;
}
