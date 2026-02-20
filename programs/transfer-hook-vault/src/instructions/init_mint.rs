use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface,mint_to,MintTo}};

use crate::ID;



#[derive(Accounts)]
#[instruction(mint_config:MintConfig)]
pub struct InitMint<'info> {
    #[account(mut)]
    pub authority:Signer<'info>,

    #[account(
        init,
        payer = authority,
        mint::decimals = mint_config.decimals,
        mint::authority = mint_config.mint_authority,
        mint::freeze_authority = mint_config.freeze_authority,
        extensions::transfer_hook::authority = mint_config.transfer_hook_authority,
        extensions::transfer_hook::program_id = crate::id()
    )]
    pub mint : InterfaceAccount<'info,Mint>, 

    pub associated_token_program : Program<'info,AssociatedToken>,
    pub token_program : Interface<'info,TokenInterface>,
    pub system_program : Program<'info,System>
}


impl<'info> InitMint<'info> {
    pub fn init_mint(&mut self,mint_config:MintConfig)->Result<()>{
        msg!("Mint has been Initialized !! Start Minting .....");
        Ok(())
    }
}



#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(
        mut
    )]
    pub mint_authority:Signer<'info>,

    pub recipient:SystemAccount<'info>,

    #[account(
        mint::authority = mint_authority
    )]
    pub mint:InterfaceAccount<'info,Mint>,
    #[account(
        init_if_needed,
        payer = mint_authority,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_ata : InterfaceAccount<'info,TokenAccount>,

    pub associated_token_program : Program<'info,AssociatedToken>,
    pub token_program : Interface<'info,TokenInterface>,
    pub system_program : Program<'info,System>    
}


impl<'info> MintTokens<'info> {
    pub fn mint(&mut self,amount:u64){
        
        let cpi_program = self.token_program.to_account_info();
        let mint_to_acc = MintTo{
            mint: self.mint.to_account_info(),
            to: self.recipient_ata.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, mint_to_acc);
        mint_to(cpi_ctx, amount).unwrap();
    }
}


#[derive(AnchorDeserialize,AnchorSerialize)]
pub struct MintConfig {
    pub decimals:u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Pubkey,
    pub transfer_hook_authority:Pubkey
}