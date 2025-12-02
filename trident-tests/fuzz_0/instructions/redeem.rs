use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("2WXpY8havGjfRxme9LUxtjFHTh1EfU3ur4v6wiK4KdNC")]
#[discriminator([184u8, 12u8, 86u8, 149u8, 70u8, 196u8, 97u8, 225u8])]
pub struct RedeemInstruction {
    pub accounts: RedeemInstructionAccounts,
    pub data: RedeemInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(RedeemInstructionData)]
#[storage(FuzzAccounts)]
pub struct RedeemInstructionAccounts {
    pub identity_pda: TridentAccount,

    #[account(mut)]
    pub swap_data: TridentAccount,

    #[account(mut)]
    pub token_vault: TridentAccount,

    #[account(mut)]
    pub redeemer_token_account: TridentAccount,

    #[account(mut)]
    pub rent_sponsor: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct RedeemInstructionData {
    pub secret: [u8; 32],
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for RedeemInstruction {
    type IxAccounts = FuzzAccounts;
}
