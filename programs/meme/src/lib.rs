mod instructions;
mod state;
mod constants;
mod errors;

use anchor_lang::prelude::*;

declare_id!("3muV9L2hNgFxpMAhrAzGKk8kb8B1XV3RnNMwZYF5QnPP");

#[program]
pub mod meme {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
