use alloy_primitives::Address;
use hdi::prelude::*;

pub fn validate_create_link_agent_to_evm_wallet(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let evm_address = String::from_utf8(target_address.into_inner()).map_err(|e| {
        wasm_error!(WasmErrorInner::Guest(format!(
            "Error converting target address to string: {:?}",
            e
        )))
    })?;
    if !AgentPubKey::try_from(base_address).is_ok() {
        return Ok(ValidateCallbackResult::Invalid(
            "Can only link wallet from an AgentPubKey".to_string(),
        ));
    }
    if !Address::parse_checksummed(&evm_address, None).is_ok() {
        return Ok(ValidateCallbackResult::Invalid(
            "Can only link to valid evm address".to_string(),
        ));
    }

    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_agent_to_evm_wallet(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
