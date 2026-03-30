use guardrail3_domain_report::Severity;
use test_support::{create_dir_all, create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn errors_for_uncovered_standalone_package_roots() {
    let tmp = create_temp_dir("rs-clippy-01-standalone-uncovered");
    create_dir_all(&tmp.path().join("packages/shared-types"));
    write_file(
        tmp.path(),
        "packages/shared-types/Cargo.toml",
        "[package]\nname = \"shared-types\"\n",
    );

    let results = run_for_tests(tmp.path());
    let coverage = results
        .iter()
        .filter(|result| result.id == "RS-CLIPPY-01")
        .collect::<Vec<_>>();

    assert_eq!(coverage.len(), 1);
    let result = coverage[0];
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "Rust unit uncovered by clippy.toml");
    assert_eq!(result.file.as_deref(), Some("packages/shared-types"));
    assert_eq!(
        result.message,
        "standalone package root `packages/shared-types` is not covered by any allowed clippy.toml at the validation root, a workspace root, or a standalone package root."
    );
    assert!(!result.inventory);
}
