use guardrail3_app_rs_placement::RustRootClassification;
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RootClassificationInput;

const ID: &str = "RS-ARCH-01";

pub fn check(input: &RootClassificationInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.classification != RustRootClassification::Ambiguous {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!(
            "Rust root `{}` has ambiguous architecture classification",
            display_dir(&input.root.rel_dir)
        ),
        message: format!(
            "`{}` matches multiple architecture zones. app candidates: [{}]; package candidates: [{}]. Each discovered Rust root must classify as exactly one of app, package, or other.",
            input.root.cargo_rel_path,
            input.root.app_zone_candidates.join(", "),
            input.root.package_zone_candidates.join(", "),
        ),
        file: Some(input.root.cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

#[cfg(test)]
#[path = "rs_arch_01_root_classification_tests/mod.rs"]
mod tests;
