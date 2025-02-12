use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token::{Token,Mint};

use crate::state::StakeConfig;


#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub admin : Signer<'info>,

    #[account(
        init,
        payer=admin,
        space =8+StakeConfig::INIT_SPACE,
        seeds =[b"config"],
        bump
    )]
    pub config_account:Account<'info,StakeConfig>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards",config_account.key().as_ref()],
        bump,
        mint::decimals =6,
        mint::authority = config_account,
    )]
    pub rewards_mint : Account<'info,Mint>,

    pub system_program :Program<'info,System>,
    pub token_program : Program<'info,Token>

}

impl<'info> Initialize<'info> {

    pub fn initialize_config(&mut self,points_per_stake:u8,max_stake : u8,freeze_period:u32,bump:&InitializeBumps)->ProgramResult{
        self.config_account.set_inner(StakeConfig{
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump:bump.rewards_mint,
            bump : bump.config_account
        });

        Ok(())
    }
    
}