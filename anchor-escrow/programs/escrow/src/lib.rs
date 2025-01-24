use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("2zG7ZNMUyYNQ61hsKt99mmAY3Em3e2VujXgAyRCjbedk");


pub mod escrow {

    use super::*;
    pub fn initialize(ctx:Context<Make>,seed:u64,recieve_amount:u64,bump:u8) -> Result<()>{
        ctx.accounts.init_escrow(seed, recieve_amount, bump)?;
        Ok(())
    }

    pub fn deposit(ctx:Context<Make>,deposit:u64)->Result<()>{
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn send_and_withdraw(ctx:Context<Take>) -> Result<()>{
        ctx.accounts.send_and_withdraw()?;
        Ok(())
    }

    pub fn close(ctx:Context<Take>) ->Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }

    pub fn refund_and_close(ctx:Context<Refund>) -> Result<()>{
        ctx.accounts.refund_and_close()?;
        Ok(())
    }
}

