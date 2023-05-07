use anchor_lang::prelude::*;

#[error_code]
pub enum ProtocolError {
    #[msg("The provided name string should be a maximum of 50 characters long")]
    NameTooLong,

    #[msg("This pool has already transferred the funds to the receiver")]
    ReleasedFunds,

    #[msg("This pool has already been cancelled")]
    PoolClosed,

    #[msg("This pool is still active")]
    PoolStillActive,

    #[msg("A pool can't be created with a start time that's passed")]
    PoolInvalidStart,

    #[msg("The pool has not begun its funding round yet")]
    PoolNotStarted,

    #[msg("The end date has already passed")]
    EndDatePassed,
}
