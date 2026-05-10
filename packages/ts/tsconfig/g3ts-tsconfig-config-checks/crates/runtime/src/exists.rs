use g3ts_tsconfig_types::G3TsTsconfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{info, root_rel_path};

/// Check ID emitted by this rule.
const ID: &str = "g3ts-tsconfig/exists";

/// Emit a finding describing whether a root tsconfig file exists.
pub(crate) fn check(input: &G3TsTsconfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(rel_path) = root_rel_path(input) else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "tsconfig missing".to_owned(),
            "No root `tsconfig.json` or `tsconfig.base.json` file was found. Add a root TypeScript config.".to_owned(),
            None,
            None,
        ));
        return;
    };

    results.push(info(
        ID,
        "tsconfig exists",
        format!("Found root TypeScript config `{rel_path}`."),
        rel_path,
    ));
}
