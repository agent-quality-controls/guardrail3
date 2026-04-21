use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_root};

const ID: &str = "TS-PACKAGE-CONFIG-06";

pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    let mut problems = Vec::new();

    if !snapshot
        .preinstall_script
        .as_deref()
        .is_some_and(|script| script.contains("only-allow pnpm"))
    {
        problems.push("scripts.preinstall must contain `only-allow pnpm`".to_owned());
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
