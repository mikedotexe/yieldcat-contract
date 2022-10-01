#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    SubMsg, Uint64, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

// Version info for migration (boilerplate stuff)
const CONTRACT_NAME: &str = "crates.io:cw-yieldcat";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Reply IDs
const REPLY_REGISTER_WITH_PAYMENT: u64 = 0;
const REPLY_REGISTER_WITH_SCHOLARSHIP: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // This sets the version, imported from cw2
    // Boilerplate, don't worry about it
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // let granter = msg
    //     .granter
    //     .and_then(|addr_string| deps.api.addr_validate(addr_string.as_str()).ok())
    //     .unwrap_or(info.sender);

    let config = Config {
        granter: msg.granter.clone(),
        allowed: msg.allowed.clone(),
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("contract", CONTRACT_NAME)
        .add_attribute("method", "instantiate")
        .add_attribute("granter", msg.granter))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterWithPayment { dinner_contract } => {
            execute_register_with_payment(info, dinner_contract)
        }
        ExecuteMsg::RegisterWithScholarship { dinner_contract } => {
            execute_register_with_scholarship(info, dinner_contract)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::RegistrantNumber {} => to_binary(&Uint64::new(19)),
    }
}

pub fn execute_register_with_payment(
    info: MessageInfo,
    dinner_contract: String,
) -> Result<Response, ContractError> {
    let hi = "foo";
    // let action = CosmosMsg::Wasm(WasmMsg::Execute {
    //     contract_addr: dinner_contract,
    //     msg: to_binary(
    //         &cross_contract_dinner::msg::ExecuteMsg::RegisterWithPayment {
    //             address: info.sender,
    //         },
    //     )
    //     .unwrap(),
    //     funds: info.funds,
    // });
    // // use reply_on_success because we need to increase number of registrants in case of success
    // let sub_msg: SubMsg = SubMsg::reply_on_success(action, REPLY_REGISTER_WITH_PAYMENT);
    Ok(Response::new()
        .add_attribute("contract", "demo-totals")
        .add_attribute("method", "execute_register_with_payment"))
    //     .add_submessage(sub_msg))
}

pub fn execute_register_with_scholarship(
    info: MessageInfo,
    dinner_contract: String,
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
        .add_attribute("method", "execute_register_with_scholarship"))
}
