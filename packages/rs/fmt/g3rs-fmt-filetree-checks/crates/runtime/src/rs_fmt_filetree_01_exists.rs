use g3rs_fmt_filetree_checks_types::G3RsFmtFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-FMT-FILETREE-01";

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

#[cfg(test)]
#[path = "rs_fmt_filetree_01_exists_tests/mod.rs"]
mod tests;
