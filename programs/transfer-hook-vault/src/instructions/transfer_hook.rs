use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022::spl_token_2022::{extension::{BaseStateWithExtensions, PodStateWithExtensionsMut, transfer_hook::TransferHookAccount}, pod::PodAccount}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{constant::{EXTRA_ACCOUNT_META, WHITELISTED_ENTRY}, state::UserVaultAccount,error::VaultError};


/// should this Transfer Hook only work when we deposit or withdraw from vault or for any transfer
#[derive(Accounts)]
pub struct TransferHook<'info> {
    // pehle 4 accounts will be the same in the same order
    #[account(
        token::mint = mint, 
        token::authority = owner,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account,
    #[account(
        seeds = [EXTRA_ACCOUNT_META.as_bytes(), mint.key().as_ref()], 
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    /// we need whitelist PDA, we will check it manually
    /// CHECK: it needs seeds , which we don't have here
    pub user_vault:UncheckedAccount<'info>
}

impl<'info> TransferHook<'info> {
    pub fn transfer_hook(&mut self,amount:u64)->Result<()>{
        self.check_is_transferring()?;

        let user_vault_account_info = &self.user_vault.to_account_info();

        let data_ref = user_vault_account_info.try_borrow_data()?;
        let mut data_slice: &[u8] = &data_ref;

        let user_vault_data = UserVaultAccount::try_deserialize(&mut data_slice)?;



        require_eq!(self.owner.key(),user_vault_data.user,VaultError::OwnerMisMatch);
        require!(user_vault_data.allowed,VaultError::BlackListed);
        Ok(())
    }

    fn check_is_transferring(&self) -> Result<()> {
        let source_token_info = self.source_token.to_account_info();
        let mut account_data = source_token_info.try_borrow_mut_data()?;
        let account = PodStateWithExtensionsMut::<PodAccount>::unpack(&mut account_data)?;
        let transfer_hook = account.get_extension::<TransferHookAccount>()?;

        if !bool::from(transfer_hook.transferring) {
            return err!(anchor_lang::error::ErrorCode::AccountNotInitialized);
        }

        Ok(())
    }
}