use grants_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn create_time_period(time_period: TimePeriod) -> ExternResult<Record> {
    let time_period_hash = create_entry(&EntryTypes::TimePeriod(time_period.clone()))?;
    let record = get(time_period_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the newly created TimePeriod"))
    ))?;
    let path = Path::from("all_time_periods");
    create_link(
        path.path_entry_hash()?,
        time_period_hash.clone(),
        LinkTypes::AllTimePeriods,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_time_period(time_period_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(time_period_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed get details response"
        )))),
    }
}
