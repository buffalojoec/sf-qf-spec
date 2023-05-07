use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};

use crate::{state::{
    pool::*,
    project::*,
}, util::set_and_maybe_realloc};

/// Submits a vote with an SPL Token.
/// Each time a vote is cast, the QF algorithm updates the 
/// `Pool` state.
pub fn vote_spl(
    ctx: Context<VoteSpl>,
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
                Some(ctx.accounts.mint.key()), 
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
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.payer_token_account.to_account_info(),
                to: ctx.accounts.project_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        amount,
    )?;

    // Update the QF algorithm
    ctx.accounts.pool.update_shares()?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    pool_id: u64,
    project_id: u64,
    _amount: u64, // Anchor barfs if you don't have all ix args
)]
pub struct VoteSpl<'info> {
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
    pub mint: Account<'info, token::Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        token::mint = mint,
        token::authority = project,
    )]
    pub project_token_account: Account<'info, token::TokenAccount>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = payer,
    )]
    pub payer_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}