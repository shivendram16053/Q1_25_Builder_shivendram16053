use anchor_lang::prelude::*;

declare_id!("FS9dQ6N8vGMGeAG942ZUr13xycwggE3CaDgWzuN6siBj");

mod state;
mod instructions;

use instructions::*;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        let _ = ctx.accounts.initialize_config(points_per_stake,max_stake,freeze_period,&ctx.bumps);

        Ok(())        
    }

    pub fn registed_user(ctx: Context<InitializeUser>, points: u32, amount_staked: u8) -> Result<()> {
        ctx.accounts.initialize_user_account(points,amount_staked,&ctx.bumps);

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        let _ = ctx.accounts.stake(&ctx.bumps);

        Ok(())
    }
}
