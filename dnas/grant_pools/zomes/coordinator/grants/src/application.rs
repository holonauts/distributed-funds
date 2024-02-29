use crate::{
    evaluator_to_applications::{add_application_for_evaluator, AddApplicationForEvaluatorInput},
    grant_pool_to_applications::{add_application_for_grant_pool, AddApplicationForGrantPoolInput},
};
use grants_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn create_application(application: Application) -> ExternResult<Record> {
    let application_hash = create_entry(&EntryTypes::Application(application.clone()))?;
    let record = get(application_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the newly created Application"))
    ))?;
    let path = Path::from("all_applications");
    create_link(
        path.path_entry_hash()?,
        application_hash.clone(),
        LinkTypes::AllApplications,
        (),
    )?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    create_link(
        my_agent_pub_key,
        application_hash.clone(),
        LinkTypes::MyApplications,
        (),
    )?;
    add_application_for_grant_pool(AddApplicationForGrantPoolInput {
        base_grant_pool_hash: application.grant_pool.clone(),
        target_application_hash: record.action_address().clone(),
    })?;
    let grant_pool_record = get(application.grant_pool, GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("No grant pool found".into())),
    )?;
    let grant_pool: GrantPool = grant_pool_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Record must contain an entry"
        ))))?;
    for evaluator in grant_pool.evaluators {
        let _ = add_application_for_evaluator(AddApplicationForEvaluatorInput {
            base_evaluator: evaluator,
            target_application_hash: application_hash.clone(),
        })?;
    }

    Ok(record)
}
#[hdk_extern]
pub fn get_latest_application(
    original_application_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_application_hash.clone(),
        LinkTypes::ApplicationUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_application_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                    "No action hash associated with link"
                ))))?
        }
        None => original_application_hash.clone(),
    };
    get(latest_application_hash, GetOptions::default())
}
#[hdk_extern]
pub fn get_original_application(
    original_application_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_application_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed get details response"
        )))),
    }
}
#[hdk_extern]
pub fn get_all_revisions_for_application(
    original_application_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_application(original_application_hash.clone())? else {
        return Ok(vec![]);
    };
    let links = get_links(
        original_application_hash.clone(),
        LinkTypes::ApplicationUpdates,
        None,
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| {
            Ok(GetInput::new(
                link.target
                    .into_action_hash()
                    .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                        "No action hash associated with link"
                    ))))?
                    .into(),
                GetOptions::default(),
            ))
        })
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let mut records: Vec<Record> = records.into_iter().flatten().collect();
    records.insert(0, original_record);
    Ok(records)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateApplicationInput {
    pub original_application_hash: ActionHash,
    pub previous_application_hash: ActionHash,
    pub updated_application: Application,
}
#[hdk_extern]
pub fn update_application(input: UpdateApplicationInput) -> ExternResult<Record> {
    let updated_application_hash = update_entry(
        input.previous_application_hash.clone(),
        &input.updated_application,
    )?;
    create_link(
        input.original_application_hash.clone(),
        updated_application_hash.clone(),
        LinkTypes::ApplicationUpdates,
        (),
    )?;
    let record =
        get(updated_application_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(String::from("Could not find the newly updated Application"))
        ))?;
    Ok(record)
}
