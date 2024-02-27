use grants_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn get_all_evaluation_templates(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_evaluation_templates");
    get_links(
        path.path_entry_hash()?,
        LinkTypes::AllEvaluationTemplates,
        None,
    )
}
