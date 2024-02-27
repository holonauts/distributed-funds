use grants_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn create_grant_pool_outcome(grant_pool_outcome: GrantPoolOutcome) -> ExternResult<Record> {
    let grant_pool_outcome_hash =
        create_entry(&EntryTypes::GrantPoolOutcome(grant_pool_outcome.clone()))?;
    create_link(
        grant_pool_outcome.grant_pool.clone(),
        grant_pool_outcome_hash.clone(),
        LinkTypes::GrantPoolToGrantPoolOutcomes,
        (),
    )?;
    let record = get(grant_pool_outcome_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest(String::from(
            "Could not find the newly created GrantPoolOutcome"
        ))),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_grant_pool_outcome(grant_pool_outcome_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(grant_pool_outcome_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed get details response"
        )))),
    }
}
#[hdk_extern]
pub fn get_grant_pool_outcomes_for_grant_pool(
    grant_pool_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        grant_pool_hash,
        LinkTypes::GrantPoolToGrantPoolOutcomes,
        None,
    )
}
