use std::collections::BTreeSet;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};
use super::super::check;

#[test]
fn skips_test_files_and_src_fs_rs_exemption() {
    let fixture = copy_fixture();
    let root = fixture.path();

    write_file(
        root,
        "apps/backend/crates/app/commands/tests/fs_glob.rs",
        "use std::fs::*;\n#[test]\nfn smoke() {}\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/commands/test/fs_glob_test.rs",
        "use std::fs::*;\nfn smoke() {}\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/commands/__tests__/fs_glob.rs",
        "use std::fs::*;\nfn smoke() {}\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/commands/src/fs_glob_test.rs",
        "use std::fs::*;\nfn smoke() {}\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/commands/src/fs_glob_tests.rs",
        "use std::fs::*;\nfn smoke() {}\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/commands/src/tests.rs",
        "use std::fs::*;\npub fn smoke() {}\n",
    );
    write_file(
        root,
        "apps/backend/crates/app/commands/src/fs.rs",
        "use std::fs::*;\npub fn allowed_probe() {}\n",
    );

    let results = run_family(root);
    let rs_code_21_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-21")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-21"), BTreeSet::new());
    assert!(rs_code_21_results.is_empty());
}

#[test]
fn no_hit_on_non_glob_fs_imports() {
    let content = "use std::fs::File;\nuse std::fs::{self};\nuse std::fs::{File, Read};\nuse std::{fs::{self, File as Alias}, io};\nfn main() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(
        results.len(),
        0,
        "non-glob fs imports must not trigger RS-CODE-21"
    );
    assert!(results.iter().all(|result| result.id != "RS-CODE-21"));
}

#[test]
fn no_hit_on_non_std_fs_glob() {
    let content = "use other_crate::fs::*;\nuse mylib::fs::*;\nfn main() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(
        results.len(),
        0,
        "glob imports from non-std crates must not trigger"
    );
    assert!(results.iter().all(|result| result.id != "RS-CODE-21"));
}

#[test]
fn no_hit_on_std_non_fs_glob() {
    let content = "use std::io::*;\nuse std::collections::*;\nfn main() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(
        results.len(),
        0,
        "glob imports from std but not fs must not trigger"
    );
    assert!(results.iter().all(|result| result.id != "RS-CODE-21"));
}

#[test]
fn no_hit_on_cfg_test_guarded_glob() {
    let content = "#[cfg(test)]\nuse std::fs::*;\nfn main() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 0, "cfg(test) guarded glob must not trigger");
    assert!(results.iter().all(|result| result.id != "RS-CODE-21"));
}

#[test]
fn no_hit_on_cfg_all_test_guarded_glob() {
    let content = "#[cfg(all(test, unix))]\nuse std::fs::*;\nfn main() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(
        results.len(),
        0,
        "cfg(all(test, ...)) glob must not trigger"
    );
    assert!(results.iter().all(|result| result.id != "RS-CODE-21"));
}

#[test]
fn no_hit_on_cfg_test_module_containing_glob() {
    let content =
        "#[cfg(test)]\nmod tests {\n    use std::fs::*;\n    fn probe() {}\n}\nfn main() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(
        results.len(),
        0,
        "glob inside cfg(test) module must not trigger"
    );
    assert!(results.iter().all(|result| result.id != "RS-CODE-21"));
}

#[test]
fn no_hit_on_cfg_all_test_module_containing_glob() {
    let content = "#[cfg(all(test, unix))]\nmod tests {\n    use std::fs::*;\n    fn probe() {}\n}\nfn main() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(
        results.len(),
        0,
        "glob inside cfg(all(test, ...)) module must not trigger"
    );
    assert!(results.iter().all(|result| result.id != "RS-CODE-21"));
}

#[test]
fn no_hit_on_cfg_all_test_function_containing_glob() {
    let content = "#[cfg(all(test, unix))]\nfn helper() {\n    use std::fs::*;\n    let _ = read_to_string(\"fixture\");\n}\nfn main() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(
        results.len(),
        0,
        "glob inside cfg(all(test, ...)) function must not trigger"
    );
    assert!(results.iter().all(|result| result.id != "RS-CODE-21"));
}

#[test]
fn no_hit_on_test_file_with_inline_module_glob() {
    let content = "mod helpers {\n    use std::fs::*;\n}\n#[test]\nfn smoke() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: true,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 0, "test files are fully exempt");
    assert!(results.iter().all(|result| result.id != "RS-CODE-21"));
}
