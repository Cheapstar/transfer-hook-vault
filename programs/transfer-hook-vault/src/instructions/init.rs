use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_interface::{Mint, TokenAccount}};
use crate::{ID, state::vault::Vault};


#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin:Signer<'info>,

    #[account(
        init,
        payer=admin,
        seeds = [b"vault",seeds.to_le_bytes().as_ref()],
        space = 8 + Vault::INIT_SPACE,
        bump

    )]
    pub vault:Account<'info,Vault>,

    pub mint:InterfaceAccount<'info,Mint>,
    #[account(
        init, 
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = vault
    )]
    pub vault_ata:InterfaceAccount<'info,TokenAccount>,

    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Program<'info,Token>,
    pub system_program:Program<'info,System>
}


impl<'info> Initialize<'info> {
    pub fn init_vault(&mut self,seeds:u64,bumps:&InitializeBumps)->Result<()>{
        self.vault.set_inner(
            Vault {
                admin: *self.admin.key,
                mint: self.mint.key(),
                amount: 0,
                seeds:seeds,
                bump: bumps.vault,
            }
        );

        Ok(())
    }
}