use anchor_lang::prelude::*;

use crate::WhiteList;

#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    /// CHECK: New account to remove from white list
    #[account()]
    pub removed_account: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"white_list"],
        bump
    )]
    pub white_list: Account<'info, WhiteList>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

impl<'info> RemoveFromWhitelist<'info> {
    pub fn remove_from_whitelist(&mut self) -> Result<()> {
        if self.white_list.authority != self.signer.key() {
            panic!("Only the authority can add to the white list!");
        }

        if let Some(pos) = self.white_list.white_list.iter().position(|&x| x == self.removed_account.key()) {
            self.white_list.white_list.remove(pos);
            msg!("Account removed from white list! {0}", self.removed_account.key().to_string());
            msg!("White list length! {0}", self.white_list.white_list.len());
        } else {
            msg!("Account not found in white list! {0}", self.removed_account.key().to_string());
        }

        Ok(())
    }
}