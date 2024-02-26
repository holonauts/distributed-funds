use hdi::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct NumberRange {
    min: u32,
    max: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct WeightedCriteria {
    label: String,
    weight: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum QuantitativeRating {
    Single(NumberRange),
    Weighted(Vec<WeightedCriteria>),
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct EvaluationTemplate {
    pub qualitative_json_schema: String,
    pub quantitative_rating: QuantitativeRating,
}
pub fn validate_create_evaluation_template(
    _action: EntryCreationAction,
    evaluation_template: EvaluationTemplate,
) -> ExternResult<ValidateCallbackResult> {
    let valid_json: Result<String, serde_json::Error> =
        serde_json::from_str(&evaluation_template.qualitative_json_schema);
    if valid_json.is_err() {
        return Ok(ValidateCallbackResult::Invalid(
            "Schema not valid json".to_string(),
        ));
    }
    match evaluation_template.quantitative_rating {
        QuantitativeRating::Single(range) => {
            if range.max < range.min {
                return Ok(ValidateCallbackResult::Invalid(
                    "Max must be greater than min".to_string(),
                ));
            }
        }
        QuantitativeRating::Weighted(weighted_criteria) => {
            if weighted_criteria.len() < 2 {
                return Ok(ValidateCallbackResult::Invalid(
                    "Must have more than one weighted criteria".to_string(),
                ));
            }
            for criteria in weighted_criteria {
                if criteria.label.is_empty() {
                    return Ok(ValidateCallbackResult::Invalid(
                        "Label can't be empty".to_string(),
                    ));
                }
                if criteria.weight <= 0 {
                    return Ok(ValidateCallbackResult::Invalid(
                        "Weight must be a positive value".to_string(),
                    ));
                }
            }
        }
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_evaluation_template(
    _action: Update,
    _evaluation_template: EvaluationTemplate,
    _original_action: EntryCreationAction,
    _original_evaluation_template: EvaluationTemplate,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Evaluation Templates cannot be updated",
    )))
}
pub fn validate_delete_evaluation_template(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_evaluation_template: EvaluationTemplate,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Evaluation Templates cannot be deleted",
    )))
}
pub fn validate_create_link_all_evaluation_templates(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "No action hash associated with link"
            ))))?;
    let record = must_get_valid_record(action_hash)?;
    let _evaluation_template: crate::EvaluationTemplate = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
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
