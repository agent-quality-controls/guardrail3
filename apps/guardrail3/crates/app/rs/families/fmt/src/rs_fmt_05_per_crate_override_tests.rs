use guardrail3_domain_report::Severity;

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

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-FMT-05");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "Per-crate rustfmt override");
    assert_eq!(
        result.message,
        ".rustfmt.toml below workspace root overrides root formatting policy"
    );
    assert_eq!(result.file.as_deref(), Some("crates/core/.rustfmt.toml"));
}
