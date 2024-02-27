use grants_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn create_evaluation_template(evaluation_template: EvaluationTemplate) -> ExternResult<Record> {
    let evaluation_template_hash =
        create_entry(&EntryTypes::EvaluationTemplate(evaluation_template.clone()))?;
    let record = get(evaluation_template_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest(String::from(
            "Could not find the newly created EvaluationTemplate"
        ))),
    )?;
    let path = Path::from("all_evaluation_templates");
    create_link(
        path.path_entry_hash()?,
        evaluation_template_hash.clone(),
        LinkTypes::AllEvaluationTemplates,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_evaluation_template(
    evaluation_template_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(evaluation_template_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed get details response"
        )))),
    }
}
