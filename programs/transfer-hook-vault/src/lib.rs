#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;
mod instructions;
mod state;
mod error;
mod constant;
use instructions::*;

declare_id!("CH6Dm39gnKnBKg2734Ns6j3Qngdz5mkRvHaWn4AfZdtT");

#[program]
pub mod transfer_hook_vault {

    use super::*;

    pub fn init_vault(ctx:Context<InitializeVault>,seeds:u64)->Result<()>{
        ctx.accounts.init_vault(seeds, &ctx.bumps)?;
        Ok(())
        
    }
    pub fn deposit(ctx:Context<Deposit>,deposit_amount:u64,seeds:u64)->Result<()>{
        ctx.accounts.deposit(deposit_amount, seeds)?;
        Ok(())
    }
    pub fn withdraw(ctx:Context<WithDraw>,withdraw_amount:u64,seeds:u64)->Result<()>{
        ctx.accounts.withdraw(withdraw_amount, seeds)?;
        Ok(())
    }

    pub fn init_mint(ctx:Context<InitMint>,mint_config:MintConfig)->Result<()> {
        ctx.accounts.init_mint(mint_config)?;
        Ok(())
    }
    pub fn mint_tokens(ctx:Context<MintTokens>,amount:u64)->Result<()> {
        ctx.accounts.mint(amount)?;
        Ok(())
    }
    pub fn init_meta_list(ctx:Context<InitializeExtraAccountMetaList>,seeds:u64)->Result<()> {
        ctx.accounts.initialize_meta_list(seeds,&ctx.bumps)?;
        Ok(())
    }
    pub fn add_to_whitelist(ctx:Context<AddUser>,user:Pubkey,seeds:u64)->Result<()> {
        ctx.accounts.add_user(user,seeds,&ctx.bumps)?;
        Ok(())
    }
    pub fn remove_from_whitelist(ctx:Context<RemoveUser>,user:Pubkey,seeds:u64)->Result<()> {
        ctx.accounts.remove_user(user,seeds,&ctx.bumps)?;
        Ok(())
    }
    
    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx:Context<TransferHook>,amount:u64)->Result<()>{
        ctx.accounts.transfer_hook(amount)?;
        Ok(())
    }
}

