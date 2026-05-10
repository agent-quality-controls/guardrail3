use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_root};

/// `ID` constant.
const ID: &str = "g3ts-package/root-scripts";

/// `check`: check.
pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    let mut problems = Vec::new();

    if !snapshot.safely_runs_only_allow_pnpm {
        problems.push(
            "scripts.preinstall must run `only-allow pnpm` in a supported fail-closed command position"
                .to_owned(),
        );
    }
    if snapshot.prepare_script.is_none() {
        problems.push("scripts.prepare is missing".to_owned());
    }
    if snapshot.lint_script.is_none() {
        problems.push("scripts.lint is missing".to_owned());
    }
    if snapshot.typecheck_script.is_none() {
        problems.push("scripts.typecheck is missing".to_owned());
    }

    if problems.is_empty() {
        results.push(info(
            ID,
            "root package scripts are present",
            "The root package manifest keeps the required `preinstall`, `prepare`, `lint`, and `typecheck` scripts."
                .to_owned(),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "root package scripts are incomplete",
        format!(
            "The root package manifest script baseline is broken: {}.",
            problems.join("; ")
        ),
        &snapshot.rel_path,
    ));
}
