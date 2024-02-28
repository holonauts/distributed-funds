use alloy_primitives::U256;
use grants_integrity::GrantPoolOutcome;
use grants_integrity::*;
use hdk::prelude::*;
use std::collections::BTreeMap;
#[hdk_extern]
pub fn create_grant_pool_outcome_for_grant_pool(grant_pool: ActionHash) -> ExternResult<Record> {
    let links_for_deposits = get_links(grant_pool.clone(), LinkTypes::GrantPoolToSponsor, None)?;
    let mut deposits: BTreeMap<AgentPubKey, U256> = BTreeMap::new();
    for link in links_for_deposits {
        deposits.insert(
            link.target
                .into_agent_pub_key()
                .ok_or(wasm_error!("Error converting AgentPubKey"))?,
            U256::from_le_slice(&link.tag.into_inner()),
        );
    }
    let links_for_applications =
        get_links(grant_pool.clone(), LinkTypes::GrantPoolToApplication, None)?;
    let mut grant_pool_evaluations: BTreeMap<ActionHash, Vec<ActionHash>> = BTreeMap::new();

    for app_link in links_for_applications {
        let links_for_evaluations = get_links(
            app_link.target.clone(),
            LinkTypes::ApplicationToEvaluation,
            None,
        )?;
        let mut application_evaluations: Vec<ActionHash> = Vec::new();
        for eval_link in links_for_evaluations {
            application_evaluations.push(
                eval_link
                    .target
                    .into_action_hash()
                    .ok_or(wasm_error!("Error converting ActionHash"))?,
            );
        }
        grant_pool_evaluations.insert(
            app_link
                .target
                .into_action_hash()
                .ok_or(wasm_error!("Error converting ActionHash"))?,
            application_evaluations,
        );
    }
    // TODO: finish
    let ranked_list = Vec::new();
    let coupon = Vec::new();
    //

    let grant_pool_outomce = GrantPoolOutcome {
        grant_pool,
        deposits,
        evaluations: grant_pool_evaluations,
        ranked_list,
        coupon,
    };
    create_grant_pool_outcome(grant_pool_outomce)
}

// .into_iter()
//                 .map(|l| {
//                     l.target
//                         .into_action_hash()
//                         .ok_or(wasm_error!("Error converting ActionHash"))?
//                 })
//                 .collect();

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
