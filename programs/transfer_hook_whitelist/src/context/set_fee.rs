use anchor_lang::prelude::*;

use crate::WhiteList;

#[derive(Accounts)]
pub struct SetFee<'info>{
    #[account(
        mut,
        seeds = [b"white_list"],
        bump
    )]
    pub white_list: Account<'info, WhiteList>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

impl<'info> SetFee<'info> {
    pub fn set_fee(&mut self,_fee:u64) -> Result<()> {
        self.white_list.fee = _fee;
        Ok(())
    }
}