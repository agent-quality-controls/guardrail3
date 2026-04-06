use cargo_toml_parser::InheritableValue;
use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{crate_name, error, info, is_publishable};

/// Check ID for keywords presence.
const ID: &str = "RS-RELEASE-CONFIG-04";

/// Verify that a publishable crate has 1-5 keywords.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_publishable(&input.cargo) {
        return;
    }

    let name = crate_name(&input.cargo, &input.cargo_rel_path);
    let file = &input.cargo_rel_path;

    let keywords = input.cargo.package.as_ref().and_then(|p| p.keywords.as_ref());

    match keywords {
        Some(InheritableValue::Inherit(_)) => {
            // Workspace manages keywords — treat as valid.
            results.push(info(ID, format!("{name}: keywords present"), String::new(), file));
        }
        Some(InheritableValue::Value(kw)) => {
            let count = kw.len();
            if (1..=5).contains(&count) {
                results.push(info(ID, format!("{name}: keywords present"), String::new(), file));
            } else {
                results.push(error(
                    ID,
                    format!("{name}: keywords count invalid ({count})"),
                    "Publishable crates must have between 1 and 5 keywords.".to_owned(),
                    file,
                ));
            }
        }
        None => {
            results.push(error(
                ID,
                format!("{name}: keywords missing"),
                "Publishable crates must have keywords in [package].".to_owned(),
                file,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
