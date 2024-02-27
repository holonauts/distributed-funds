use grants_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn get_all_grant_pools(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_grant_pools");
    get_links(path.path_entry_hash()?, LinkTypes::AllGrantPools, None)
}
