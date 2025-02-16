use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::state::Config;

#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint = mint_x,
        associated_token::authority=user,
    )]
    pub user_x_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint = mint_y,
        associated_token::authority =user,
    )]
    pub user_y_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint =mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint =mint_y,
        associated_token::authority =config,
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        has_one=mint_x,
        has_one=mint_y,
        seeds = [b"config",seeds.to_le_bytes().as_ref()],
        bump=config.config_bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, amount: u64, is_x: bool, min: u64) -> Result<()> {
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.vault_x.amount,
            self.config.fees,
            None,
        )
        .unwrap();

        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let res = curve.swap(p, amount, min).unwrap();

        self.deposit_token(is_x, res.deposit).unwrap();
        self.withdraw_token(is_x, res.withdraw).unwrap();

        Ok(())
    }

    pub fn deposit_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (
                self.user_x_ata.to_account_info(),
                self.vault_x.to_account_info(),
            ),
            false => (
                self.user_y_ata.to_account_info(),
                self.vault_y.to_account_info(),
            ),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (
                self.vault_x.to_account_info(),
                self.user_x_ata.to_account_info(),
            ),
            false => (
                self.vault_y.to_account_info(),
                self.user_y_ata.to_account_info(),
            ),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.config.to_account_info(),
        };

        let seeds = &[
           &b"config"[..],
           &self.config.seed.to_le_bytes()[..],
           &[self.config.config_bump]
        ];

        let signer_seeds =&[&seeds[..]];

        let cpi_ctx=CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;


        Ok(())
    }
}
