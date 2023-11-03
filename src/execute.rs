use cosmwasm_std::{
    BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response,
};

use crate::contract::DENOM;
use crate::hasher::{generate_proof_as_string};
use crate::msg::{AddMsg, ProveMsg};
use crate::state::{CONFIG, Secret, SECRETS};
use crate::error::ContractError;

pub fn execute_prove(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ProveMsg,
) -> Result<Response, ContractError> {
    // Load hash puzzle
    let mut hash_puzzle = SECRETS.load(deps.storage, &msg.id)?;
    if msg.depth >= hash_puzzle.depth {
        return Err(ContractError::InvalidInput {});
    }
    if hash_puzzle.claimed {
        return Err(ContractError::Claimed {});
    }

    // Verify proof
    let depth: u32 = hash_puzzle.depth - msg.depth;
    let res: String = generate_proof_as_string(depth, msg.proof.clone()).unwrap();
    if res != hash_puzzle.secret {
        return Err(ContractError::Unauthorized {});
    }

    // Clean up storage (hash_puzzle is solved)
    // Remove legacy secret and re-save
    hash_puzzle.claimed = true;
    SECRETS.remove(deps.storage, &msg.id);
    SECRETS.save(deps.storage, &msg.id, &hash_puzzle)?;

    let mut resp = Response::new()
        .add_attribute("action", "execute_prove")
        .add_attribute("secret", hash_puzzle.id)
        .add_attribute("prover", info.sender.to_string())
        .add_attribute("proof", msg.proof);

    // Release rewards (if any)
    let rewards = hash_puzzle.rewards;
    if let Some(rewards) = rewards {
        let rewards_msg = BankMsg::Send {
            to_address: info.sender.into(),
            amount: ([Coin {
                denom: DENOM.into(),
                amount: rewards,
            }])
            .to_vec(),
        };
        let bank_transfer: CosmosMsg = cosmwasm_std::CosmosMsg::Bank(rewards_msg);
        resp = resp.clone().add_message(bank_transfer);
    }

    Ok(resp)
}

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
        amount: msg.rewards.unwrap(),
    };
    check_sent_required_payment_exact(&info.funds, Some(required_payment))?;

    let secret = Secret {
        id: msg.id.clone(), 
        secret: msg.secret,
        depth: msg.depth,
        rewards: msg.rewards,
        claimed: false,
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