use cosmwasm_std::{
    Coin, DepsMut, Env, MessageInfo, Response,
};

use crate::contract::DENOM;
use crate::msg::{AddMsg};
use crate::state::{CONFIG, Secret, SECRETS};
use crate::error::ContractError;

pub fn execute_add_secret(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: AddMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // Tx must cover rewards payment
    let required_payment = Coin {
        denom: DENOM.to_string(),
        amount: msg.rewards.clone().unwrap(),
    };
    check_sent_required_payment_exact(&info.funds, Some(required_payment))?;

    let secret = Secret {
        id: msg.id.clone(), 
        secret: msg.secret,
        depth: msg.depth,
        rewards: msg.rewards,
    };
    SECRETS.update(deps.storage, &msg.id, |existing| match existing {
        None => Ok(secret.clone()),
        Some(_) => Err(ContractError::InvalidInput {}),
    })?;

    Ok(Response::new()
        .add_attribute("action", "add_secret")
        .add_attribute("secret", secret.id))
}

// Avoid unreachable left-over contract balances
pub fn check_sent_required_payment_exact(
    sent: &[Coin],
    required: Option<Coin>,
) -> Result<(), ContractError> {
    if let Some(required_coin) = required {
        let required_amount = required_coin.amount.u128();
        if required_amount > 0 {
            let sent_exact_funds = sent.iter().any(|coin| {
                // check if a given sent coin matches denom
                // and has sent exactly the required amount
                coin.denom == required_coin.denom && coin.amount.u128() == required_amount
            });

            if sent_exact_funds {
                return Ok(());
            } else {
                return Err(ContractError::ExactFunds {});
            }
        }
    }
    Ok(())
}