use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

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
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{}: README is a stub", krate.name),
            format!(
                "README at `{}` is only {} bytes. Add meaningful content to the README.",
                krate.readme_rel_path,
                content.len()
            ),
            Some(krate.readme_rel_path.clone()),
            None,
            false,
        ));
        return;
    }
    if !has_markdown_heading(content) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{}: README has no heading", krate.name),
            format!(
                "README at `{}` has no markdown heading. Add a markdown heading (e.g. `# Crate Name`).",
                krate.readme_rel_path
            ),
            Some(krate.readme_rel_path.clone()),
            None,
            false,
        ));
        return;
    }
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: README quality looks good", krate.name),
            format!(
                "README at `{}` has content and headings.",
                krate.readme_rel_path
            ),
            Some(krate.readme_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}




fn has_markdown_heading(content: &str) -> bool {
    let mut in_fenced_code = false;
    let mut last_text_line_can_be_setext_heading = false;
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fenced_code = !in_fenced_code;
            last_text_line_can_be_setext_heading = false;
            continue;
        }
        if in_fenced_code || line.starts_with("    ") || line.starts_with('\t') {
            last_text_line_can_be_setext_heading = false;
            continue;
        }
        if let Some(after_hashes) = trimmed.strip_prefix('#') {
            if after_hashes.starts_with('#') {
                let heading_text = trimmed.trim_start_matches('#');
                if heading_text.starts_with(char::is_whitespace) {
                    return true;
                }
                last_text_line_can_be_setext_heading = false;
                continue;
            }
            if after_hashes.starts_with(char::is_whitespace) {
                return true;
            }
            last_text_line_can_be_setext_heading = false;
        }
        if trimmed == "#" {
            return true;
        }
        if !trimmed.is_empty() && trimmed.chars().all(|ch| ch == '=' || ch == '-') {
            if last_text_line_can_be_setext_heading {
                return true;
            }
            last_text_line_can_be_setext_heading = false;
            continue;
        }
        last_text_line_can_be_setext_heading = !trimmed.is_empty();
    }
    false
}

