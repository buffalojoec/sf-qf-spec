use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::error::ProtocolError;
use crate::state::{
    pool::*,
    project::*,
};
use crate::util::{
    SOL_USD_PRICE_FEED_ID, 
    USDC_USD_PRICE_FEED_ID, 
    set_and_maybe_realloc, 
    to_pubkey
};

/// Submits a SOL vote.
/// Each time a vote is cast, the QF algorithm updates the 
/// `Pool` state.
pub fn vote(
    ctx: Context<Vote>,
    _pool_id: u64,
    _project_id: u64,
    amount: u64,
) -> Result<()> {
    // Check to make sure the pool is not closed
    ctx.accounts.pool.is_active()?;

    // Add the project to the shares, if it doesn't exist
    let project_key = ctx.accounts.project.key();
    if !ctx.accounts.pool.project_shares.contains_key(&project_key) {
        let mut pool_data = ctx.accounts.pool.clone().into_inner();
        pool_data.project_shares.insert(
            project_key, 
            PoolShare::new_with_vote(VoteTicket::new(
                ctx.accounts.payer.key(), 
                None, 
                amount,
            ))
        );
        set_and_maybe_realloc(
            &mut ctx.accounts.pool, 
            pool_data, 
            ctx.accounts.payer.to_account_info(), 
            ctx.accounts.system_program.to_account_info()
        )?;
    }

    // Transfer the vote to the project
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.project.to_account_info(),
            },
        ),
        amount,
    )?;

    // Update the QF algorithm
    ctx.accounts.pool.update_shares(
        ctx.accounts.pyth_sol_usd.to_account_info(),
        ctx.accounts.pyth_usdc_usd.to_account_info(),
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    pool_id: u64,
    project_id: u64,
    _amount: u64, // Anchor barfs if you don't have all ix args
)]
pub struct Vote<'info> {
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
    #[account( 
        seeds = [
            Project::SEED_PREFIX.as_bytes(),
            project_id.to_le_bytes().as_ref(),
        ],
        bump = project.bump,
    )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}