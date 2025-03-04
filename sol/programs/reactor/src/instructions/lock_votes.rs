use crate::caller_program;
use crate::state::Reactor;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions as tx_instructions;

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
    // lock & unlock must be called from CPI by guage program.
    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter();
    let instructions_account = next_account_info(remaining_accounts_iter)?;
    require_keys_eq!(*instructions_account.key, tx_instructions::ID);
    let current_ix = tx_instructions::get_instruction_relative(0, &instructions_account).unwrap();
    require_keys_eq!(current_ix.program_id, caller_program::id());

    ctx.accounts.reactor.lock_votes(amount)?;
    Ok(())
}
