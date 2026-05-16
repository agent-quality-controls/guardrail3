use g3rs_fmt_types::G3RsFmtFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-fmt/dual-file-conflict";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsFmtFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for dir_rel in &input.dual_conflict_dirs {
        let file = if dir_rel.is_empty() {
            "rustfmt.toml".to_owned()
        } else {
            format!("{dir_rel}/rustfmt.toml")
        };
        let dir_display = if dir_rel.is_empty() {
            ".".to_owned()
        } else {
            dir_rel.clone()
        };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "Conflicting rustfmt config files".to_owned(),
            format!(
                "Both `rustfmt.toml` and `.rustfmt.toml` exist in `{dir_display}`. Delete `.rustfmt.toml` and keep `rustfmt.toml`."
            ),
            Some(file),
            None,
        ));
    }
}
