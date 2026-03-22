use crate::domain::report::Severity;

use super::super::inputs::RustfmtDualConflictInput;
use super::check;

#[test]
fn reports_dual_root_config_conflicts() {
    let input = RustfmtDualConflictInput { dir_rel: "" };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-FMT-08"
            && result.severity == Severity::Warn
            && result.title == "Conflicting rustfmt config files"
            && result.message == "Both rustfmt.toml and .rustfmt.toml exist in the same directory"
            && result.file.as_deref() == Some("rustfmt.toml")
    }));
}
