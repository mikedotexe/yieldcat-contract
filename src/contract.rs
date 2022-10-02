#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Api, Binary, Deps, DepsMut, DistributionMsg, Env, MessageInfo, Response,
    StakingMsg, StdResult, Uint64,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::CONFIG;

// Version info for migration (boilerplate stuff)
const CONTRACT_NAME: &str = "crates.io:cw-yieldcat";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Taken from the cw-plus crate's cw1-whitelist
fn map_validate(api: &dyn Api, admins: &[String]) -> StdResult<Vec<Addr>> {
    admins.iter().map(|addr| api.addr_validate(addr)).collect()
}

pub enum HackWasmDistributionMsg {
    /// This is translated to a [MsgSetWithdrawAddress](https://github.com/cosmos/cosmos-sdk/blob/v0.42.4/proto/cosmos/distribution/v1beta1/tx.proto#L29-L37).
    /// `delegator_address` is automatically filled with the current contract's address.
    SetWithdrawAddress {
        /// The `withdraw_address`
        address: String,
    },
    /// This is translated to a [[MsgWithdrawDelegatorReward](https://github.com/cosmos/cosmos-sdk/blob/v0.42.4/proto/cosmos/distribution/v1beta1/tx.proto#L42-L50).
    /// `delegator_address` is automatically filled with the current contract's address.
    WithdrawDelegatorReward {
        /// The `validator_address`
        validator: String,
    },
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    // Validate that they sent us good addresses
    config.allowed = map_validate(deps.api, &msg.allowed)?;
    config.granter = info.sender;

    // This sets the version, imported from cw2, just a normal thing to do
    // Boilerplate, don't worry about it
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("contract", CONTRACT_NAME)
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::WithdrawRewards { validator_address } => {
            execute_withdraw_rewards(deps, info, validator_address)
        }
        ExecuteMsg::DelegateRewards {
            validator_address,
            amount,
            denom,
        } => execute_delegate_rewards(deps, info, validator_address, amount, denom),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Granter {} => {
            let config = CONFIG.load(deps.storage)?;
            to_binary(&config.granter)
        }
    }
}

pub fn execute_withdraw_rewards(
    deps: DepsMut,
    info: MessageInfo,
    validator_address: String,
) -> Result<Response, ContractError> {
    // The question mark here and other places means,
    // "throw the error programmed behind the scenes if it fails"
    deps.api.addr_validate(&validator_address)?;

    let withdrawMsg = DistributionMsg::WithdrawDelegatorReward {
        validator: validator_address,
    };

    Ok(Response::new()
        .add_attribute("contract", "demo-totals")
        .add_attribute("method", "execute_withdraw_rewards")
        .add_message(withdrawMsg))
}

pub fn execute_delegate_rewards(
    deps: DepsMut,
    info: MessageInfo,
    validator_address: String,
    amount: Uint64,
    denom: String,
) -> Result<Response, ContractError> {
    // let action = CosmosMsg::Wasm(WasmMsg::Execute {
    //     contract_addr: dinner_contract,
    //     msg: to_binary(
    //         &cross_contract_dinner::msg::ExecuteMsg::RegisterWithScholarship {
    //             address: info.sender,
    //         },
    //     )
    //     .unwrap(),
    //     funds: vec![],
    // });
    // // use reply_on_success because we need to increase number of registrants in case of success
    // let sub_msg: SubMsg = SubMsg::reply_on_success(action, REPLY_REGISTER_WITH_SCHOLARSHIP);
    Ok(Response::new()
        .add_attribute("contract", "demo-totals")
        .add_attribute("method", "execute_delegate_rewards"))
}
