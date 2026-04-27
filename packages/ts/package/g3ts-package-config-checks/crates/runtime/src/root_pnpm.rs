use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_root};

const ID: &str = "g3ts-package/root-pnpm";

pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    let mut missing = Vec::new();
    if snapshot.pnpm_override_keys.is_empty() {
        missing.push("pnpm.overrides");
    }
    if snapshot.pnpm_only_built_dependencies.is_empty() {
        missing.push("pnpm.onlyBuiltDependencies");
    }

    if missing.is_empty() {
        results.push(info(
            ID,
            "root pnpm policy is present",
            "The root package manifest keeps both `pnpm.overrides` and `pnpm.onlyBuiltDependencies`."
                .to_owned(),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "root pnpm policy is incomplete",
        format!(
            "The root package manifest must declare {}.",
            missing.join(" and ")
        ),
        &snapshot.rel_path,
    ));
}
