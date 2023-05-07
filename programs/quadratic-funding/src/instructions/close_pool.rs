use anchor_lang::prelude::*;

use crate::error::ProtocolError;
use crate::state::pool::*;
use crate::util::{SOL_USD_PRICE_FEED_ID, USDC_USD_PRICE_FEED_ID, to_pubkey};

/// Closes a funding round and issues all payments to all projects.
pub fn close_pool(
    ctx: Context<ClosePool>,
    _pool_id: u64,
) -> Result<()> {
    ctx.accounts.pool.close_and_issue_payments(
        ctx.accounts.pyth_sol_usd.to_account_info(),
        ctx.accounts.pyth_usdc_usd.to_account_info(),
        ctx.remaining_accounts,
    )
}

#[derive(Accounts)]
#[instruction(
    pool_id: u64,
)]
pub struct ClosePool<'info> {
    /// CHECK: Pyth will check this
    #[account(
        address = to_pubkey(SOL_USD_PRICE_FEED_ID)
            @ ProtocolError::PythAccountInvalid
    )]
    pub pyth_sol_usd: UncheckedAccount<'info>,
    /// CHECK: Pyth will check this
    #[account(
        address = to_pubkey(USDC_USD_PRICE_FEED_ID)
            @ ProtocolError::PythAccountInvalid
    )]
    pub pyth_usdc_usd: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            Pool::SEED_PREFIX.as_bytes(),
            pool_id.to_le_bytes().as_ref(),
        ], 
        bump = pool.bump, 
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}