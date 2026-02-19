use anchor_lang::prelude::*;

mod instructions;
mod state;
use instructions::*;
use state::*;
declare_id!("CH6Dm39gnKnBKg2734Ns6j3Qngdz5mkRvHaWn4AfZdtT");

#[program]
pub mod transfer_hook_vault {
    use super::*;

    pub fn init(ctx:Context<Initialize>)->Result<()>{
        Ok(())
    }
    pub fn deposit(ctx:Context<Deposit>)->Result<()>{
        Ok(())
    }
    pub fn withdraw(ctx:Context<WithDraw>)->Result<()>{
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
