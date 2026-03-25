use guardrail3_domain_report::Severity;

use super::super::inputs::RustfmtDualConflictInput;
use super::check;

#[test]
fn reports_dual_root_config_conflicts() {
    let input = RustfmtDualConflictInput { dir_rel: "" };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-FMT-08");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "Conflicting rustfmt config files");
    assert_eq!(
        result.message,
        "Both rustfmt.toml and .rustfmt.toml exist in the same directory"
    );
    assert_eq!(result.file.as_deref(), Some("rustfmt.toml"));
}
