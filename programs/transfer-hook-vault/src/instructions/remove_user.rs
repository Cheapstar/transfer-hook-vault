
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount,TransferChecked,transfer_checked,TokenInterface};

use crate::state::{UserVaultAccount, Vault};
use crate::error::VaultError;
use crate::constant::{VAULT, WHITELISTED_ENTRY};

// all this does is transfer from user_ata to vault_ata
#[derive(Accounts)]
#[instruction(user:Pubkey,seeds:u64)]
pub struct RemoveUser<'info> {
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

    pub user_vault_data:Account<'info,UserVaultAccount>,
    pub system_program:Program<'info,System>
}



impl<'info> RemoveUser<'info> {
    pub fn remove_user(&mut self, user:Pubkey, seeds:u64,bumps:&RemoveUserBumps)->Result<()> {

        let (expected_key,bump) = Pubkey::find_program_address(
            &[WHITELISTED_ENTRY.as_bytes(),user.key().as_ref(),self.mint.key().as_ref(),self.vault.seeds.to_le_bytes().as_ref()]
                    , &crate::id());

        require_eq!(self.user_vault_data.key(),expected_key,VaultError::AccountMisMatch);
        require_eq!(self.user_vault_data.bump,bump,VaultError::BumpMisMatch);
        require_eq!(self.user_vault_data.mint,self.mint.key(),VaultError::MintMisMatch);
        require_eq!(self.user_vault_data.user,user.key(),VaultError::UserMisMatch);


        // I am trying ki if it has some deposit amount than we disallow it , else we remove it
        if self.user_vault_data.deposited > 0 {
            self.user_vault_data.allowed = false;
        }else {
            // runtime will remove the user_vault_data PDA if lamports becomes zero
            let current_balance = self.user_vault_data.get_lamports();
            self.admin.lamports.borrow_mut().checked_add(current_balance).ok_or(VaultError::Overflow)?;
            self.user_vault_data.sub_lamports(current_balance)?;
        }

        self.vault.number_of_users.checked_sub(1).ok_or(VaultError::Overflow)?;


        msg!("Add user {} to the whitelist", user);
        Ok(())
    }
}