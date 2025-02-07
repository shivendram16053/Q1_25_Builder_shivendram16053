use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]    //if doing manually remove this
pub struct Marketplace{
  pub admin:Pubkey,
  pub fee:u16,
  pub bump:u8,
  pub treasury_bump:u8,
  pub rewards_bump:u8,
  #[max_len(36)]  //if doing manually remove this
  pub name:String,
}

// the below code is for defining the space manually for the accounts stored in the pda

// impl Space for Marketplace{
//     const INIT_SPACE: usize = 8+32+1+1+1+1+32;
// }