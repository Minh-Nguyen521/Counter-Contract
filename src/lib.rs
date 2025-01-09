use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw_storage_plus::Item;
#[cfg(test)]
mod test;

const COUNTER: Item<i32> = Item::new("counter");

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
}

#[cw_serde]
pub enum QueryMsg {
    GetCounter {},
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &0)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment {} => increment(deps, info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCounter {} => to_json_binary(&read_counter(deps)?),
    }
}

pub fn increment(deps: DepsMut, _info: MessageInfo) -> StdResult<Response> {
    let mut counter = COUNTER.load(deps.storage)?;
    counter += 1;
    COUNTER.save(deps.storage, &counter)?;
    Ok(Response::new()
        .add_attribute("action", "increment")
        .add_attribute("counter", counter.to_string()))
}

pub fn read_counter(deps: Deps) -> StdResult<i32> {
    COUNTER.load(deps.storage)
}
