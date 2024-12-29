use alloc::boxed::Box;
use anchor_lang::{Accounts, Result};
use anchor_lang::context::Context;

use crate::state::PresaleInfo;
use crate::state::UserInfo;
use crate::constants::{PRESALE_SEED, USER_SEED};
use crate::state::presale_info::PresaleInfo;

pub fn buy_token(
    ctx: Context<BuyToken>,
    token_amount: u64,
) -> Result<()> {


}

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(
        mut,
        seeds = [PRESALE_SEED],
        bump
    )]

    pub presale_info: Box<Accounts<'info, PresaleInfo>>
}