use g3rs_fmt_types::G3RsFmtFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-fmt/rustfmt-config-exists";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsFmtFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.root_rustfmt_toml_rel_path.is_some() || input.root_dot_rustfmt_toml_rel_path.is_some()
    {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "rustfmt config missing".to_owned(),
        "Expected `rustfmt.toml` at workspace root. Create one with the required formatting settings.".to_owned(),
        Some("rustfmt.toml".to_owned()),
        None,
    ));
}
