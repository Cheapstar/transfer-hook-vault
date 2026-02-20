use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount,TransferChecked,transfer_checked,TokenInterface};

use crate::state::{UserVaultData, Vault};
use crate::error::VaultError;

// all this does is transfer from user_ata to vault_ata
#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user:Signer<'info>,

    pub mint:InterfaceAccount<'info,Mint>,

    #[account(
        mut, 
        has_one = mint,
        seeds = [b"vault",seeds.to_le_bytes().as_ref()],
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

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserVaultData::INIT_SPACE,
        seeds = [b"user_vault",user.key().as_ref()],
        bump
    )]
    pub user_vault_data:Account<'info,UserVaultData>,

    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>
}



impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, deposit_amount:u64, seeds:u64,bumps:&DepositBumps)->Result<()> {

        require!(self.user_ata.amount >= deposit_amount , VaultError::InsufficientFunds);

        let transfer_acc = TransferChecked {
            from: self.user_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.vault.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let transfer_ctx = CpiContext::new(cpi_program, transfer_acc);

        transfer_checked(transfer_ctx, deposit_amount, self.mint.decimals);

        self.user_vault_data.set_inner(
            UserVaultData { 
                user: *self.user.key,
                mint: self.mint.key(),
                deposited: deposit_amount, 
                bump: bumps.user_vault_data
             }
        );

        Ok(())
    }
}