use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub admin:Pubkey,
    pub mint:Pubkey,
    pub amount:u64,
    pub seeds:u64,
    pub bump:u8
}