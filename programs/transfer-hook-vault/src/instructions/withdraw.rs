use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount,TransferChecked,transfer_checked,TokenInterface};

use crate::constant::{VAULT, WHITELISTED_ENTRY};
use crate::state::{UserVaultAccount, Vault};
use crate::error::VaultError;

// all this does is transfer from user_ata to vault_ata
#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct WithDraw<'info> {
    #[account(mut)]
    pub user:Signer<'info>,

    pub mint:InterfaceAccount<'info,Mint>,

    #[account(
        mut, 
        has_one = mint,
        seeds = [VAULT.as_bytes(),seeds.to_le_bytes().as_ref()],
        bump
    )]
    pub vault : Account<'info,Vault>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_ata:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault
    )]
    pub vault_ata:InterfaceAccount<'info,TokenAccount>,

    /// CHECK: checking constraints Manually to save CU's
    pub user_vault_data:Account<'info,UserVaultAccount>,

    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>
}



impl<'info> WithDraw<'info> {
    pub fn withdraw(&mut self, withdraw_amount:u64,seeds:u64)->Result<()> {

        
        let (expected_key,bump) = Pubkey::find_program_address(
            &[WHITELISTED_ENTRY.as_bytes(),self.user.key().as_ref(),self.mint.key().as_ref(),self.vault.seeds.to_le_bytes().as_ref()]
                    , &crate::id());

        require_eq!(self.user_vault_data.key(),expected_key,VaultError::AccountMisMatch);
        require_eq!(self.user_vault_data.bump,bump,VaultError::BumpMisMatch);
        require_eq!(self.user_vault_data.mint,self.mint.key(),VaultError::MintMisMatch);
        require_eq!(self.user_vault_data.user,self.user.key(),VaultError::UserMisMatch);


        require!(withdraw_amount > self.user_vault_data.deposited  , VaultError::WithdrawTooMuch);
        
        let binding = self.vault.seeds.to_le_bytes();
        let vault_seeds = binding.as_ref();
        let signer_seeds:&[&[u8]] =&[VAULT.as_bytes(),vault_seeds,&[self.vault.bump]];


        let transfer_acc = TransferChecked {
            from: self.vault_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.user_ata.to_account_info(),
            authority: self.vault.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let binding = [signer_seeds];
        let transfer_ctx = CpiContext::new_with_signer(cpi_program, transfer_acc, &binding);

        transfer_checked(transfer_ctx, withdraw_amount, self.mint.decimals)?;

        let new_deposited = self.user_vault_data.deposited.checked_sub(withdraw_amount).unwrap();

        self.user_vault_data.deposited = new_deposited;

        Ok(())
    }
}