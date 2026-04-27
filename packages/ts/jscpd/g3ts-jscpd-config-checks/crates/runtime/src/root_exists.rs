use g3ts_jscpd_types::G3TsJscpdChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{info, root_rel_path};

const ID: &str = "g3ts-jscpd/root-exists";

pub(crate) fn check(input: &G3TsJscpdChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(rel_path) = root_rel_path(input) else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "root .jscpd.json missing".to_owned(),
            "No root `.jscpd.json` file was found. Add a root duplication-policy config."
                .to_owned(),
            None,
            None,
        ));
        return;
    };

    results.push(info(
        ID,
        "root .jscpd.json exists",
        format!("Found root JSCpd config `{rel_path}`."),
        rel_path,
    ));
}
