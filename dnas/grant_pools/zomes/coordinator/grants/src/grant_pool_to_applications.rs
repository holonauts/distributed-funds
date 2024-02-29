use grants_integrity::*;
use hdk::prelude::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddApplicationForGrantPoolInput {
    pub base_grant_pool_hash: ActionHash,
    pub target_application_hash: ActionHash,
}
#[hdk_extern]
pub fn add_application_for_grant_pool(input: AddApplicationForGrantPoolInput) -> ExternResult<()> {
    create_link(
        input.base_grant_pool_hash.clone(),
        input.target_application_hash.clone(),
        LinkTypes::GrantPoolToApplication,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_applications_for_grant_pool(grant_pool_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(grant_pool_hash, LinkTypes::GrantPoolToApplication, None)
}
#[hdk_extern]
pub fn get_deleted_applications_for_grant_pool(
    grant_pool_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(grant_pool_hash, LinkTypes::GrantPoolToApplication, None)?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveApplicationForGrantPoolInput {
    pub base_grant_pool_hash: ActionHash,
    pub target_application_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_application_for_grant_pool(
    input: RemoveApplicationForGrantPoolInput,
) -> ExternResult<()> {
    let links = get_links(
        input.base_grant_pool_hash.clone(),
        LinkTypes::GrantPoolToApplication,
        None,
    )?;
    for link in links {
        if link
            .target
            .clone()
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "No action hash associated with link"
            ))))?
            .eq(&input.target_application_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
