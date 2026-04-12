use g3rs_fmt_filetree_checks_types::{G3RsFmtConfigFileKind, G3RsFmtFileTreeChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-FMT-FILETREE-05";

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

#[cfg(test)]
#[path = "rs_fmt_filetree_05_per_crate_override_tests/mod.rs"]
mod tests;
