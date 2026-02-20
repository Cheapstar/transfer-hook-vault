use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub admin:Pubkey,
    pub mint:Pubkey,
    pub amount:u64,
    pub seeds:u64,
    pub number_of_users:u64,
    pub bump:u8
}


#[account]
#[derive(InitSpace)]
pub struct UserVaultData {
    pub user:Pubkey,
    pub mint:Pubkey,
    pub deposited:u64,
    pub allowed:bool,
    pub seeds:u64,
    pub bump:u8
}