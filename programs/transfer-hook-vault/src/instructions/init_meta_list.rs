use anchor_lang::{prelude::*, system_program::{CreateAccount, create_account}};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenInterface}};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::{constant::{EXTRA_ACCOUNT_META, VAULT, WHITELISTED_ENTRY}, error::VaultError, state::Vault};

#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub admin:Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        mut,
        seeds = [EXTRA_ACCOUNT_META.as_bytes(), mint.key().as_ref()], 
        bump
    )]
    pub extra_account_meta_list: AccountInfo<'info>,

    // we need this for seeds
    #[account(
        mut, 
        has_one = mint,
        has_one = admin,
        seeds = [VAULT.as_bytes(),seeds.to_le_bytes().as_ref()],
        bump
    )]
    pub vault : Account<'info,Vault>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn initialize_meta_list(&mut self,seeds:u64,bumps:&InitializeExtraAccountMetaListBumps)->Result<()>{
        let account_metas = vec![
            // user_vault_account 
              ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal { bytes: WHITELISTED_ENTRY.as_bytes().to_vec() },
                    Seed::AccountKey { index: 3 },
                    Seed::AccountKey { index: 2},
                    Seed::Literal { bytes: self.vault.seeds.to_le_bytes().to_vec() }
                ], 
                false,
                false
            ).map_err(|_| VaultError::InvalidExtraMeta)?
        ];
        // calculate account size
        let account_size = ExtraAccountMetaList::size_of(account_metas.len())
                                        .map_err(|_| VaultError::InvalidAccountSize)? as u64;
                                    
        // calculate minimum required lamports
        let lamports = Rent::get()?.minimum_balance(account_size as usize);

        let mint = self.mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            EXTRA_ACCOUNT_META.as_bytes(),
            &mint.as_ref(),
            &[bumps.extra_account_meta_list],
        ]];

        // create ExtraAccountMetaList account
        create_account(
            CpiContext::new(
                self.system_program.to_account_info(),
                CreateAccount {
                    from: self.admin.to_account_info(),
                    to: self.extra_account_meta_list.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            lamports,
            account_size,
            &crate::id(),
        )?;

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut self.extra_account_meta_list.try_borrow_mut_data()?,
            &account_metas,
        ).map_err(|_| VaultError::InvalidAccountSize)?;
        Ok(())
    }
}


