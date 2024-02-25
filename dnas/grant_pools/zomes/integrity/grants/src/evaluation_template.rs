use hdi::prelude::*;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeightedCriteria {
    label: String,
    weight: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum QuantitativeRating {
    Single(u32),
    Weighted(Vec<WeightedCriteria>),
}
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct EvaluationTemplate {
    pub qualitative_json_schema: String,
    pub quantitative_rating: QuantitativeRating,
}
pub fn validate_create_evaluation_template(
    _action: EntryCreationAction,
    _evaluation_template: EvaluationTemplate,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_evaluation_template(
    _action: Update,
    _evaluation_template: EvaluationTemplate,
    _original_action: EntryCreationAction,
    _original_evaluation_template: EvaluationTemplate,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Evaluation Templates cannot be updated"),
        ),
    )
}
pub fn validate_delete_evaluation_template(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_evaluation_template: EvaluationTemplate,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Evaluation Templates cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_evaluation_templates(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _evaluation_template: crate::EvaluationTemplate = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_evaluation_templates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
