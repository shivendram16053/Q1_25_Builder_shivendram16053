use anchor_lang::prelude::*;

declare_id!("5wmLj3HSmgC7HsoGcdq2WkTM9C4RXQW4Pr6tfQ3pqAJC");

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
