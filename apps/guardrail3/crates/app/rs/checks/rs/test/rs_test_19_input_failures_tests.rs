use crate::domain::report::Severity;

use super::super::test_support::{
    StubToolChecker, dir_entry, failure_input, project_tree, temp_root, write_file,
};
use super::check;

#[test]
fn errors_on_direct_failure() {
    let input = failure_input("Cargo.toml", "Failed to parse Cargo.toml");
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-19");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn family_surfaces_source_parse_failures() {
    let root = temp_root("rs-test-19-source-parse");
    write_file(&root, "src/lib.rs", "#[test]\nfn broken( {");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["src"], &["Cargo.toml"])),
            ("src", dir_entry(&[], &["lib.rs"])),
        ],
        vec![(
            "Cargo.toml",
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
        )],
        root.clone(),
    );

    let results = crate::app::rs::checks::rs::test::check(&tree, &StubToolChecker::new(true));
    assert!(results.iter().any(|result| {
        result.id == "RS-TEST-19"
            && result.file.as_deref() == Some("src/lib.rs")
            && result.message.contains("Failed to parse Rust source file")
    }));

    std::fs::remove_dir_all(root).expect("cleanup");
}
