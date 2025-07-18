pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("89dQ8CxbjwTnvFAht1ZdaHjf235f3np1p3TxB9vxQw7y");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts.init_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()>{
        ctx.accounts.init_user_account(&ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;
        Ok(())
    }

}
