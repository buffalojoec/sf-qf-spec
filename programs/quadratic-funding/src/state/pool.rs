use anchor_lang::prelude::*;
use std::collections::HashMap;

use crate::{error::ProtocolError, util::MAX_NAME_LEN};

#[account]
pub struct Pool {
    pub pool_id: u64,
    pub name: String,
    pub start: u64,
    pub end: u64,
    pub project_shares: HashMap<Pubkey, PoolShare>,
    pub funders: Vec<FundingTicket>,
    pub pool_state: PoolState,
    pub bump: u8,
}

impl Pool {
    pub const SEED_PREFIX: &'static str = "pool";

    pub const SPACE: usize = 8
        + 4                         // u64
        + 4 + MAX_NAME_LEN          // String
        + 4                         // u64
        + 4                         // u64
        + 4                         // HashMap (empty)
        + 4                         // Vec (empty)
        + 1                         // Enum (singleton)
        + 1; // u8

    pub fn new(pool_id: u64, name: String, start: u64, end: u64, bump: u8) -> Result<Self> {
        if name.as_bytes().len() > MAX_NAME_LEN {
            return Err(ProtocolError::NameTooLong.into());
        }
        let current_time = Clock::get()?.unix_timestamp as u64;
        if current_time < start {
            return Err(ProtocolError::PoolInvalidStart.into());
        }
        Ok(Self {
            pool_id,
            name,
            start,
            end,
            project_shares: HashMap::new(),
            funders: vec![],
            pool_state: PoolState::PendingStart,
            bump,
        })
    }

    pub fn close_pool(&mut self) -> Result<()> {
        self.is_active()?;
        self.pool_state = PoolState::Closed;
        Ok(())
    }

    pub fn is_active(&self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        if current_time > self.end {
            return Err(ProtocolError::EndDatePassed.into());
        }
        match self.pool_state {
            PoolState::PendingStart => Err(ProtocolError::PoolNotStarted.into()),
            PoolState::Active => Ok(()),
            PoolState::Distributed => Err(ProtocolError::ReleasedFunds.into()),
            PoolState::Closed => Err(ProtocolError::PoolClosed.into()),
        }
    }

    /// Updates all shares using the Quadratic Funding algorithm
    pub fn update_shares(&mut self) -> Result<()> {
        todo!()
    }

    /// Issues all payments according to the `project_shares`
    pub fn close_and_issue_payments(&mut self) -> Result<()> {
        todo!()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FundingTicket {
    pub source: Pubkey,
    pub mint: Option<Pubkey>,
    pub amount: u64,
}

impl FundingTicket {
    pub fn new(source: Pubkey, mint: Option<Pubkey>, amount: u64) -> Self {
        Self {
            source,
            mint,
            amount,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PoolShare {
    pub share: f32,
    pub votes: Vec<VoteTicket>,
}

impl PoolShare {
    pub fn new() -> Self {
        Self {
            share: 0.0,
            votes: vec![],
        }
    }

    pub fn new_with_vote(vote: VoteTicket) -> Self {
        Self {
            share: 0.0,
            votes: vec![vote],
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VoteTicket {
    pub payer: Pubkey,
    pub mint: Option<Pubkey>,
    pub amount: u64,
}

impl VoteTicket {
    pub fn new(payer: Pubkey, mint: Option<Pubkey>, amount: u64) -> Self {
        Self {
            payer,
            mint,
            amount,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PoolState {
    PendingStart,
    Active,
    Distributed,
    Closed,
}
