use anchor_lang::prelude::*;

declare_id!("5wmLj3HSmgC7HsoGcdq2WkTM9C4RXQW4Pr6tfQ3pqAJC");

mod instructions;
mod state;
mod error;

use instructions::*;

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,authority:Option<Pubkey>,seeds:u64,fees:u16) -> Result<()> {
       ctx.accounts.initialize(authority, seeds, fees, &ctx.bumps)
    }

    pub fn deposit(ctx:Context<Deposit>,amount:u64,max_x:u64,max_y:u64)->Result<()>{
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    pub fn swap(ctx:Context<Swap>,amount:u64,is_x:bool,min:u64)->Result<()>{
        ctx.accounts.swap(amount, is_x, min)
    }

    pub fn withdraw(ctx: Context<Withdraw>,lp_amount:u64,min_x:u64,min_y:u64)->Result<()>{
        ctx.accounts.withdraw(lp_amount, min_x, min_y)
    }


}

