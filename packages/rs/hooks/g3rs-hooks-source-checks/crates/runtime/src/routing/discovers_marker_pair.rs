use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::RustHookCommandInput;

/// `ID` constant.
const ID: &str = "g3rs-hooks/routing-discovers-marker-pair";

/// `check` function.
pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let mut tests_cargo_workspace = false;
    let mut tests_g3rs_marker = false;

    for line in &input.parsed.source_lines {
        let trimmed = line.raw.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        let raw = line.raw.as_str();
        if line_tests_path(raw, "Cargo.toml") || line_quotes_filename(raw, "Cargo.toml") {
            tests_cargo_workspace = true;
        }
        if line_tests_path(raw, "guardrail3-rs.toml")
            || line_quotes_filename(raw, "guardrail3-rs.toml")
        {
            tests_g3rs_marker = true;
        }
    }

    if !tests_cargo_workspace && !tests_g3rs_marker {
        // Nothing to evaluate: hook does no Rust marker discovery.
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Error,
                "no Rust marker pair discovery present".to_owned(),
                ".githooks/pre-commit does not discover Rust adopted units via the marker pair (`Cargo.toml` + `guardrail3-rs.toml`).".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            ),
        );
        return;
    }

    if !tests_cargo_workspace {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Error,
            "half-adopted Rust marker check".to_owned(),
            ".githooks/pre-commit checks for `guardrail3-rs.toml` but never tests for sibling `Cargo.toml` with `[workspace]`. Half-adopted directories must be rejected; both markers must be required for unit discovery.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }
    if !tests_g3rs_marker {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Error,
            "half-adopted Rust marker check".to_owned(),
            ".githooks/pre-commit tests for `Cargo.toml` but never tests for sibling `guardrail3-rs.toml`. Half-adopted directories must be rejected; both markers must be required for unit discovery.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }

    results.push(
        G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "marker pair discovery present".to_owned(),
            ".githooks/pre-commit tests for both `Cargo.toml` and sibling `guardrail3-rs.toml` to identify owning adopted units.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        )
        .into_inventory(),
    );
}

/// Returns true if `line` contains a path test (e.g. `[ -f .../<filename> ]`, `test -e ...<filename>`)
/// referencing the given marker filename.
fn line_tests_path(line: &str, filename: &str) -> bool {
    if !line.contains(filename) {
        return false;
    }
    // Heuristic: look for shell test operators near the filename.
    let test_markers = ["[ -f", "[[ -f", "[ -e", "[[ -e", "test -f", "test -e"];
    if test_markers.iter().any(|marker| line.contains(marker)) {
        return true;
    }
    // Also accept a `find ... -name <filename>` pattern.
    if line.contains("-name") && line.contains(filename) {
        return true;
    }
    // Or a plain `if [ -f .../<filename> ]` already covered above. Accept `&&` chains too.
    line.contains(filename) && (line.contains(" -f ") || line.contains(" -e "))
}

/// Returns true if `line` contains the marker filename as a double- or single-quoted
/// string literal. Used to recognise indirection through helper functions where the
/// filename is passed as an argument (e.g. `find_owning_unit "$dir" "Cargo.toml" ...`).
fn line_quotes_filename(line: &str, filename: &str) -> bool {
    let double = format!("\"{filename}\"");
    let single = format!("'{filename}'");
    line.contains(double.as_str()) || line.contains(single.as_str())
}
