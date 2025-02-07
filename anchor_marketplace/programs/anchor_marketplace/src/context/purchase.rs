use std::io::Result;

use anchor_lang::{
    prelude::*, system_program::{transfer, Transfer}
};
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked,close_account}
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [b"marketplace",name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close=maker,
        seeds = [b"listing",marketplace.key().as_ref()],
        bump = marketplace.bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"treasury",marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"rewards",marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
        mint::decimals =6,
        mint::authority = marketplace,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn send_sol(&self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        let _ = transfer(cpi_context, self.listing.price);

        Ok(())
    }

    pub fn send_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let bindings = self.marketplace.key();
        let seeds = &[
            b"listing",
            bindings.as_ref(),
            &[self.marketplace.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        let _ = transfer_checked(cpi_context, 1, self.maker_mint.decimals);

        Ok(())
    }

    pub fn close_mint_vault(&mut self)->Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.maker.to_account_info(),
            authority:self.listing.to_account_info(),
        };

        let bindings = self.marketplace.key();

        let seeds =&[
            b"listing",
            bindings.as_ref(),
            &[self.marketplace.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        let _ = close_account(cpi_context);

        Ok(())

    }


}
