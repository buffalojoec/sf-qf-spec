use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod quadratic_funding {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        name: String,
        description: String,
        author: Pubkey,
        end_time: i32,
        initial_amount: u64,
    ) -> Result<()> {
        instructions::create_pool(ctx, name, description, author, end_time, initial_amount).expect("Failed to create pool.");

        Ok(())
    }
}
