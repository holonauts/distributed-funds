use alloy_primitives::{Address, U256};
use hdi::prelude::*;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AmountRange {
    min: U256,
    max: U256,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AllowedERC20 {
    token: Address,
    decimals: u8,
}
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct GrantPool {
    pub name: String,
    pub purpose_description: String,
    pub rules_description: String,
    pub time_period: ActionHash,
    pub application_template: ActionHash,
    pub evaluation_template: ActionHash,
    pub amount_range: AmountRange,
    pub allowed_erc20: AllowedERC20,
    pub evaluators: Vec<AgentPubKey>,
}
pub fn validate_create_grant_pool(
    _action: EntryCreationAction,
    grant_pool: GrantPool,
) -> ExternResult<ValidateCallbackResult> {
    if grant_pool.name.is_empty() {
        return Ok(ValidateCallbackResult::Invalid("Name cannot be empty".to_string()));
    }
    if grant_pool.purpose_description.is_empty() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Purpose description cannot be empty".to_string(),
            ),
        );
    }
    if grant_pool.rules_description.is_empty() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Rules description cannot be empty".to_string(),
            ),
        );
    }
    if grant_pool.evaluators.is_empty() {
        return Ok(
            ValidateCallbackResult::Invalid("Evaluators cannot be empty".to_string()),
        );
    }
    let record = must_get_valid_record(grant_pool.time_period.clone())?;
    let _time_period: crate::TimePeriod = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Dependant action must be accompanied by an entry"))
            ),
        )?;
    let record = must_get_valid_record(grant_pool.application_template.clone())?;
    let _application_template: crate::ApplicationTemplate = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Dependant action must be accompanied by an entry"))
            ),
        )?;
    let record = must_get_valid_record(grant_pool.evaluation_template.clone())?;
    let _evaluation_template: crate::EvaluationTemplate = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Dependant action must be accompanied by an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_grant_pool(
    _action: Update,
    _grant_pool: GrantPool,
    _original_action: EntryCreationAction,
    _original_grant_pool: GrantPool,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Grant Pools cannot be updated")))
}
pub fn validate_delete_grant_pool(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_grant_pool: GrantPool,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Grant Pools cannot be deleted")))
}
pub fn validate_create_link_time_period_to_grant_pools(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = base_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _time_period: crate::TimePeriod = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _grant_pool: crate::GrantPool = record
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
pub fn validate_delete_link_time_period_to_grant_pools(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("TimePeriodToGrantPools links cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_application_template_to_grant_pools(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = base_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _application_template: crate::ApplicationTemplate = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _grant_pool: crate::GrantPool = record
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
pub fn validate_delete_link_application_template_to_grant_pools(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("ApplicationTemplateToGrantPools links cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_evaluation_template_to_grant_pools(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = base_address
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
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _grant_pool: crate::GrantPool = record
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
pub fn validate_delete_link_evaluation_template_to_grant_pools(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("EvaluationTemplateToGrantPools links cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_grant_pools(
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
    let _grant_pool: crate::GrantPool = record
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
pub fn validate_delete_link_all_grant_pools(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
