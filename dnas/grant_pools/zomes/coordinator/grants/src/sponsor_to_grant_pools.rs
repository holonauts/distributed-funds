use alloy_primitives::{TxHash, U256};
use grants_integrity::*;
use hdk::prelude::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddGrantPoolForSponsorInput {
    pub base_sponsor: AgentPubKey,
    pub target_grant_pool_hash: ActionHash,
    pub amount: U256,
    pub transaction_hash: TxHash,
}
#[hdk_extern]
pub fn add_grant_pool_for_sponsor(input: AddGrantPoolForSponsorInput) -> ExternResult<()> {
    let deposit = Deposit {
        amount: input.amount,
        transaction_hash: input.transaction_hash,
    };
    let deposit_bytes = SerializedBytes::try_from(deposit)
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;

    create_link(
        input.base_sponsor.clone(),
        input.target_grant_pool_hash.clone(),
        LinkTypes::SponsorToGrantPool,
        LinkTag(deposit_bytes.bytes().clone()),
    )?;
    create_link(
        input.target_grant_pool_hash,
        input.base_sponsor,
        LinkTypes::GrantPoolToSponsor,
        LinkTag(deposit_bytes.bytes().clone()),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_grant_pools_for_sponsor(sponsor: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(sponsor, LinkTypes::SponsorToGrantPool, None)
}
#[hdk_extern]
pub fn get_sponsors_for_grant_pool(grant_pool_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(grant_pool_hash, LinkTypes::GrantPoolToSponsor, None)
}
