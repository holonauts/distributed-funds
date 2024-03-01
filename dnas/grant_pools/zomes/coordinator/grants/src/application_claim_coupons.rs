use crate::sponsor_to_grant_pools::get_sponsors_for_grant_pool;
use alloy_primitives::U256;
use grants_integrity::*;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddClaimCouponForApplicationInput {
    pub base_application_hash: ActionHash,
    pub coupon: ClaimCoupon,
}
#[hdk_extern]
pub fn add_claim_coupon_for_application(
    input: AddClaimCouponForApplicationInput,
) -> ExternResult<()> {
    let coupon_bytes = SerializedBytes::try_from(input.coupon)
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;
    create_link(
        input.base_application_hash.clone(),
        // Link target is arbitrary since we're  just using a link to store the coupon in the link tag
        input.base_application_hash.clone(),
        LinkTypes::ApplicationClaimCoupons,
        LinkTag::new(coupon_bytes.bytes().clone()),
    )?;

    Ok(())
}

#[hdk_extern]
pub fn get_claim_coupons_for_application(application_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(application_hash, LinkTypes::ApplicationClaimCoupons, None)
}

#[hdk_extern]
pub fn construct_claim_context_for_application(
    application_hash: ActionHash,
) -> ExternResult<ClaimContext> {
    let application_record = get(application_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("No application found".into())),
    )?;
    let application: Application = application_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Record must contain an entry"
        ))))?;

    let grant_pool_record = get(application.grant_pool.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("No grant pool found".into())),
    )?;
    let grant_pool: GrantPool = grant_pool_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Record must contain an entry"
        ))))?;

    // calculate total deposited
    let total_deposit_amount = get_sponsors_for_grant_pool(application.grant_pool)?
        .into_iter()
        .filter_map(|l| U256::try_from_le_slice(l.tag.0.as_slice()))
        .sum();

    ClaimContext::new(
        grant_pool.flow_evm_address,
        application.evm_wallet,
        application_hash,
        application.amount,
        total_deposit_amount,
    )
}
