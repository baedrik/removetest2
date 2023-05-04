use cosmwasm_std::{
    Api, Env, Extern, HandleResponse, HandleResult, InitResponse, InitResult, Querier,
    QueryResponse, QueryResult, ReadonlyStorage, StdError, Storage,
};
use cosmwasm_storage::PrefixedStorage;

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{PREFIX_TEST, TEST_KEY};

pub const BLOCK_SIZE: usize = 256;

////////////////////////////////////// Init ///////////////////////////////////////
/// Returns InitResult
///
/// Initializes the rewind contract
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `msg` - InitMsg passed in with the instantiation message
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> InitResult {
    let mut store = PrefixedStorage::new(PREFIX_TEST, &mut deps.storage);
    // save something
    store.set(
        TEST_KEY,
        &bincode2::serialize(&true).map_err(|_| StdError::generic_err("serialization error"))?,
    );
    // test retrieving it
    if let Some(raw) = store.get(TEST_KEY) {
        let val: bool = bincode2::deserialize(&raw)
            .map_err(|_| StdError::generic_err("deserialization error"))?;
        if !val {
            return Err(StdError::generic_err("Corrupt retrieval from storage"));
        }
    } else {
        return Err(StdError::generic_err("Could not retrieve from storage"));
    }
    // remove it from storage
    store.remove(TEST_KEY);
    // test if it was removed
    if store.get(TEST_KEY).is_some() {
        return Err(StdError::generic_err("This key should have been removed!"));
    }

    Ok(InitResponse::default())
}

///////////////////////////////////// Handle //////////////////////////////////////
/// Returns HandleResult
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `msg` - HandleMsg passed in with the execute message
pub fn handle<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: HandleMsg,
) -> HandleResult {
    Ok(HandleResponse::default())
}

/////////////////////////////////////// Query /////////////////////////////////////
/// Returns QueryResult
///
/// # Arguments
///
/// * `deps` - reference to Extern containing all the contract's external dependencies
/// * `msg` - QueryMsg passed in with the query call
pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _msg: QueryMsg,
) -> QueryResult {
    Ok(QueryResponse::default())
}
