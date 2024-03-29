use alloy_primitives::{Address, U256};
use hdi::prelude::*;
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, SerializedBytes)]
#[serde(tag = "type", content = "content")]
pub enum ApplicationStatus {
    Draft,
    Submitted,
    Claimed(U256),
}
impl ApplicationStatus {
    fn is_draft(&self) -> bool {
        matches!(self, ApplicationStatus::Draft)
    }
    fn is_submitted(&self) -> bool {
        matches!(self, ApplicationStatus::Submitted)
    }
}
#[hdk_entry_helper]
#[derive(Clone, Eq, PartialEq)]
pub struct Application {
    pub grant_pool: ActionHash,
    pub form_content: String,
    pub amount: U256,
    pub status: ApplicationStatus,
    pub evm_wallet: Address,
}
pub fn validate_create_application(
    _action: EntryCreationAction,
    application: Application,
) -> ExternResult<ValidateCallbackResult> {
    if application.form_content.is_empty() {
        return Ok(ValidateCallbackResult::Invalid(
            "Content cannot be empty".to_string(),
        ));
    }
    let valid_json: Result<Value, serde_json::Error> =
        serde_json::from_str(&application.form_content);
    if valid_json.is_err() {
        return Ok(ValidateCallbackResult::Invalid(
            "Schema not valid json".to_string(),
        ));
    }
    if application.amount == U256::from(0) {
        return Ok(ValidateCallbackResult::Invalid(
            "Amount cannot be zero".to_string(),
        ));
    }
    let record = must_get_valid_record(application.grant_pool.clone())?;
    let grant_pool: crate::GrantPool = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Dependant action must be accompanied by an entry"
        ))))?;

    if application.amount < grant_pool.amount_range.min {
        return Ok(ValidateCallbackResult::Invalid(
            "Application amount must be equal to or greater than Grant Pool min range".to_string(),
        ));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_application(
    action: Update,
    application: Application,
    original_action: EntryCreationAction,
    original_application: Application,
) -> ExternResult<ValidateCallbackResult> {
    let original_status = original_application.status;
    if &action.author != original_action.author() {
        return Ok(ValidateCallbackResult::Invalid(
            "Only original author can update".to_string(),
        ));
    }
    if original_application.grant_pool != application.grant_pool {
        return Ok(ValidateCallbackResult::Invalid(
            "Grant Pool action hash must be the same".to_string(),
        ));
    }
    if !&original_status.is_draft() {
        if original_application.form_content != application.form_content {
            return Ok(ValidateCallbackResult::Invalid(
                "Content can only be changed in draft".to_string(),
            ));
        }
        if original_application.amount != application.amount {
            return Ok(ValidateCallbackResult::Invalid(
                "Amount can only be changed in draft".to_string(),
            ));
        }
    }
    match application.status {
        ApplicationStatus::Draft => {
            if !&original_status.is_draft() {
                return Ok(ValidateCallbackResult::Invalid(
                    "Status cannot be reverted".to_string(),
                ));
            }
        }
        ApplicationStatus::Submitted => {
            if !&original_status.is_draft() {
                return Ok(ValidateCallbackResult::Invalid(
                    "Status cannot be reverted".to_string(),
                ));
            }
        }
        ApplicationStatus::Claimed(_) => {
            if !&original_status.is_submitted() {
                return Ok(ValidateCallbackResult::Invalid(
                    "Status cannot be reverted".to_string(),
                ));
            }
        }
    };
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_application(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_application: Application,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Applications cannot be deleted",
    )))
}
pub fn validate_create_link_application_updates(
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
    let _application: crate::Application = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_application_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "ApplicationUpdates links cannot be deleted",
    )))
}
pub fn validate_create_link_all_applications(
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
    let _application: crate::Application = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_applications(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_my_applications(
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
    let _application: crate::Application = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_my_applications(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
