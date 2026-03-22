use crate::domain::report::Severity;

use super::super::facts::RustfmtConfigKind;
use super::super::inputs::RustfmtExtraConfigInput;
use super::check;

#[test]
fn reports_extra_per_crate_rustfmt_configs() {
    let input = RustfmtExtraConfigInput {
        config_rel: "crates/core/.rustfmt.toml",
        config_kind: RustfmtConfigKind::DotRustfmtToml,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-FMT-05"
            && result.severity == Severity::Warn
            && result.title == "Per-crate rustfmt override"
            && result.message == ".rustfmt.toml below workspace root overrides root formatting policy"
            && result.file.as_deref() == Some("crates/core/.rustfmt.toml")
    }));
}
