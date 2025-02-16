use std::u64;

use anchor_lang::prelude::*;
use anchor_spl::token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer};
use constant_product_curve::ConstantProduct;

use crate::state::Config;


#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub user :Signer<'info>,

    pub mint_x:Account<'info,Mint>,
    pub mint_y:Account<'info,Mint>,

    #[account(
        mut,
        mint::authority=config,
        mint::decimals=6,
        seeds = ["lp".as_bytes(),config.key().as_ref()],
        bump 
    )]
    pub mint_lp:Account<'info,Mint>,

    #[account(
        mut,
        associated_token::mint= mint_x,
        associated_token::authority=user,
    )]
    pub user_x_ata: Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority=user,
    )]
    pub user_y_ata: Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x:Account<'info,TokenAccount>,

    #[account(
        mut,
        seeds=[b"config",seeds.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config:Account<'info,Config>,

    #[account(
        mut,
        associated_token::mint = mint_lp,
        associated_token::authority=user,
    )]
    pub user_lp : Account<'info,TokenAccount>,

    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>
}

impl<'info> Withdraw<'info> {

    pub fn withdraw(&mut self,lp_amount:u64,min_x:u64,min_y:u64)->Result<()>{

        let (x,y)= match self.vault_x.amount!=0&&self.vault_y.amount!=0&&self.mint_lp.supply!=0 {

            true => {

                let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
                    self.vault_x.amount, 
                    self.vault_y.amount,
                    self.mint_lp.supply, 
                    lp_amount, 
                    6
                ).unwrap();

                (amounts.x,amounts.y)
                

            },
            false => (min_x,min_y)
            
        };

        self.burn_lp_token(lp_amount)?;
        self.withdraw_token(true, x)?;
        self.withdraw_token(false, y)?;



        Ok(())
    }

    pub fn withdraw_token(&mut self,is_x:bool,amount:u64)->Result<()>{
        
        let (from,to)= match is_x{
            true=>(self.vault_x.to_account_info(),self.user_x_ata.to_account_info()),
            false => (self.vault_y.to_account_info(),self.user_y_ata.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from,
            to,
            authority:self.config.to_account_info()
        };

        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes()[..],
            &[self.config.config_bump]
        ];

        let signer_seeds =&[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }


    pub fn burn_lp_token(&mut self,amount:u64
        )->Result<()>{

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Burn{
            mint:self.mint_lp.to_account_info(),
            from:self.user_lp.to_account_info(),
            authority:self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        burn(cpi_ctx, amount)?;

        Ok(())
    }
    
}