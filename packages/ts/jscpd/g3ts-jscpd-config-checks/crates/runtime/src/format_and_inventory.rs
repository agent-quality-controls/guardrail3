use g3ts_jscpd_types::G3TsJscpdChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, has_typescript_format, info, parsed_root};

/// Stable rule identifier reported on each emitted result.
const ID: &str = "g3ts-jscpd/format-and-inventory";

/// Run the rule and append any results to `results`.
pub(crate) fn check(input: &G3TsJscpdChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    if has_typescript_format(snapshot) {
        results.push(info(
            ID,
            "jscpd format includes typescript",
            "The root `.jscpd.json` format list includes `typescript`.".to_owned(),
            &snapshot.rel_path,
        ));
    } else {
        results.push(error(
            ID,
            "jscpd format misses typescript",
            "Root `.jscpd.json` must include `typescript` in `format`.".to_owned(),
            &snapshot.rel_path,
        ));
    }

    for key in &snapshot.extra_keys {
        results.push(info(
            ID,
            "jscpd extra top-level key present",
            format!(
                "Extra root `.jscpd.json` key `{key}` is outside the current wave-1 baseline. Keep it only if intentional."
            ),
            &snapshot.rel_path,
        ));
    }
}
