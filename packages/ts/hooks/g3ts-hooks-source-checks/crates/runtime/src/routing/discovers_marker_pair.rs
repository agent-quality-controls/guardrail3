use g3ts_hooks_types::G3TsHooksSourceChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Result identifier for the marker-pair routing rule.
const ID: &str = "g3ts-hooks/routing-discovers-marker-pair";

/// Records a finding when the hook lacks discovery of both TS adopted-unit markers.
pub(crate) fn check(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let parsed = input.parsed();
    let mut tests_package_json = false;
    let mut tests_g3ts_marker = false;

    for line in &parsed.source_lines {
        let trimmed = line.raw.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        let raw = line.raw.as_str();
        if line_tests_path(raw, "package.json") || line_quotes_filename(raw, "package.json") {
            tests_package_json = true;
        }
        if line_tests_path(raw, "guardrail3-ts.toml")
            || line_quotes_filename(raw, "guardrail3-ts.toml")
        {
            tests_g3ts_marker = true;
        }
    }

    if !tests_package_json && !tests_g3ts_marker {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "no TS marker pair discovery present".to_owned(),
            ".githooks/pre-commit does not discover TS adopted units via the marker pair (`package.json` + `guardrail3-ts.toml`).".to_owned(),
            Some(input.rel_path().to_owned()),
            None,
        ));
        return;
    }

    if !tests_package_json {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "half-adopted TS marker check".to_owned(),
            ".githooks/pre-commit checks for `guardrail3-ts.toml` but never tests for sibling `package.json`. Half-adopted directories must be rejected; both markers must be required for unit discovery.".to_owned(),
            Some(input.rel_path().to_owned()),
            None,
        ));
        return;
    }
    if !tests_g3ts_marker {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "half-adopted TS marker check".to_owned(),
            ".githooks/pre-commit tests for `package.json` but never tests for sibling `guardrail3-ts.toml`. Half-adopted directories must be rejected; both markers must be required for unit discovery.".to_owned(),
            Some(input.rel_path().to_owned()),
            None,
        ));
    }
}

/// Returns true when `line` performs a file-existence test on `filename`.
fn line_tests_path(line: &str, filename: &str) -> bool {
    if !line.contains(filename) {
        return false;
    }
    let test_markers = ["[ -f", "[[ -f", "[ -e", "[[ -e", "test -f", "test -e"];
    if test_markers.iter().any(|marker| line.contains(marker)) {
        return true;
    }
    if line.contains("-name") && line.contains(filename) {
        return true;
    }
    line.contains(filename) && (line.contains(" -f ") || line.contains(" -e "))
}

/// Returns true when `line` contains `filename` inside single or double quotes.
fn line_quotes_filename(line: &str, filename: &str) -> bool {
    let double = format!("\"{filename}\"");
    let single = format!("'{filename}'");
    line.contains(double.as_str()) || line.contains(single.as_str())
}
