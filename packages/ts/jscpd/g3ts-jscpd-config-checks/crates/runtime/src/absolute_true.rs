use g3ts_jscpd_types::G3TsJscpdChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_root};

/// Stable rule identifier reported on each emitted result.
const ID: &str = "g3ts-jscpd/absolute-true";

/// Run the rule and append any results to `results`.
pub(crate) fn check(input: &G3TsJscpdChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    match snapshot.absolute {
        Some(true) => results.push(info(
            ID,
            "jscpd absolute paths enabled",
            "The root `.jscpd.json` sets `absolute: true`.".to_owned(),
            &snapshot.rel_path,
        )),
        Some(false) => results.push(error(
            ID,
            "jscpd absolute paths disabled",
            "Root `.jscpd.json` sets `absolute: false`, but the current baseline requires `true`."
                .to_owned(),
            &snapshot.rel_path,
        )),
        None => results.push(error(
            ID,
            "jscpd absolute field missing",
            "Root `.jscpd.json` must set `absolute: true`.".to_owned(),
            &snapshot.rel_path,
        )),
    }
}
