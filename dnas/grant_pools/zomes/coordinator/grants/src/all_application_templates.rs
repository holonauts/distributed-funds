use grants_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn get_all_application_templates(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_application_templates");
    get_links(
        path.path_entry_hash()?,
        LinkTypes::AllApplicationTemplates,
        None,
    )
}
