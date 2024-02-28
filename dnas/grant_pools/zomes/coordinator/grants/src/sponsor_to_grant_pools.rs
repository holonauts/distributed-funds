use hdk::prelude::*;
use grants_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddGrantPoolForSponsorInput {
    pub base_sponsor: AgentPubKey,
    pub target_grant_pool_hash: ActionHash,
}
#[hdk_extern]
pub fn add_grant_pool_for_sponsor(input: AddGrantPoolForSponsorInput) -> ExternResult<()> {
    create_link(input.base_sponsor.clone(), input.target_grant_pool_hash.clone(), LinkTypes::SponsorToGrantPools, ())?;
    create_link(input.target_grant_pool_hash, input.base_sponsor, LinkTypes::GrantPoolToSponsors, ())?;

    Ok(())    
}

#[hdk_extern]
pub fn get_grant_pools_for_sponsor(sponsor: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(sponsor, LinkTypes::SponsorToGrantPools, None)
}




#[hdk_extern]
pub fn get_sponsors_for_grant_pool(grant_pool_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(grant_pool_hash, LinkTypes::GrantPoolToSponsors, None)
}


        
