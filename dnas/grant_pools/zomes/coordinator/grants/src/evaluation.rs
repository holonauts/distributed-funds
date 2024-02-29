use grants_integrity::Score;
use grants_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn create_evaluation(evaluation: Evaluation) -> ExternResult<Record> {
    let evaluation_hash = create_entry(&EntryTypes::Evaluation(evaluation.clone()))?;
    create_link(
        evaluation.application.clone(),
        evaluation_hash.clone(),
        LinkTypes::ApplicationToEvaluation,
        (),
    )?;
    let record = get(evaluation_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the newly created Evaluation"))
    ))?;
    Ok(record)
}
#[hdk_extern]
pub fn get_evaluation(evaluation_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(evaluation_hash, GetOptions::default())? else {
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
pub fn get_evaluations_for_application(application_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(application_hash, LinkTypes::ApplicationToEvaluation, None)
}
pub fn get_score_for_evaluation(evaluation: Evaluation) -> u64 {
    match evaluation.score {
        Score::Single(score) => score,
        Score::Weighted(vec) => {
            let total: u64 = vec.iter().map(|score| score.value * score.weight).sum();
            total
        }
    }
}
