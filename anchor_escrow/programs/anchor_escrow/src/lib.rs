use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;
declare_id!("CPVezueWeHyWbwwoKAy8TT1DpnKvdyDTjRWmqEDAA2jv");

#[program]
pub mod anchor_escrow {
    use super::*;
    pub fn initialize(ctx:Context<Make>,seed:u64,recieve_amount:u64) -> Result<()>{
        ctx.accounts.init_escrow(seed, recieve_amount,&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx:Context<Make>,deposit:u64)->Result<()>{
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn send_to_vault(ctx:Context<Take>) -> Result<()>{
        ctx.accounts.send_to_vault()?;
        Ok(())
    }

    pub fn withdraw_and_close(ctx:Context<Take>) ->Result<()> {
        ctx.accounts.withdraw_and_close()?;
        Ok(())
    }

    pub fn refund_and_close(ctx:Context<Refund>) -> Result<()>{
        ctx.accounts.refund_and_close()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
