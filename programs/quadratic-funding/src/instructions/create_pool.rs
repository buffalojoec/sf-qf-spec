use anchor_lang::prelude::*;

use crate::state::pool::*;

/// Creates a new funding round as a `Pool`.
/// Initializes the new pool with no funding,
/// only details about the pool/round itself.
pub fn create_pool(
    ctx: Context<CreatePool>,
    pool_id: u64,
    name: String,
    start: u64,
    end: u64,
) -> Result<()> {
    ctx.accounts.pool.set_inner(
        Pool::new(
            pool_id,
            name,
            start,
            end,
            *ctx.bumps
                .get("pool")
                .expect("Failed to derive bump for `pool`"),
        )?
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    pool_id: u64,
    _name: String,
    _start: u64,
    _end: u64, // Anchor barfs if you don't have all ix args
)]
pub struct CreatePool<'info> {
    #[account(
        init, 
        space = Pool::SPACE,
        payer = payer, 
        seeds = [
            Pool::SEED_PREFIX.as_bytes(),
            pool_id.to_le_bytes().as_ref(),
        ], 
        bump, 
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}