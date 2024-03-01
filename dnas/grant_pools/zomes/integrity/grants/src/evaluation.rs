use crate::{EvaluationTemplate, ScoreTemplate};
use hdi::prelude::*;
use rust_decimal::Decimal;
use std::iter::zip;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
pub struct AttributeScore {
    pub label: String,
    pub value: u64,
    pub weight: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
#[serde(tag = "type", content = "content")]
pub enum Score {
    Single(u64),
    Weighted(Vec<AttributeScore>),
}
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Evaluation {
    pub application: ActionHash,
    pub form_content: String,
    pub comments: String,
    pub score: Score,
}

pub fn get_score_for_evaluation(evaluation: Evaluation) -> u64 {
    match evaluation.score {
        Score::Single(score) => score,
        Score::Weighted(vec) => {
            let total: u64 = vec.iter().map(|score| score.value * score.weight).sum();
            total
        }
    }
}

pub fn calc_absolute_score(raw_scores: Vec<u64>, num_evals: usize) -> Decimal {
    Decimal::from(raw_scores.iter().sum::<u64>()) / Decimal::from(num_evals)
}

pub fn validate_create_evaluation(
    action: EntryCreationAction,
    evaluation: Evaluation,
) -> ExternResult<ValidateCallbackResult> {
    let app_record = must_get_valid_record(evaluation.application.clone())?;
    let application: crate::Application = app_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Dependant action must be accompanied by an entry"
        ))))?;

    let pool_record = must_get_valid_record(application.grant_pool)?;
    let grant_pool: crate::GrantPool = pool_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Dependant action must be accompanied by an entry"
        ))))?;

    if !grant_pool.evaluators.contains(action.author()) {
        return Ok(ValidateCallbackResult::Invalid(
            "Only the grant pool's evaluators can create evaluations".into(),
        ));
    }

    let eval_template_record = must_get_valid_record(grant_pool.evaluation_template)?;
    let evaluation_template: EvaluationTemplate = eval_template_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Dependant action must be accompanied by an entry"
        ))))?;

    match evaluation.score {
        Score::Single(_) => {
            if evaluation_template.score != ScoreTemplate::Single {
                return Ok(ValidateCallbackResult::Invalid(
                    "Evaluation score must match template".into(),
                ));
            }
        }
        Score::Weighted(mut scores) => match evaluation_template.score {
            ScoreTemplate::Single => {
                return Ok(ValidateCallbackResult::Invalid(
                    "Evaluation score must match template".into(),
                ));
            }
            ScoreTemplate::Weighted(mut scores_template) => {
                if scores.len() != scores_template.len() {
                    return Ok(ValidateCallbackResult::Invalid(
                        "Evaluation score must match template".into(),
                    ));
                }
                scores.sort_by_key(|s| s.label.clone());
                scores_template.sort_by_key(|s| s.label.clone());
                let iter = zip(scores, scores_template);
                for item in iter {
                    if item.0.label != item.1.label {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Evaluation score must match template".into(),
                        ));
                    }
                    if item.0.weight != item.1.weight {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Evaluation score must match template".into(),
                        ));
                    }
                }
            }
        },
    }

    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_evaluation(
    _action: Update,
    _evaluation: Evaluation,
    _original_action: EntryCreationAction,
    _original_evaluation: Evaluation,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Evaluations cannot be updated",
    )))
}
pub fn validate_delete_evaluation(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_evaluation: Evaluation,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Evaluations cannot be deleted",
    )))
}
pub fn validate_create_link_application_to_evaluation(
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
    let _application: crate::Application = record
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
    let _evaluation: crate::Evaluation = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_application_to_evaluation(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "ApplicationToEvaluations links cannot be deleted",
    )))
}
