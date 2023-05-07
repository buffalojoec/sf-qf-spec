use anchor_lang::prelude::*;

use crate::state::pool::*;

/// Closes a funding round and issues all payments to all projects.
pub fn close_pool(
    ctx: Context<ClosePool>,
    _pool_id: u64,
) -> Result<()> {
    ctx.accounts.pool.close_and_issue_payments()
}

#[derive(Accounts)]
#[instruction(
    pool_id: u64,
)]
pub struct ClosePool<'info> {
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