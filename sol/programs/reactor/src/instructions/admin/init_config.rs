use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{errors::ReactorErrors, state::ReactorConfig, REACTOR_CONFIG_SEED};

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(
        mut,
        address = crate::admin::id() @ ReactorErrors::NotAdmin
    )]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [
            REACTOR_CONFIG_SEED.as_ref(),
        ],
        bump,
        space = 8 + ReactorConfig::LEN
    )]
    pub config: Account<'info, ReactorConfig>,

    /// Vault to hold RAY tokens that are staked by Reactor depositors
    #[account(
        init,
        seeds = [
            b"ray-vault".as_ref(),
        ],
        bump,
        token::authority = config,
        token::mint = ray_mint,
        payer = payer
    )]
    pub ray_vault: Account<'info, TokenAccount>,

    /// Hopper to hold RAY rewards that get paid out to Reactor stakers
    #[account(
        init,
        seeds = [
            b"ray-reward-hopper".as_ref(),
        ],
        bump,
        token::authority = config,
        token::mint = ray_mint,
        payer = payer
    )]
    pub ray_hopper: Account<'info, TokenAccount>,

    // This is a high-trust instruction and the ray_mint must be correct
    pub ray_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitConfig>,
    // RAY emitted per day
    ray_reward_daily_emisison: u64,

    // APR for isoRAY in basis points
    iso_ray_apr_bps: u16,
) -> Result<()> {
    let c = &mut ctx.accounts.config;

    c.bump = [ctx.bumps.config];
    c.ray_mint = ctx.accounts.ray_mint.key();
    c.ray_vault = ctx.accounts.ray_vault.key();
    c.rewards_emitted_until = Clock::get()?.unix_timestamp as u64;
    c.ray_reward_hopper = ctx.accounts.ray_hopper.key();
    c.ray_reward_daily_emission = ray_reward_daily_emisison;
    c.iso_ray_apr_bps = iso_ray_apr_bps;

    Ok(())
}
