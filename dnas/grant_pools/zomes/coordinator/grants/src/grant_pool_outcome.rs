use crate::evaluation::{get_evaluation, get_score_for_evaluation};
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
    let mut absolute_scores: Vec<AbsoluteScore> = Vec::new();
    for app_link in links_for_applications {
        let app_action_hash = app_link
            .target
            .into_action_hash()
            .ok_or(wasm_error!("Error converting ActionHash"))?;

        let links_for_evaluations = get_links(
            app_action_hash.clone(),
            LinkTypes::ApplicationToEvaluation,
            None,
        )?;

        let num_evals = links_for_evaluations.len() as f64;

        let mut evaluation_action_hashes: Vec<ActionHash> = Vec::new();
        for eval_link in links_for_evaluations {
            let eval_action_hash = eval_link
                .target
                .into_action_hash()
                .ok_or(wasm_error!("Error converting ActionHash"))?;
            evaluation_action_hashes.push(action_hash.clone());
            let record = get_evaluation(action_hash.clone())?.ok_or(wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference a record"))
            ))?;
            let evaluation: Evaluation = record
                .entry()
                .to_app_option()
                .map_err(|e| wasm_error!(e))?
                .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                    "Record must contain an entry"
                ))))?;
            let score = get_score_for_evaluation(evaluation.clone());
            let absolute_score = AbsoluteScore {
                application: action_hash,
                score,
            };
            absolute_scores.push(absolute_score);
        }
        let raw_sum: f64 = raw_scores.iter().sum::<f64>() as f64;

        let absolute_score = AbsoluteScore {
            application: app_action_hash.clone(),
            score: raw_sum / num_evals,
        };

        absolute_scores.push(absolute_score);
        grant_pool_evaluations.insert(app_action_hash, evaluation_action_hashes);
    }
    absolute_scores.sort_by(|a, b| b.score.cmp(&a.score));
    let coupon = Vec::new();
    let grant_pool_outomce = GrantPoolOutcome {
        grant_pool,
        deposits,
        evaluations: grant_pool_evaluations,
        ranked_list: absolute_scores,
        coupon,
    };
    create_grant_pool_outcome(grant_pool_outcome)
}
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
