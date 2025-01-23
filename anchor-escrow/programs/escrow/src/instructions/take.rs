use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint,TokenAccount};

#[derive(Accounts)]
pub struct Take<'info>{
    #[account(mut)]
    pub taker :Signer<'info>,
    pub maker : SystemAccount<'info>,
    pub mint_a : InterfaceAccount<'info,Mint>,
    pub mint_b : InterfaceAccount<'info,Mint>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,

    )]
    pub taker_ata_a : InterfaceAccount<'info,TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint =mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker
    )]
    pub maker_ata_b:InterfaceAccount<'info,TokenAccount>,
}
