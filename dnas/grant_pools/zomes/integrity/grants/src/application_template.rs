use hdi::prelude::*;
use serde_json::Value;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ApplicationTemplate {
    pub json_schema: String,
    pub name: String,
}
pub fn validate_create_application_template(
    _action: EntryCreationAction,
    application_template: ApplicationTemplate,
) -> ExternResult<ValidateCallbackResult> {
    let valid_json: Result<Value, serde_json::Error> =
        serde_json::from_str(&application_template.json_schema);
    if valid_json.is_err() {
        return Ok(ValidateCallbackResult::Invalid(
            "Schema not valid json".to_string(),
        ));
    }
    if application_template.name.is_empty() {
        return Ok(ValidateCallbackResult::Invalid(
            "Name cannot be empty".to_string(),
        ));
    }

    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_application_template(
    _action: Update,
    _application_template: ApplicationTemplate,
    _original_action: EntryCreationAction,
    _original_application_template: ApplicationTemplate,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Application Templates cannot be updated",
    )))
}
pub fn validate_delete_application_template(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_application_template: ApplicationTemplate,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Application Templates cannot be deleted",
    )))
}
pub fn validate_create_link_all_application_templates(
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
    let _application_template: crate::ApplicationTemplate = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_application_templates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
