
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint};

use crate::state::{UserVaultAccount, Vault};
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
        space = 8 + UserVaultAccount::INIT_SPACE,
        seeds = [WHITELISTED_ENTRY.as_bytes(),user.as_ref(),mint.key().as_ref(),seeds.to_le_bytes().as_ref()],
        bump
    )]
    pub user_vault_data:Account<'info,UserVaultAccount>,
    pub system_program:Program<'info,System>
}



impl<'info> AddUser<'info> {
    pub fn add_user(&mut self, user:Pubkey, seeds:u64,bumps:&AddUserBumps)->Result<()> {

        self.user_vault_data.set_inner(
            UserVaultAccount { 
                user: user,
                mint: self.mint.key(),
                deposited: self.user_vault_data.deposited, // since anchor initializes field to zero we can use it for case like this
                seeds,
                bump: bumps.user_vault_data,
                allowed:true        // if this pda exists then we need to turn it to true
            }
        );

        self.vault.number_of_users.checked_add(1).ok_or(VaultError::Overflow)?;


        msg!("Add user {} to the whitelist", user);
        Ok(())
    }
}