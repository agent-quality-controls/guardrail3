use std::collections::BTreeSet;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};
use super::super::check;

#[test]
fn skips_same_line_reason_documented_attrs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\n#[deny(clippy::panic)] // reason: domain models stay panic free\nfn documented_probe() {{}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_22_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-22")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-22"), BTreeSet::new());
    assert!(rs_code_22_results.is_empty());
}

#[test]
fn skips_reason_variants_on_same_line() {
    let content = "#![deny(clippy::panic, clippy::expect_used)] // reason: root policy is documented\n#[deny(clippy::panic)] // REASON: keep this API panic free\nfn one() {}\n#[forbid(clippy::expect_used)] //reason: worker retries handle fallibility\nfn two() {}\n";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().all(|result| result.id != "RS-CODE-22"));
}

#[test]
fn inventories_forbid_unsafe_code_even_with_reason() {
    let content = "#![forbid(unsafe_code)] // reason: this crate must stay safe\nfn one() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-22");
    assert_eq!(results[0].severity, guardrail3_domain_report::Severity::Info);
    assert_eq!(results[0].title, "forbid(unsafe_code)");
    assert!(results[0].inventory);
}

#[test]
fn does_not_treat_block_comment_as_same_line_reason() {
    let content = "#[deny(clippy::panic)] /* reason: not supported */\nfn one() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    let rs_code_22_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-22")
        .collect::<Vec<_>>();
    assert_eq!(rs_code_22_results.len(), 1);
}

#[test]
fn empty_or_wrong_key_reason_comments_do_not_suppress() {
    let content = "#[deny(clippy::panic)] // reason:\nfn one() {}\n#[forbid(clippy::expect_used)] // because: not the accepted key\nfn two() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    let rs_code_22_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-22")
        .collect::<Vec<_>>();
    assert_eq!(rs_code_22_results.len(), 2);
}

#[test]
fn ignores_tests_fixture_files_even_with_undocumented_policy_attrs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    write_file(
        root,
        "apps/backend/crates/app/queries/tests/fixtures/lint_policy.rs",
        "#[deny(clippy::panic)]\nfn fixture_probe() {}\n",
    );

    let results = run_family(root);
    let rs_code_22_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-22")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-22"), BTreeSet::new());
    assert!(rs_code_22_results.is_empty());
}

#[test]
fn skips_multiline_attr_with_reason_on_closing_line() {
    let content = "#[deny(\n    clippy::panic,\n    clippy::expect_used\n)] // reason: local boundary hardening\nfn one() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().all(|result| result.id != "RS-CODE-22"));
}
