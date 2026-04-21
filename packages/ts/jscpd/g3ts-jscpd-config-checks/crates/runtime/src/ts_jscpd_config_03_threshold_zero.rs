use g3ts_jscpd_types::G3TsJscpdChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_root};

const ID: &str = "TS-JSCPD-CONFIG-03";

pub(crate) fn check(input: &G3TsJscpdChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    match snapshot.threshold {
        Some(0) => results.push(info(
            ID,
            "jscpd threshold set to zero",
            "The root `.jscpd.json` enforces zero duplication tolerance with `threshold: 0`."
                .to_owned(),
            &snapshot.rel_path,
        )),
        Some(value) => results.push(error(
            ID,
            "jscpd threshold is not zero",
            format!(
                "Root `.jscpd.json` sets `threshold` to `{value}`, but the current baseline requires `0`."
            ),
            &snapshot.rel_path,
        )),
        None => results.push(error(
            ID,
            "jscpd threshold missing",
            "Root `.jscpd.json` must set `threshold: 0`.".to_owned(),
            &snapshot.rel_path,
        )),
    }
}
