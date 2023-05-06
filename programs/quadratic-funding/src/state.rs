use anchor_lang::prelude::*;

#[account]
#[derive(default)]
pub struct Pool<'info> {
    pub name: String,
    pub description: String,
    pub author: Pubkey,
    pub end_time: i32,
    pub initial_amount: u64,
    pub total_contributors: u32,
    pub active: PoolState,
    pub bump: u8,
}

impl Pool {
    //Will need to update this later
    pub const SPACE: usize = 8 + 32 + 32 + 256;

    pub fn new(
        name: String,
        description: String,
        author: Pubkey,
        end_time: i32,
        initial_amount: u64,
    ) -> Self {
        Pool {
            name,
            description,
            author,
            end_time,
            initial_amount,
            total_contributors = 0,
            ..Default::default()
        }
    }

    pub fn close_pool(&mut self) -> Result<()> {
        self.is_active()?;
        self.active = PoolState::Closed;
        Ok(())
    }

    pub fn is_active(&self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        msg!("now: {}", current_time);
        msg!("End Time: {}", self.end_time);
        if current_time > self.end_time {
            return err!(PoolError::EndDatePassed)
        }
        match self.active {
            PoolState::Active => Ok(()),
            PoolState::Distributed => err!(PoolError::ReleasedFunds),
            PoolState::Closed => err!(PoolError::ClosedPool),
        }
    }
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PoolState {
    Active,
    Distributed,
    Closed,
}

impl Default for FundingState {
    fn default() -> Self {
        FundingState::Active
    }
}