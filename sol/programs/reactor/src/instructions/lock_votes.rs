use anchor_lang::prelude::*;

use crate::state::Reactor;

#[derive(Accounts)]
pub struct LockVotes<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner,
    )]
    pub reactor: Account<'info, Reactor>,
}

pub fn handler(ctx: Context<LockVotes>, amount: u64) -> Result<()> {
    ctx.accounts.reactor.lock_votes(amount)?;
    Ok(())
}
