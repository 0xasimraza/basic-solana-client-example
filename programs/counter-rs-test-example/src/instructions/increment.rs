use crate::state::counter::Counter;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

pub fn handler(ctx: Context<Increment>) -> Result<()> {
    ctx.accounts.counter.count = ctx.accounts.counter.count.checked_add(1).unwrap();
    msg!("Counter incremented to {}", ctx.accounts.counter.count);
    Ok(())
}
