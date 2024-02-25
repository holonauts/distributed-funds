use hdk::prelude::*;
use grants_integrity::*;
#[hdk_extern]
pub fn create_application_template(
    application_template: ApplicationTemplate,
) -> ExternResult<Record> {
    let application_template_hash = create_entry(
        &EntryTypes::ApplicationTemplate(application_template.clone()),
    )?;
    let record = get(application_template_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created ApplicationTemplate"))
            ),
        )?;
    let path = Path::from("all_application_templates");
    create_link(
        path.path_entry_hash()?,
        application_template_hash.clone(),
        LinkTypes::AllApplicationTemplates,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_application_template(
    application_template_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(application_template_hash, GetOptions::default())?
    else {
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
