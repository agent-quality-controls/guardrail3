use g3ts_package_types::{
    G3TsPackageChecksInput, G3TsPackageRootSnapshot, G3TsPackageScriptCommandSeparator,
};
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_root};

const ID: &str = "g3ts-package/validate-script-fail-closed";

pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    if snapshot.validate_script.is_none() {
        return;
    }

    if validate_script_is_fail_closed(snapshot) {
        results.push(info(
            ID,
            "validate script is fail-closed",
            "The root package manifest defines `validate` with supported shell syntax and no reachable `||` fallback.".to_owned(),
            &snapshot.rel_path,
        ));
    } else {
        results.push(error(
            ID,
            "validate script is not fail-closed",
            "The root package manifest must define `validate` with supported shell syntax and no reachable `||` fallback.".to_owned(),
            &snapshot.rel_path,
        ));
    }
}

fn validate_script_is_fail_closed(snapshot: &G3TsPackageRootSnapshot) -> bool {
    snapshot
        .script_parse_blockers
        .iter()
        .all(|blocker| blocker.script_name != "validate")
        && snapshot
            .script_tool_invocations
            .iter()
            .filter(|invocation| invocation.script_name == "validate")
            .all(|invocation| {
                invocation.preceded_by != Some(G3TsPackageScriptCommandSeparator::Or)
                    && invocation.followed_by != Some(G3TsPackageScriptCommandSeparator::Or)
            })
}

#[cfg(test)]
#[path = "validate_script_fail_closed_tests/mod.rs"]
mod validate_script_fail_closed_tests;
