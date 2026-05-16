use g3rs_fmt_types::{G3RsFmtConfigFileKind, G3RsFmtFileTreeChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-fmt/per-crate-override";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsFmtFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for file in &input.nested_config_files {
        let kind = match file.kind {
            G3RsFmtConfigFileKind::RustfmtToml => "rustfmt.toml",
            G3RsFmtConfigFileKind::DotRustfmtToml => ".rustfmt.toml",
        };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "Illegal nested rustfmt config".to_owned(),
            format!(
                "`{kind}` below repository root is forbidden; rustfmt policy is root-only. Delete this file and ensure all formatting settings are in the root `rustfmt.toml`."
            ),
            Some(file.rel_path.clone()),
            None,
        ));
    }
}
