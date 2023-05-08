use cosmwasm_std::{
    log, to_binary, Api, Env, Extern, HandleResponse, HandleResult, InitResponse, InitResult,
    Querier, QueryResult, ReadonlyStorage, StdError, Storage,
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

use crate::msg::{HandleMsg, InitMsg, QueryMsg, ReadResponse};
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
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: HandleMsg,
) -> HandleResult {
    let mut store = PrefixedStorage::new(PREFIX_TEST, &mut deps.storage);
    // remove it from storage
    store.remove(TEST_KEY);

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("status", "success")],
        data: None,
    })
}

/////////////////////////////////////// Query /////////////////////////////////////
/// Returns QueryResult
///
/// # Arguments
///
/// * `deps` - reference to Extern containing all the contract's external dependencies
/// * `msg` - QueryMsg passed in with the query call
pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    _msg: QueryMsg,
) -> QueryResult {
    let store = ReadonlyPrefixedStorage::new(PREFIX_TEST, &deps.storage);

    if let Some(raw) = store.get(TEST_KEY) {
        let val: bool = bincode2::deserialize(&raw)
            .map_err(|_| StdError::generic_err("deserialization error"))?;
        to_binary(&ReadResponse { val })
    } else {
        Err(StdError::generic_err("This key has been removed"))
    }
}
