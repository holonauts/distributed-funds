use hdi::prelude::*;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum EvaluationStatus {
    Draft,
    Submitted,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AttributeScore {
    pub label: String,
    pub value: u64,
    pub weight: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub status: EvaluationStatus,
}
pub fn validate_create_evaluation(
    _action: EntryCreationAction,
    evaluation: Evaluation,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(evaluation.application.clone())?;
    let _application: crate::Application = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Dependant action must be accompanied by an entry"
        ))))?;
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
pub fn validate_create_link_application_to_evaluations(
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
pub fn validate_delete_link_application_to_evaluations(
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
