use hdk::prelude::*;
use grants_integrity::*;
#[hdk_extern]
pub fn create_grant_pool(grant_pool: GrantPool) -> ExternResult<Record> {
    let grant_pool_hash = create_entry(&EntryTypes::GrantPool(grant_pool.clone()))?;
    create_link(
        grant_pool.time_period.clone(),
        grant_pool_hash.clone(),
        LinkTypes::TimePeriodToGrantPools,
        (),
    )?;
    create_link(
        grant_pool.application_template.clone(),
        grant_pool_hash.clone(),
        LinkTypes::ApplicationTemplateToGrantPools,
        (),
    )?;
    create_link(
        grant_pool.evaluation_template.clone(),
        grant_pool_hash.clone(),
        LinkTypes::EvaluationTemplateToGrantPools,
        (),
    )?;
    let record = get(grant_pool_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created GrantPool"))
            ),
        )?;
    let path = Path::from("all_grant_pools");
    create_link(
        path.path_entry_hash()?,
        grant_pool_hash.clone(),
        LinkTypes::AllGrantPools,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_grant_pool(grant_pool_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(grant_pool_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(String::from("Malformed get details response"))
                ),
            )
        }
    }
}
#[hdk_extern]
pub fn get_grant_pools_for_time_period(
    time_period_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(time_period_hash, LinkTypes::TimePeriodToGrantPools, None)
}
#[hdk_extern]
pub fn get_grant_pools_for_application_template(
    application_template_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        application_template_hash,
        LinkTypes::ApplicationTemplateToGrantPools,
        None,
    )
}
#[hdk_extern]
pub fn get_grant_pools_for_evaluation_template(
    evaluation_template_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(evaluation_template_hash, LinkTypes::EvaluationTemplateToGrantPools, None)
}
