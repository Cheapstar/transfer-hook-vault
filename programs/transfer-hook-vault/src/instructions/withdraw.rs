use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount,TransferChecked,transfer_checked,TokenInterface};
use anchor_spl::token_interface::{approve, Approve};

use crate::constant::{VAULT, WHITELISTED_ENTRY};
use crate::state::{UserVaultAccount, Vault};
use crate::error::VaultError;

// all this does is transfer from user_ata to vault_ata
#[derive(Accounts)]
pub struct WithDraw<'info> {
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
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program, 
    )]
    pub vault_ata:InterfaceAccount<'info,TokenAccount>,

    #[account(mut)]
    /// CHECK: checking constraints Manually to save CU's
    pub user_vault_data:Account<'info,UserVaultAccount>,

    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>
}



impl<'info> WithDraw<'info> {
    pub fn withdraw(&mut self, withdraw_amount:u64)->Result<()> {

        
        let (expected_key,bump) = Pubkey::find_program_address(
            &[WHITELISTED_ENTRY.as_bytes(),self.user.key().as_ref(),self.mint.key().as_ref(),self.vault.seeds.to_le_bytes().as_ref()]
                    , &crate::id());

        require_keys_eq!(self.user_vault_data.key(),expected_key,VaultError::AccountMisMatch);
        require_eq!(self.user_vault_data.bump,bump,VaultError::BumpMisMatch);
        require_keys_eq!(self.user_vault_data.mint,self.mint.key(),VaultError::MintMisMatch);
        require_keys_eq!(self.user_vault_data.user,self.user.key(),VaultError::UserMisMatch);


        require!(withdraw_amount <= self.user_vault_data.deposited, VaultError::WithdrawTooMuch);
    



        // Approve user as delegate on vault's token account.
        // Client must follow this with a transfer_checked ix in the same tx.
        // Since the user is the delegate authority, the transfer hook
        // checks the user's whitelist — no need to whitelist the vault PDA.
        let vault_seeds = self.vault.seeds.to_le_bytes();
        let bump = self.vault.bump;
        let signer_seeds: &[&[&[u8]]] = &[&[VAULT.as_bytes(),vault_seeds.as_ref(), &[bump]]];

        approve(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Approve {
                    to: self.vault_ata.to_account_info(),
                    delegate: self.user.to_account_info(),
                    authority: self.vault.to_account_info(),
                },
                signer_seeds,
            ),
            withdraw_amount,
        )?;


        let new_deposited = self.user_vault_data.deposited.checked_sub(withdraw_amount).ok_or(VaultError::WithdrawTooMuch)?;

        self.user_vault_data.deposited = new_deposited;
        self.vault.amount = self.vault.amount
            .checked_sub(withdraw_amount)
            .ok_or(VaultError::WithdrawTooMuch)?;

        msg!("Approved withdrawal of {} tokens", withdraw_amount);
        Ok(())
    }
}