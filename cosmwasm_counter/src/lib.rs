use cosmwasm_std::
    HandleResponse, HandleResult, InitResponse, InitResult,
    Storage, Querier, Params, MessageInfo, DepsMut, Deps, to_binary, Binary
};


pub const KEY_COUNT: &[u8] = b"count";


pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> InitResult {
    deps.storage.set(KEY_COUNT, &0u64.to_be_bytes());
    Ok(InitResponse::default())
}


pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: HandleMsg,
) -> HandleResult {
    match msg {
        HandleMsg::Increment {} => try_increment(deps),
    }
}


pub fn try_increment<S: Storage>(deps: DepsMut<S>) -> HandleResult {
    let count = deps.storage.get(KEY_COUNT).unwrap().as_slice().read_u64::<BigEndian>().unwrap() + 1;
    deps.storage.set(KEY_COUNT, &count.to_be_bytes());
    Ok(HandleResponse::default())
}