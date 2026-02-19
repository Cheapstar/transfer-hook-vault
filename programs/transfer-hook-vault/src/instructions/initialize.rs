use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use crate::state::vault::Vault;


#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin:Signer<'info>,

    #[account(
        init,
        payer=admin,
        seeds = [b"vault",admin.key().as_ref(),seeds.to_le_bytes().as_ref()],
        space = 8,
        bump

    )]
    pub vault:Account<'info,Vault>,

    pub mint:InterfaceAccount<'info,Mint>,

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