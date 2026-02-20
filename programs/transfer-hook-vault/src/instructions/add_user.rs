
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount,TransferChecked,transfer_checked,TokenInterface};

use crate::state::{UserVaultData, Vault};
use crate::error::VaultError;
use crate::constant::{VAULT, WHITELISTED_ENTRY};

// all this does is transfer from user_ata to vault_ata
#[derive(Accounts)]
#[instruction(user:Pubkey,seeds:u64)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub admin:Signer<'info>,

    pub mint:InterfaceAccount<'info,Mint>,

    // I am adding this to verify that vault has been initialized before this 
    #[account(
        mut, 
        has_one = mint,
        has_one = admin,
        seeds = [VAULT.as_bytes(),seeds.to_le_bytes().as_ref()],
        bump
    )]
    pub vault : Account<'info,Vault>,

    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + UserVaultData::INIT_SPACE,
        seeds = [WHITELISTED_ENTRY.as_bytes(),user.as_ref(),seeds.to_be_bytes().as_ref()],
        bump
    )]
    pub user_vault_data:Account<'info,UserVaultData>,
    pub system_program:Program<'info,System>
}



impl<'info> AddUser<'info> {
    pub fn add_user(&mut self, user:Pubkey, seeds:u64,bumps:&AddUserBumps)->Result<()> {

        self.user_vault_data.set_inner(
            UserVaultData { 
                user: user,
                mint: self.mint.key(),
                deposited: 0, 
                seeds,
                bump: bumps.user_vault_data,
                allowed:true
             }
        );

        self.vault.number_of_users.checked_add(1).ok_or(VaultError::Overflow)?;


        msg!("Add user {} to the whitelist", user);
        Ok(())
    }
}