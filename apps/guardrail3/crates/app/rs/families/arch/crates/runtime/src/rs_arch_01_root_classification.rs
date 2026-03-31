use guardrail3_app_rs_placement::RustRootClassification;
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RootClassificationInput;

const ID: &str = "RS-ARCH-01";

pub fn check(input: &RootClassificationInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.classification == RustRootClassification::Ambiguous {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!(
                "Rust root `{}` has ambiguous architecture classification",
                display_dir(&input.root.rel_dir)
            ),
            format!(
                "`{}` matches multiple architecture zones. app candidates: [{}]; package candidates: [{}]. Each discovered Rust root must classify as exactly one of app, package, or other.",
                input.root.cargo_rel_path,
                input.root.app_zone_candidates.join(", "),
                input.root.package_zone_candidates.join(", "),
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!(
                "Rust root `{}` classification is unambiguous",
                display_dir(&input.root.rel_dir)
            ),
            format!(
                "`{}` classifies cleanly as `{}`.",
                input.root.cargo_rel_path,
                classification_label(input.root.classification),
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

fn classification_label(classification: RustRootClassification) -> &'static str {
    match classification {
        RustRootClassification::App => "app",
        RustRootClassification::Package => "package",
        RustRootClassification::Auxiliary => "auxiliary",
        RustRootClassification::Other => "other",
        RustRootClassification::Ambiguous => "ambiguous",
    }
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_arch_01_root_classification_tests/mod.rs"]
mod rs_arch_01_root_classification_tests;
