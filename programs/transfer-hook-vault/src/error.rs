use anchor_lang::prelude::*;


#[error_code]
pub enum VaultError {
    #[msg("User has Insufficient Funds")]
    InsufficientFunds,
    #[msg("You cannot Withdraw more than your deposit")]
    WithdrawTooMuch,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("OwnerMismatch")]
    OwnerMisMatch,
    #[msg("AccountMisMatch")]
    AccountMisMatch,
    #[msg("BumpMisMatch")]
    BumpMisMatch,
    #[msg("MintMisMatch")]
    MintMisMatch,
    #[msg("UserMisMatch")]
    UserMisMatch,
    #[msg("User is BlackListed , Cannot Proceed With The Transfer")]
    BlackListed,
    #[msg("Invalid Extra Meta")]
    InvalidExtraMeta,
    #[msg("Invalid account size")]
    InvalidAccountSize,

}