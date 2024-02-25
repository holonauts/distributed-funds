use hdk::prelude::*;
use grants_integrity::*;
#[hdk_extern]
pub fn get_all_time_periods(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_time_periods");
    get_links(path.path_entry_hash()?, LinkTypes::AllTimePeriods, None)
}
