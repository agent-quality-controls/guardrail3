use g3ts_npmrc_types::G3TsNpmrcChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{info, root_rel_path};

const ID: &str = "g3ts-npmrc/root-exists";

pub(crate) fn check(input: &G3TsNpmrcChecksInput, results: &mut Vec<G3CheckResult>) {
    if matches!(
        input.root,
        g3ts_npmrc_types::G3TsNpmrcRootState::NotPackageManagerRoot
    ) {
        return;
    }

    let Some(rel_path) = root_rel_path(input) else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "root .npmrc missing".to_owned(),
            "No root `.npmrc` file was found. Add a root package-manager config.".to_owned(),
            None,
            None,
        ));
        return;
    };

    results.push(info(
        ID,
        "root .npmrc exists",
        format!("Found root .npmrc `{rel_path}`."),
        rel_path,
    ));
}
