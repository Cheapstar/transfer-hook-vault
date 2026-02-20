#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

mod instructions;
mod state;
mod error;
mod constant;
use instructions::*;

declare_id!("CH6Dm39gnKnBKg2734Ns6j3Qngdz5mkRvHaWn4AfZdtT");

#[program]
pub mod transfer_hook_vault {

    use super::*;

    pub fn init(ctx:Context<InitializeVault>,seeds:u64)->Result<()>{
        ctx.accounts.init_vault(seeds, &ctx.bumps);
        Ok(())
        
    }
    pub fn deposit(ctx:Context<Deposit>,deposit_amount:u64,seeds:u64)->Result<()>{
        ctx.accounts.deposit(deposit_amount, seeds, &ctx.bumps);
        Ok(())
    }
    pub fn withdraw(ctx:Context<WithDraw>,withdraw_amount:u64,seeds:u64)->Result<()>{
        ctx.accounts.withdraw(withdraw_amount, seeds, &ctx.bumps);
        Ok(())
    }
}

