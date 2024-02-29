use alloy_primitives::U256;
use hdi::prelude::*;
use std::collections::BTreeMap;

// use crate::Evaluation;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AbsoluteScore {
    pub application: ActionHash,
    pub score: f64,
}
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct GrantPoolOutcome {
    pub grant_pool: ActionHash,
    pub deposits: BTreeMap<AgentPubKey, U256>,
    pub evaluations: BTreeMap<ActionHash, Vec<ActionHash>>,
    pub ranked_list: Vec<AbsoluteScore>,
    pub coupon: Vec<u32>,
}
pub fn validate_create_grant_pool_outcome(
    action: EntryCreationAction,
    grant_pool_outcome: GrantPoolOutcome,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(grant_pool_outcome.grant_pool.clone())?;
    let _grant_pool: crate::GrantPool = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Dependant action must be accompanied by an entry"
        ))))?;
    if record.signed_action.action().author() != action.author() {
        return Ok(ValidateCallbackResult::Invalid(
            "Only grant pool author can create an outcome".to_string(),
        ));
    }
    for (application, evaluations) in grant_pool_outcome.evaluations.clone() {
        must_get_valid_record(application.clone())?;
        for evaluation_action_hash in evaluations {
            let _record = must_get_valid_record(evaluation_action_hash.clone())?;
            // let evaluation: Evaluation = record
            //     .entry()
            //     .to_app_option()
            //     .map_err(|e| wasm_error!(e))?
            //     .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            //         "Dependant action must be accompanied by an entry"
            //     ))))?;
        }
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_grant_pool_outcome(
    _action: Update,
    _grant_pool_outcome: GrantPoolOutcome,
    _original_action: EntryCreationAction,
    _original_grant_pool_outcome: GrantPoolOutcome,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Grant Pool Outcomes cannot be updated",
    )))
}
pub fn validate_delete_grant_pool_outcome(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_grant_pool_outcome: GrantPoolOutcome,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Grant Pool Outcomes cannot be deleted",
    )))
}
pub fn validate_create_link_grant_pool_to_grant_pool_outcomes(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
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
    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "No action hash associated with link"
            ))))?;
    let record = must_get_valid_record(action_hash)?;
    let _grant_pool_outcome: crate::GrantPoolOutcome = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_grant_pool_to_grant_pool_outcomes(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "GrantPoolToGrantPoolOutcomes links cannot be deleted",
    )))
}
