use crate::domain::report::{CheckResult, Severity};

use super::facts::RustfmtConfigKind;
use super::inputs::RustfmtExtraConfigInput;

const ID: &str = "RS-FMT-05";

pub fn check(input: &RustfmtExtraConfigInput<'_>, results: &mut Vec<CheckResult>) {
    let kind = match input.config_kind {
        RustfmtConfigKind::RustfmtToml => "rustfmt.toml",
        RustfmtConfigKind::DotRustfmtToml => ".rustfmt.toml",
    };

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "Per-crate rustfmt override".to_owned(),
        message: format!("{kind} below workspace root overrides root formatting policy"),
        file: Some(input.config_rel.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_fmt_05_per_crate_override_tests.rs"]
mod tests;
