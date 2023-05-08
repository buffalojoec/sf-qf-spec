use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::{
    pool::*,
    source::*,
};
use crate::util::set_and_maybe_realloc;

/// Funds a pool from a `Source` with SOL.
/// Pools can be funded at any time, they don't have to be active.
/// They just have to be not closed.
pub fn fund_pool(
    ctx: Context<FundPool>,
    _pool_id: u64,
    amount: u64,
) -> Result<()> {
    // Check to make sure the pool is not closed
    ctx.accounts.pool.is_active()?;

    // Record the funder
    // First create a ticket
    let ticket = FundingTicket::new(
        ctx.accounts.source.key(),
        None,
        amount,
    );

    // Determine new account size with new ticket
    let mut pool_data: Pool = ctx.accounts.pool.clone().into_inner();
    pool_data.funders.push(ticket);
    set_and_maybe_realloc(
        &mut ctx.accounts.pool, 
        pool_data, 
        ctx.accounts.payer.to_account_info(), 
        ctx.accounts.system_program.to_account_info(),
    )?;
    
    // Perform transfer
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.pool.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    pool_id: u64,
    _amount: u64, // Anchor barfs if you don't have all ix args
)]
pub struct FundPool<'info> {
    #[account( 
        mut,
        seeds = [
            Pool::SEED_PREFIX.as_bytes(),
            pool_id.to_le_bytes().as_ref(),
        ],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account( 
        seeds = [
            FundingSource::SEED_PREFIX.as_bytes(),
            payer.key().as_ref(),
        ],
        bump = source.bump,
    )]
    pub source: Account<'info, FundingSource>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}