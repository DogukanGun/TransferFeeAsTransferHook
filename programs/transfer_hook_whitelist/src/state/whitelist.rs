use anchor_lang::prelude::*;

#[account]
pub struct WhiteList {
    pub authority: Pubkey,
    pub fee: u64,
    pub white_list: Vec<Pubkey>,
}