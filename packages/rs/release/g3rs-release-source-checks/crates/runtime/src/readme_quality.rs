use g3rs_release_types::G3RsReleaseSourceReadme;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3rs-release/readme-quality";

/// Validates that a crate README is non-stub and has at least one heading.
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

/// Returns true when `content` contains at least one ATX or setext markdown heading.
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
        match classify_line(trimmed, last_text_line_can_be_setext_heading) {
            LineClass::HeadingFound => return true,
            LineClass::Skip => {
                last_text_line_can_be_setext_heading = false;
            }
            LineClass::TextLine => {
                last_text_line_can_be_setext_heading = !trimmed.is_empty();
            }
        }
    }

    false
}

/// Outcome of classifying a single non-fenced, non-indented markdown line.
enum LineClass {
    /// Line is itself a heading or completes a setext heading.
    HeadingFound,
    /// Line should not be considered as a setext heading anchor.
    Skip,
    /// Line is plain text and may anchor a following setext underline.
    TextLine,
}

/// Classifies one trimmed markdown line for heading detection.
fn classify_line(trimmed: &str, last_text_line_can_be_setext_heading: bool) -> LineClass {
    if let Some(after_hashes) = trimmed.strip_prefix('#') {
        return classify_hash_prefixed(trimmed, after_hashes);
    }
    if trimmed == "#" {
        return LineClass::HeadingFound;
    }
    if !trimmed.is_empty() && trimmed.chars().all(|ch| ch == '=' || ch == '-') {
        if last_text_line_can_be_setext_heading {
            return LineClass::HeadingFound;
        }
        return LineClass::Skip;
    }
    LineClass::TextLine
}

/// Classifies a line that begins with `#`.
fn classify_hash_prefixed(trimmed: &str, after_hashes: &str) -> LineClass {
    if after_hashes.starts_with('#') {
        let heading_text = trimmed.trim_start_matches('#');
        if heading_text.starts_with(char::is_whitespace) {
            return LineClass::HeadingFound;
        }
        return LineClass::Skip;
    }
    if after_hashes.starts_with(char::is_whitespace) {
        return LineClass::HeadingFound;
    }
    LineClass::Skip
}

#[cfg(test)]
#[path = "readme_quality_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod readme_quality_tests;
