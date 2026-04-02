use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-01";
const ALLOWED_LICENSE_PATHS: &[&str] = &["LICENSE", "LICENSE-MIT", "LICENSE-APACHE", "LICENSE.md"];

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    match &input.repo.license_rel_path {
        Some(rel_path) if ALLOWED_LICENSE_PATHS.contains(&rel_path.as_str()) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "LICENSE file exists".to_owned(),
                format!("Repo root includes `{rel_path}`."),
                Some(rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        _ => results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    "LICENSE file missing".to_owned(),
    "Repo root is missing LICENSE material (`LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, or `LICENSE.md`).".to_owned(),
    Some(input.repo.cargo_rel_path.clone()),
    None,
    false,
        )),
    }
}

