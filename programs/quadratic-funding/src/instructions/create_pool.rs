use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreatePool<'info> {
    #[account(init, 
        seeds = [
            name.as_ref(),
            authority.key().as_ref()], 
            bump, 
        payer = authority, 
        space = Pool::SPACE,
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}


pub fn create_pool(
    ctx: Context<CreatePool>,
    name: String,
    description: String,
    author: Pubkey,
    end_time: i32,
    initial_amount: u64,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    if name.len() > 50 {
        return Err(PoolError::NameTooLong.into());
    }

    if description.len() > 575 {
        return Err(PoolError::DescriptionTooLong.into());
    }

    pool.set_inner(Pool::new(
        name,
        description,
        *ctx.bumps
            .get("pool")
            .expect("Should've gotten bump"),
        author,
        end_time,
        initial_amount,
    ));

    Ok(())
}