pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AQGaUitJF6dcuM4izL7QKTCfaNDsEFwFx2JoHYDQwd6p");

#[program]
pub mod counter_rs_test_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        increment::handler(ctx)
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        decrement::handler(ctx)
    }
}
