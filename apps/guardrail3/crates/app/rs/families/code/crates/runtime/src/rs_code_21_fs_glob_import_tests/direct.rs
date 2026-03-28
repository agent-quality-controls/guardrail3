use guardrail3_domain_report::Severity;

use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_21_fs_glob_import::{
    assert_normalized_len, findings,
};

#[test]
fn errors_on_direct_std_fs_glob() {
    let content = "use std::fs::*;\nfn main() {}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_eq!(
        results.len(),
        1,
        "direct glob should produce exactly one hit"
    );
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_grouped_std_fs_glob_import() {
    let content = "use std::{fs::*, io};\nfn main() {}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_public_std_fs_glob_import() {
    let content = "pub use std::fs::*;\nfn main() {}";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_fs_glob_inside_group_with_self() {
    let content = "use std::fs::{self, *};\nfn main() {}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_eq!(
        results.len(),
        1,
        "glob hidden in group under fs must be caught"
    );
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_fs_glob_inside_group_with_named() {
    let content = "use std::fs::{File, *};\nfn main() {}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_eq!(
        results.len(),
        1,
        "glob mixed with named imports under fs must be caught"
    );
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_alias_plus_glob_inside_group() {
    let content = "use std::fs::{File as Alias, *};\nfn main() {}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_eq!(results.len(), 1, "alias-plus-glob must still be caught");
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_nested_grouped_fs_glob_import() {
    let content = "use std::fs::{{*}};\nfn main() {}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_eq!(
        results.len(),
        1,
        "nested-brace glob under std::fs must still be caught"
    );
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_glob_inside_inline_module() {
    let content = "mod internal {\n    use std::fs::*;\n    pub fn read_it() {}\n}\nfn main() {}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_eq!(results.len(), 1, "glob inside inline module must be caught");
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_grouped_glob_inside_inline_module() {
    let content =
        "mod internal {\n    use std::{fs::*, io};\n    pub fn read_it() {}\n}\nfn main() {}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_eq!(
        results.len(),
        1,
        "grouped glob inside inline module must be caught"
    );
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_function_local_glob_import() {
    let content =
        "fn main() {\n    use std::fs::*;\n    let _ = read_to_string(\"Cargo.toml\");\n}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_eq!(
        results.len(),
        1,
        "function-local std::fs glob imports must be caught"
    );
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_extern_crate_std_alias_glob_import() {
    let content = "extern crate std as s;\nuse s::fs::*;\nfn main() {}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_eq!(results.len(), 1, "extern crate alias glob must be caught");
    assert_eq!(results[0].id, "RS-CODE-21");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "std::fs glob import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs::*` glob import bypasses clippy method bans."
    );
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn multiple_glob_imports_in_same_file() {
    let content = "use std::fs::*;\nmod helpers {\n    use std::fs::{self, *};\n}\nfn main() {}";
    let binding = check_source("src/foo.rs", content, false);
    let results = findings(&binding);

    assert_eq!(
        results.len(),
        2,
        "both top-level and inline-module glob imports must be caught"
    );
    assert!(results.iter().all(|r| r.id == "RS-CODE-21"));
    assert!(results.iter().all(|r| r.severity == Severity::Error));
    assert!(results.iter().all(|r| r.title == "std::fs glob import"));
    assert!(results.iter().all(|r| {
        r.message == "Direct `use std::fs::*` glob import bypasses clippy method bans."
    }));
    assert_eq!(
        results.iter().map(|result| result.line).collect::<Vec<_>>(),
        vec![Some(1), Some(3)]
    );
    assert!(
        results
            .iter()
            .all(|r| r.file.as_deref() == Some("src/foo.rs"))
    );
    assert!(results.iter().all(|r| !r.inventory));
}
