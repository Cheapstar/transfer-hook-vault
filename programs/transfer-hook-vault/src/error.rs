use anchor_lang::prelude::*;


#[error_code]
pub enum VaultError {
    #[msg("User has Insufficient Funds")]
    InsufficientFunds,
    #[msg("You cannot Withdraw more than your deposit")]
    WithdrawTooMuch,
}