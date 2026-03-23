use super::super::super::test_support::{copy_fixture, run_family};
use crate::domain::report::Severity;

#[test]
fn unparsable_ports_source_warns_in_family_run() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/src/lib.rs"),
        "pub trait Repo {\n",
    )
    .expect("write broken ports source");

    let results = run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();
    assert_eq!(
        warnings.len(),
        1,
        "expected one source-analysis warning: {warnings:#?}"
    );
    assert_eq!(warnings[0].severity, Severity::Warn);
    assert_eq!(
        warnings[0].file.as_deref(),
        Some("apps/backend/crates/ports/outbound/repo/src/lib.rs")
    );
    assert!(
        warnings[0]
            .message
            .contains("Failed to parse Rust source file")
    );
}
