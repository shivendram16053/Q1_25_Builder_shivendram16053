use crate::state::Escrow;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked, CloseAccount, close_account
};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub maker: SystemAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint =mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn send_and_withdraw(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_account_send = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_b.to_account_info(),
        };

        let cpi_ctx_send = CpiContext::new(cpi_program.clone(), cpi_account_send);

        transfer_checked(
            cpi_ctx_send,
            self.escrow.recieve_amount,
            self.mint_b.decimals,
        )?;

        let cpi_account_withdraw = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let maker_key = self.maker.key();
        let seeds = self.escrow.seed.to_be_bytes();
        let seeds = &[
            b"escrow",
            maker_key.as_ref(),
            seeds.as_ref(),
            &[self.escrow.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx_withdraw =
            CpiContext::new_with_signer(cpi_program.clone(), cpi_account_withdraw, signer_seeds);

        transfer_checked(cpi_ctx_withdraw, self.vault.amount, self.mint_a.decimals)?;

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_account = CloseAccount {
            account:self.vault.to_account_info(),
            destination:self.taker.to_account_info(),
            authority:self.escrow.to_account_info(),
        };

        let maker_key = self.maker.key();
        let seeds = self.escrow.seed.to_be_bytes();
        let seed = &[
            b"escrow",
            maker_key.as_ref(),
            seeds.as_ref(),
            &[self.escrow.bump],
        ];
        let signer_seeds = &[&seed[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

        close_account(cpi_context)?;

        Ok(())
    }
}
