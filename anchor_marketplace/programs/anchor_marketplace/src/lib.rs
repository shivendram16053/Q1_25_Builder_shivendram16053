use anchor_lang::prelude::*;

declare_id!("GcD5SSqHnRK4pufmVmLYkwa9ZfTKTvfVjro8BUUkYMjQ");
mod context;
mod state;
mod error;
use context::*;

#[program]
pub mod anchor_marketplace {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>,name:String,fee:u16) ->Result<()>{
        let _ = ctx.accounts.initialize(name,fee,ctx.bumps);
        Ok(())
    }
    pub fn listing(ctx: Context<List>,price:u64,) -> Result<()> {
        let _ = ctx.accounts.create_listing(price, &ctx.bumps);
        ctx.accounts.deposit_nft()
    }
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist_nft()
    }
    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        let _ = ctx.accounts.send_sol();
        let _ = ctx.accounts.send_nft();
        let _ = ctx.accounts.close_mint_vault();
        Ok(())
    }
}