use anchor_lang::prelude::*;

#[error_code]
pub enum PoolError {
    #[msg("The provided name string should be a maximum of 50 characters long")]
    NameTooLong,

    #[msg("The provided description string should be a maximum of 250 characters long maximum.")]
    DescriptionTooLong,

    #[msg("This pool has already transferred the funds to the receiver")]
    ReleasedFunds,

    #[msg("This pool has already been cancelled")]
    PoolClosed,

    #[msg("This pool is still active")]
    PoolStillActive,

    #[msg("The end date has already passed")]
    EndDatePassed
}