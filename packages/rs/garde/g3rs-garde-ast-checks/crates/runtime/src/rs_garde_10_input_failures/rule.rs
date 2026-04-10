use guardrail3_check_types::G3CheckResult;

use crate::support::{InputFailureSite, error};

const ID: &str = "RS-GARDE-10";

pub(crate) fn check(site: &InputFailureSite, results: &mut Vec<G3CheckResult>) {
    results.push(error(
        ID,
        "garde-family input failure",
        site.message.clone(),
        site.rel_path.as_str(),
        None,
    ));
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
