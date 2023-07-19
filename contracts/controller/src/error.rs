use cosmwasm_std::{StdError, Coin};
use cw_ownable::OwnershipError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Instantiation of contract error")]
    InstantiateError {},

    #[error("Invalid funds sent. Need to send exactly {}", funds_required )]
    InvalidFunds { funds_required: Coin },

    #[error("A destination address or user id must be provided")]
    NoDestination {},

    #[error("Message is too long")]
    MessageTooLong {},

    #[error("Not enough funds to send message")]
    NotEnoughFundsForMessage {},
}
