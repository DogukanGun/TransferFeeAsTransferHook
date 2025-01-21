use anchor_lang::prelude::*;
use spl_tlv_account_resolution::state::ExtraAccountMetaList;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

pub use error::*;
pub mod error;
pub mod context;
pub use context::*;

pub mod state;
pub use state::*;

declare_id!("AM12uJDtCRDa5ENz1GM3ffqz6XZm3Ea2AW12GeEdB2Ti");

#[program]
pub mod transfer_hook_whitelist {
    use super::*;

    #[interface(spl_transfer_hook_interface::initialize_extra_account_meta_list)]
    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>
    ) -> Result<()> {
        // set authority field on white_list account as payer address
        ctx.accounts.white_list.authority = ctx.accounts.payer.key();

        let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas()?;

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas
        )?;
        Ok(())
    }

    #[interface(spl_transfer_hook_interface::execute)]
    pub fn transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        ctx.accounts.transfer_hook(_amount,&ctx.bumps)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhiteList>) -> Result<()> {
        ctx.accounts.add_to_whitelist()
    }

    pub fn set_fee(ctx: Context<SetFee>,_fee:u64) -> Result<()> {
        ctx.accounts.set_fee(_fee)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>) -> Result<()> {
        ctx.accounts.remove_from_whitelist()
    }

}