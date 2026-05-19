use g3rs_fmt_types::G3RsFmtConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-fmt/ignore-escape-hatch";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsFmtConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) = &input.rustfmt_state else {
        return;
    };
    if rustfmt.ignore.is_empty() {
        return;
    }

    let ignore = format!("{:?}", rustfmt.ignore);
    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "rustfmt ignore missing reason".to_owned(),
            format!(
                "`{}` uses `ignore = {ignore}`. Add a waiver entry in guardrail3-rs.toml with rule = \"g3rs-fmt/ignore-escape-hatch\", subject = \"{}\", selector = \"ignore\", and a reason explaining why these paths are excluded.",
                input.rustfmt_rel_path, input.rustfmt_rel_path
            ),
            Some(input.rustfmt_rel_path.clone()),
            None,
        )
        .with_selector("ignore"),
    );

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        "rustfmt ignore count".to_owned(),
        format!("`{}` has 1 rustfmt ignore waiver.", input.rustfmt_rel_path),
        Some(input.rustfmt_rel_path.clone()),
        None,
    ));
}
