use crate::domain::report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-05";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable || krate.readme_declared_false || !krate.readme_exists {
        return;
    }
    let Some(content) = &krate.readme_content else {
        return;
    };
    if content.len() < 200 {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("{}: README is a stub", krate.name),
            message: format!(
                "README at `{}` is only {} bytes.",
                krate.readme_rel_path,
                content.len()
            ),
            file: Some(krate.readme_rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    }
    if !content
        .lines()
        .any(|line| line.trim_start().starts_with('#'))
    {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("{}: README has no heading", krate.name),
            message: format!(
                "README at `{}` has no markdown heading.",
                krate.readme_rel_path
            ),
            file: Some(krate.readme_rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    }
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: README quality looks good", krate.name),
            message: format!(
                "README at `{}` has content and headings.",
                krate.readme_rel_path
            ),
            file: Some(krate.readme_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_pub_05_readme_quality_tests/mod.rs"]
mod tests;
