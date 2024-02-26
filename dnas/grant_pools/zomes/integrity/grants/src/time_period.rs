use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct TimePeriod {
    pub start_at: Timestamp,
    pub end_at: Timestamp,
}
pub fn validate_create_time_period(
    _action: EntryCreationAction,
    time_period: TimePeriod,
) -> ExternResult<ValidateCallbackResult> {
    if time_period.start_at > time_period.end_at {
        Ok(ValidateCallbackResult::Invalid(
            "End time must be greater than start time".to_string(),
        ))
    } else {
        Ok(ValidateCallbackResult::Valid)
    }
}
pub fn validate_update_time_period(
    _action: Update,
    _time_period: TimePeriod,
    _original_action: EntryCreationAction,
    _original_time_period: TimePeriod,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Time Periods cannot be updated",
    )))
}
pub fn validate_delete_time_period(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_time_period: TimePeriod,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Time Periods cannot be deleted",
    )))
}
pub fn validate_create_link_all_time_periods(
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
    let _time_period: crate::TimePeriod = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_time_periods(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
