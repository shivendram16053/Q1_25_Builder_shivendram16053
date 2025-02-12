use crate::state::{StakeAccount, StakeConfig, UserAccount};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::mpl_token_metadata::instructions::{
    FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
};
use anchor_spl::metadata::{MasterEditionAccount, Metadata, MetadataAccount};
use anchor_spl::token::{approve, Approve, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,
    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint =mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"metadata",metadata_program.key().as_ref(),mint.key().as_ref()],
        seeds::program =metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"metadata",metadata_program.key().as_ref(),mint.key().as_ref(),"edition".as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds =[b"config"],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = user,
        space = 8+StakeAccount::INIT_SPACE,
        seeds= [b"stake",config_account.key().as_ref(),mint.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds=[b"user",user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum StakeError {
    #[msg("Max Staked")]
    MaxStaked,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        require!(
            self.user_account.amounts_staked < self.config_account.max_stake,
            StakeError::MaxStaked
        );

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Approve {
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?;

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.master_edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
        let bump = &[self.stake_account.bump];

        let signer_seeds: &[&[u8]] = &[
            b"stake",
            self.config_account.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref(),
            bump, // Directly passing `bump` instead of wrapping it in an extra slice
        ];
        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        )
        .invoke_signed(&[signer_seeds])?;

        self.stake_account.set_inner(StakeAccount{
            owner: self.user.key(),
            mint: self.mint.key(),
            staked_at:Clock::get().unwrap().unix_timestamp,
            bump:bumps.stake_account,
        });

        self.user_account.amounts_staked+=1;

        Ok(())
    }

}
