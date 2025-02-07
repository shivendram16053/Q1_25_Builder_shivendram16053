use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface,TransferChecked,transfer_checked};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Delist<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,

    #[account(
        seeds = [b"marketplace",name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace : Account<'info,Marketplace>,
    pub maker_mint :InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub maker_mint_ata :InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        close =maker,
        seeds =[b"listing",marketplace.key().as_ref()],
        bump = marketplace.bump
    )]
    pub listing : Account<'info,Listing>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing
    )]
    pub vault : InterfaceAccount<'info,TokenAccount>,

    pub token_program : Interface<'info,TokenInterface>,
    pub system_program : Program<'info,System>,

}


impl<'info> Delist<'info>{
    pub fn delist_nft(&mut self) -> Result<()>{
       let cpi_program = self.token_program.to_account_info();

       let cpi_account = TransferChecked{
        from:self.vault.to_account_info(),
        to:self.maker_mint_ata.to_account_info(),
        mint:self.maker_mint.to_account_info(),
        authority:self.maker.to_account_info(),
       };

       let binding = self.marketplace.key();
       let seeds = &[b"listing", binding.as_ref(), &[self.listing.bump]];
       let signer_seeds = &[&seeds[..]];

       let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

       transfer_checked(cpi_ctx,1 , self.maker_mint.decimals)?;
    
       Ok(())
    }
}