use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer}};

use crate::state::Config;
use constant_product_curve;



#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Deposit<'info>{
    #[account(mut)]
    pub user : Signer<'info>,
    pub mint_x: Account<'info,Mint>,
    pub mint_y: Account<'info,Mint>,

    #[account(
        mut,
        mint::authority = config,
        mint::decimals =6,
        seeds = ["lp".as_bytes(),config.key().as_ref()],
        bump
    )]
    pub mint_lp: Account<'info,Mint>,

    #[account(
        mut,
        associated_token::mint= mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Account<'info,TokenAccount>,

    #[account(
        mut,
        seeds = [b"config",seeds.to_le_bytes().as_ref()],
        bump,
    )]
    pub config: Account<'info,Config>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x_ata: Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y_ata: Account<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
    )]
    pub user_lp: Account<'info,TokenAccount>,

    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>,
    pub associated_token_program: Program<'info,AssociatedToken>,
}

impl<'info> Deposit<'info>{
    pub fn deposit(&mut self,amount:u64,max_x:u64,max_y:u64)-> Result<()>{
        let (x,y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount ==0 {
            true => (max_x,max_y),
            false => {
                let amounts = constant_product_curve::ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    6
                ).unwrap();

                (amounts.x,amounts.y)
            }
        };

        
        self.deposit_token(true, x)?;
        self.deposit_token(false, y)?;
        self.mint_lp_tokens(amount)?;



        Ok(())
    }

    pub fn deposit_token(&mut self,is_x:bool,amount:u64)->Result<()>{
        let (from,to)= match is_x {
            true => (self.user_x_ata.to_account_info(),self.vault_x.to_account_info()),
            false =>(self.user_y_ata.to_account_info(),self.vault_y.to_account_info()),            
        };

        let cpi_program =self.token_program.to_account_info();

        let cpi_account = Transfer{
            from,
            to,
            authority:self.user.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program, cpi_account);

        transfer(ctx, amount)?;

        Ok(())
    }

    pub fn mint_lp_tokens(&mut self,amount:u64)-> Result<()>{

        let cpi_program = self.token_program.to_account_info();

        let cpi_account = MintTo{
            mint:self.mint_lp.to_account_info(),
            to:self.user_lp.to_account_info(),
            authority:self.config.to_account_info(),
        };

        let seeds =&[
            &b"config"[..],
            &self.config.seed.to_be_bytes(),
            &[self.config.config_bump]
        ];

        let signer_seeds =&[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

        mint_to(cpi_context, amount)?;  
        Ok(())

    }


}