use cosmwasm_std::StdError;
use thiserror::Error;

/// Custom error type for the contract
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    /// Standard error from the CosmWasm standard library
    #[error("{0}")]
    Std(#[from] StdError),

    /// Error when the CW20 contract address has not been set
    #[error("CW20 contract address has not been set.")]
    Cw20AddressNotSet,

    /// Error when the CW20 contract address has already been set
    #[error("CW20 contract address has already been set.")]
    Cw20AddressAlreadySet,

    /// Error when the CW721 token is not owned by the contract
    #[error("Token not sent to contract.")]
    Cw721NotOwnedByContract,

    /// Error when an incorrect amount of tokens is sent to the contract
    #[error("Incorrect token amount sent to contract.")]
    IncorrectTokenAmount,
}
