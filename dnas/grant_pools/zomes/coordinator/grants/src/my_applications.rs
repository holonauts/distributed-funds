use hdk::prelude::*;
use grants_integrity::*;
#[hdk_extern]
pub fn get_my_applications(author: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(author, LinkTypes::MyApplications, None)
}
