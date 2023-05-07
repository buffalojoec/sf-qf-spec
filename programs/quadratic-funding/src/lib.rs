use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod util;

pub use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod quadratic_funding {
    use super::*;

    /// Creates a new funding round as a `Pool`.
    pub fn create_pool(
        ctx: Context<CreatePool>,
        pool_id: u64,
        name: String,
        start: u64,
        end: u64,
    ) -> Result<()> {
        instructions::create_pool(ctx, pool_id, name, start, end)
    }

    /// Creates a `FundingSource` for a funder.
    pub fn create_source(ctx: Context<CreateFundingSource>, name: String) -> Result<()> {
        instructions::create_source(ctx, name)
    }

    /// Funds a pool from a `Source` with SOL.
    pub fn fund_pool(ctx: Context<FundPool>, pool_id: u64, amount: u64) -> Result<()> {
        instructions::fund_pool(ctx, pool_id, amount)
    }

    /// Funds a pool from a `Source` with an SPL Token.
    pub fn fund_pool_spl(ctx: Context<FundPoolSpl>, pool_id: u64, amount: u64) -> Result<()> {
        instructions::fund_pool_spl(ctx, pool_id, amount)
    }

    /// Creates a `Project` for a project.
    pub fn create_project(
        ctx: Context<CreateProject>,
        project_id: u64,
        name: String,
    ) -> Result<()> {
        instructions::create_project(ctx, project_id, name)
    }

    /// Submits a SOL vote.
    pub fn vote(ctx: Context<Vote>, pool_id: u64, project_id: u64, amount: u64) -> Result<()> {
        instructions::vote(ctx, pool_id, project_id, amount)
    }

    /// Submits a vote with an SPL Token.
    pub fn vote_spl(
        ctx: Context<VoteSpl>,
        pool_id: u64,
        project_id: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::vote_spl(ctx, pool_id, project_id, amount)
    }

    /// Closes a funding round and issues all payments to all projects.
    pub fn close_pool(ctx: Context<ClosePool>, pool_id: u64) -> Result<()> {
        instructions::close_pool(ctx, pool_id)
    }
}
