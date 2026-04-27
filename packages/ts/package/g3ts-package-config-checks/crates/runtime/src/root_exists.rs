use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{info, root_rel_path};

const ID: &str = "g3ts-package/root-exists";

pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    if matches!(
        input.root,
        g3ts_package_types::G3TsPackageRootState::NotPackageManagerRoot
    ) {
        return;
    }

    let Some(rel_path) = root_rel_path(input) else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "root package.json missing".to_owned(),
            "No root `package.json` file was found. Add a root workspace manifest.".to_owned(),
            None,
            None,
        ));
        return;
    };

    results.push(info(
        ID,
        "root package.json exists",
        format!("Found root package manifest `{rel_path}`."),
        rel_path,
    ));
}
