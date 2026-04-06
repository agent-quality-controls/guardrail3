use cargo_toml_parser::InheritableValue;
use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{crate_name, error, info, is_publishable};

/// Check ID for categories presence.
const ID: &str = "RS-RELEASE-CONFIG-05";

/// Verify that a publishable crate has at least one category.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_publishable(&input.cargo) {
        return;
    }

    let name = crate_name(&input.cargo, &input.cargo_rel_path);
    let file = &input.cargo_rel_path;

    let categories = input.cargo.package.as_ref().and_then(|p| p.categories.as_ref());

    match categories {
        Some(InheritableValue::Inherit(_)) => {
            // Workspace manages categories — treat as valid.
            results.push(info(ID, format!("{name}: categories present"), String::new(), file));
        }
        Some(InheritableValue::Value(cats)) => {
            if cats.is_empty() {
                results.push(error(
                    ID,
                    format!("{name}: categories missing"),
                    "Publishable crates must have at least one category.".to_owned(),
                    file,
                ));
            } else {
                results.push(info(ID, format!("{name}: categories present"), String::new(), file));
            }
        }
        None => {
            results.push(error(
                ID,
                format!("{name}: categories missing"),
                "Publishable crates must have categories in [package].".to_owned(),
                file,
            ));
        }
    }
}
