use crate::state::counter::Counter;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

pub fn handler(ctx: Context<Decrement>) -> Result<()> {
    ctx.accounts.counter.count = ctx.accounts.counter.count.checked_sub(1).unwrap();
    msg!("Counter decremented to {}", ctx.accounts.counter.count);
    Ok(())
}
