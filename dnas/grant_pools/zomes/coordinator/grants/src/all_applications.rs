use grants_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn get_all_applications(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_applications");
    get_links(path.path_entry_hash()?, LinkTypes::AllApplications, None)
}
