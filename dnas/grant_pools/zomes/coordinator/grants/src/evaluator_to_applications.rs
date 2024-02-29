use grants_integrity::*;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddApplicationForEvaluatorInput {
    pub base_evaluator: AgentPubKey,
    pub target_application_hash: ActionHash,
}
#[hdk_extern]
pub fn add_application_for_evaluator(input: AddApplicationForEvaluatorInput) -> ExternResult<()> {
    create_link(
        input.base_evaluator.clone(),
        input.target_application_hash.clone(),
        LinkTypes::EvaluatorToApplications,
        (),
    )?;
    create_link(
        input.target_application_hash,
        input.base_evaluator,
        LinkTypes::ApplicationToEvaluators,
        (),
    )?;

    Ok(())
}

#[hdk_extern]
pub fn get_applications_for_evaluator(evaluator: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(evaluator, LinkTypes::EvaluatorToApplications, None)
}

#[hdk_extern]
pub fn get_evaluators_for_application(application_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(application_hash, LinkTypes::ApplicationToEvaluators, None)
}
