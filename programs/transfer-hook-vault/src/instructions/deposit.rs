use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount,TransferChecked,transfer_checked,TokenInterface};

use crate::constant::{VAULT, WHITELISTED_ENTRY};
use crate::state::{UserVaultAccount, Vault};
use crate::error::VaultError;

// all this does is transfer from user_ata to vault_ata
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user:Signer<'info>,

    pub mint:InterfaceAccount<'info,Mint>,

    #[account(
        mut, 
        has_one = mint,
        seeds = [VAULT.as_bytes(),vault.seeds.to_le_bytes().as_ref()],
        bump
    )]
    pub vault : Account<'info,Vault>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program, 
    )]
    pub user_ata:InterfaceAccount<'info,TokenAccount>,

    #[account(mut)]
    /// Checking it manually to save CU's Maybe!!
    pub user_vault_data:Account<'info,UserVaultAccount>,

    /// THIS IS GIVEN AS I AM USING TOKEN2022 PROGRAM
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>
}



impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, deposit_amount:u64)->Result<()> {

        msg!("Depositing Tokens : {} ", deposit_amount);
        let (expected_key,bump) = Pubkey::find_program_address(
            &[WHITELISTED_ENTRY.as_bytes(),self.user.key().as_ref(),self.mint.key().as_ref(),self.vault.seeds.to_le_bytes().as_ref()]
                    , &crate::id());

        require_keys_eq!(self.user_vault_data.key(),expected_key,VaultError::AccountMisMatch);
        require_eq!(self.user_vault_data.bump,bump,VaultError::BumpMisMatch);
        require_keys_eq!(self.user_vault_data.mint,self.mint.key(),VaultError::MintMisMatch);
        require_keys_eq!(self.user_vault_data.user,self.user.key(),VaultError::UserMisMatch);

        require!(self.user_ata.amount >= deposit_amount , VaultError::InsufficientFunds);
        
        // TRANSFER NEEDED TO BE DONE MANUALLY AS TRANSFER HOOK IS DEFINED IN THE SAME PROGRAM SO REENTRANCY ERROR WILL OCCUR
        self.vault.amount = self.vault.amount.checked_add(deposit_amount).ok_or(VaultError::Overflow)?;

        self.user_vault_data.deposited = self.user_vault_data.deposited.checked_add(deposit_amount).ok_or(VaultError::Overflow)?;


        msg!("Tokens Successfully Deposited");
        Ok(())
    }
}