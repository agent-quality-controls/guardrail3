use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_root};

/// `ID` constant.
const ID: &str = "g3ts-package/root-engines";

/// `check`: check.
pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    let mut missing = Vec::new();
    if snapshot.engines_node.is_none() {
        missing.push("engines.node");
    }
    if snapshot.engines_pnpm.is_none() {
        missing.push("engines.pnpm");
    }

    if missing.is_empty() {
        results.push(info(
            ID,
            "root engines are declared",
            "The root package manifest declares both `engines.node` and `engines.pnpm`.".to_owned(),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "root engines are incomplete",
        format!(
            "The root package manifest must declare {}.",
            missing.join(" and ")
        ),
        &snapshot.rel_path,
    ));
}
