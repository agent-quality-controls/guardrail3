use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, is_pinned_pnpm_package_manager, parsed_root};

const ID: &str = "g3ts-package/root-package-manager";

pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    if is_pinned_pnpm_package_manager(snapshot.package_manager.as_deref()) {
        results.push(info(
            ID,
            "root packageManager is pinned to pnpm",
            format!(
                "The root package manifest pins `packageManager` to `{}`.",
                snapshot
                    .package_manager
                    .as_deref()
                    .expect("pinned package manager should exist")
            ),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "root packageManager missing or not pinned to pnpm",
        "The root package manifest must set a pinned `packageManager` such as `pnpm@10.32.0`."
            .to_owned(),
        &snapshot.rel_path,
    ));
}
