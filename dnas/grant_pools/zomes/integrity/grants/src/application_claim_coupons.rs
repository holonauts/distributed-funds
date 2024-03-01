use hdi::prelude::*;

use crate::ClaimCoupon;
pub fn validate_create_link_application_claim_coupons(
    _action: CreateLink,
    base_address: AnyLinkableHash,

    // We don't actually care about the target address for this link type (it links to itself)
    _target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = base_address
        .into_action_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "No action hash associated with link"
        ))))?;
    let record = must_get_valid_record(action_hash)?;
    let application: crate::Application = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    let record = must_get_valid_record(application.grant_pool)?;
    let grant_pool: crate::GrantPool = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;

    // check the link tag contains a ClaimCoupon
    let coupon_serialized_bytes = SerializedBytes::try_from(UnsafeBytes::from(tag.0))?;
    let claim_coupon: ClaimCoupon = coupon_serialized_bytes.try_into().map_err(|_| {
        wasm_error!(WasmErrorInner::Guest(
            "Failed to deserialize link tag to ClaimCoupon".into()
        ))
    })?;

    // coupon context_packed is valid
    if claim_coupon.context.pack() != claim_coupon.context_packed {
        return Ok(ValidateCallbackResult::Invalid(
            "context_packed must be result of packing context".into(),
        ));
    }

    // coupon context_packed_hash is valid
    let expected_hash = hash_keccak256(claim_coupon.context.pack())
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;
    if expected_hash != claim_coupon.context_packed_hash {
        return Ok(ValidateCallbackResult::Invalid(
            "context packed hash must be result of packing context, then taking hash".into(),
        ));
    }

    // coupon signature is valid
    let recovered_address = claim_coupon
        .signature
        .recover_address_from_msg(expected_hash)
        .map_err(|_| {
            wasm_error!(WasmErrorInner::Guest(
                "ClaimCoupon signature verification failed".into()
            ))
        })?;
    if recovered_address != grant_pool.notary_evm_wallet {
        return Ok(ValidateCallbackResult::Invalid(
            "signer must be grant pool notary".into(),
        ));
    }

    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_application_claim_coupons(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "ApplicationClaimCoupons links cannot be deleted",
    )))
}
