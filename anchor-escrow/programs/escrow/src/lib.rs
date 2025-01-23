use anchor_lang::prelude::*;

declare_id!("2zG7ZNMUyYNQ61hsKt99mmAY3Em3e2VujXgAyRCjbedk");

pub mod instructions;
pub mod state;

#[program]
pub mod escrow {
    use super::*;
    pub fn make(ctx: Context<Make>) -> Result<()>{
        Ok(())
    }
}

