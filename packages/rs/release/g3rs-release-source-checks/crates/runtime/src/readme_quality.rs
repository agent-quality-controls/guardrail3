use g3rs_release_types::G3RsReleaseSourceReadme;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-release/readme-quality";

pub(crate) fn check(readme: &G3RsReleaseSourceReadme, results: &mut Vec<G3CheckResult>) {
    let content = readme.content.as_str();

    if content.len() < 200 {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!("{}: README is a stub", readme.crate_name),
            format!(
                "README at `{}` is only {} bytes. Add meaningful content to the README.",
                readme.readme_rel_path,
                content.len()
            ),
            Some(readme.readme_rel_path.clone()),
            None,
        ));
        return;
    }

    if !has_markdown_heading(content) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!("{}: README has no heading", readme.crate_name),
            format!(
                "README at `{}` has no markdown heading. Add a markdown heading (for example `# Crate Name`).",
                readme.readme_rel_path
            ),
            Some(readme.readme_rel_path.clone()),
            None,
        ));
        return;
    }

    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            format!("{}: README quality looks good", readme.crate_name),
            format!(
                "README at `{}` has content and headings.",
                readme.readme_rel_path
            ),
            Some(readme.readme_rel_path.clone()),
            None,
        )
        .into_inventory(),
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

#[cfg(test)]
#[path = "readme_quality_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod readme_quality_tests;
