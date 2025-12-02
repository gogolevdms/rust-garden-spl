use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub token_program: AccountsStorage,

    pub system_program: AccountsStorage,

    pub redeemer_token_account: AccountsStorage,

    pub funder: AccountsStorage,

    pub redeemer: AccountsStorage,

    pub refundee_token_account: AccountsStorage,

    pub swap_data: AccountsStorage,

    pub token_vault: AccountsStorage,

    pub rent_sponsor: AccountsStorage,

    pub mint: AccountsStorage,

    pub funder_token_account: AccountsStorage,

    pub identity_pda: AccountsStorage,
}
