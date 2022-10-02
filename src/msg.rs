use cosmwasm_std::Uint64;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub allowed: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Note that both of these functions don't take any parameters because we set this
    // when we instantiated the contract and it'll never change.
    // A contract only ever helps one account/address.
    /// Withdraws the rewards from staking
    WithdrawRewards { validator_address: String },
    /// Delegates to a validator
    /// Note: for the hackathon this is hardcoded to Juno
    DelegateRewards {
        validator_address: String,
        amount: Uint64,
        denom: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the address that is associated to this contract.
    /// The address returned is the one that granted authz access to
    /// claim rewards and delegate.
    Granter {},
}
