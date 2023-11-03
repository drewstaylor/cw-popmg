use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    
    #[error("Invalid input")]
    InvalidInput {},

    #[error("Must send exactly the required funds")]
    ExactFunds {},

    #[error("Hash puzzle already solved, nothing to claim")]
    Claimed {},
}
