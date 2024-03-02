use alloy_primitives::{TxHash, U256};
use hdi::prelude::*;
#[derive(Serialize, Deserialize, Debug, Clone, SerializedBytes)]
pub struct Deposit {
    pub amount: U256,
    pub transaction_hash: TxHash,
}

pub fn validate_create_link_sponsor_to_grant_pool(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let author_pubkey = base_address
        .into_agent_pub_key()
        .ok_or(wasm_error!("Base address must be an AgentPubKey"))?;
    if action.author != author_pubkey {
        return Ok(ValidateCallbackResult::Invalid(
            "Author not valid, can only add yourself as a sponsor".to_string(),
        ));
    }

    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "No action hash associated with link"
            ))))?;
    let record = must_get_valid_record(action_hash)?;
    let _grant_pool: crate::GrantPool = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;

    // check the link tag contains a Deposit
    let coupon_serialized_bytes = SerializedBytes::try_from(UnsafeBytes::from(tag.0))?;
    let deposit: Deposit = coupon_serialized_bytes.try_into().map_err(|_| {
        wasm_error!(WasmErrorInner::Guest(
            "Failed to deserialize link tag to Deposit".into()
        ))
    })?;

    // check deposit amount is not 0
    if deposit.amount == U256::from(0) {
        return Ok(ValidateCallbackResult::Invalid(
            "amount cannot be zero".to_string(),
        ));
    }

    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_sponsor_to_grant_pool(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "SponsorToGrantPools links cannot be deleted",
    )))
}
pub fn validate_create_link_grant_pool_to_sponsor(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = base_address
        .into_action_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "No action hash associated with link"
        ))))?;
    let record = must_get_valid_record(action_hash)?;
    let _grant_pool: crate::GrantPool = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_grant_pool_to_sponsor(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "GrantPoolToSponsors links cannot be deleted",
    )))
}
