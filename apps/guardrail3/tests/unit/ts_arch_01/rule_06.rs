use super::helpers::{arch_errors, copy_fixture, remove_dir, run_check, write_file};

// ============================================================================
// Rule 06: Leaf validity — subdirs in containers must have .ts/.tsx files,
// .gitkeep, or a modules/ dir (hex-in-hex)
// ============================================================================

#[test]
fn subdir_with_ts_files_passes() {
    // Golden already has domain/types/ with index.ts — should pass
    let tmp = copy_fixture();
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let leaf: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("no .ts/.tsx files"))
        .collect();
    assert!(
        leaf.is_empty(),
        "golden should have 0 leaf-validity errors, got: {leaf:#?}"
    );
}

#[test]
fn subdir_with_no_ts_files() {
    let tmp = copy_fixture();
    // Replace domain/types/ contents with a non-TS file
    remove_dir(tmp.path(), "apps/admin/src/modules/domain/types");
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/README.md",
        "# Types",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let leaf: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("no .ts/.tsx files"))
        .collect();
    assert_eq!(
        leaf.len(),
        1,
        "expected 1 leaf-validity error, got {}: {leaf:#?}",
        leaf.len()
    );
    assert!(
        leaf[0].title.contains("types"),
        "expected error to mention 'types' subdir, got: '{}'",
        leaf[0].title
    );
}

#[test]
fn subdir_with_tsx_files_passes() {
    let tmp = copy_fixture();
    // Replace domain/types/ with .tsx files
    remove_dir(tmp.path(), "apps/admin/src/modules/domain/types");
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/UserCard.tsx",
        "export default function UserCard() { return <div/>; }",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let leaf: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("no .ts/.tsx files"))
        .collect();
    assert!(
        leaf.is_empty(),
        ".tsx files should satisfy leaf validity, got: {leaf:#?}"
    );
}

#[test]
fn subdir_with_gitkeep_passes() {
    let tmp = copy_fixture();
    // Replace domain/types/ with just .gitkeep
    remove_dir(tmp.path(), "apps/admin/src/modules/domain/types");
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/.gitkeep",
        "",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let leaf: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("no .ts/.tsx files") && e.title.contains("types"))
        .collect();
    assert!(
        leaf.is_empty(),
        ".gitkeep should satisfy leaf validity, got: {leaf:#?}"
    );
}

#[test]
fn empty_subdir_fails() {
    let tmp = copy_fixture();
    // Add an empty subdir to domain/
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/domain/events"))
        .expect("create empty subdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let leaf: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("no .ts/.tsx files") && e.title.contains("events"))
        .collect();
    assert_eq!(
        leaf.len(),
        1,
        "expected 1 error for empty events/ subdir, got {}: {leaf:#?}",
        leaf.len()
    );
}

#[test]
fn hex_in_hex_triggers_recursion() {
    let tmp = copy_fixture();
    // Add a hex-in-hex inside adapters/inbound/mcp/
    let base = "apps/admin/src/modules/adapters/inbound/mcp/modules";
    write_file(
        tmp.path(),
        &format!("{base}/domain/types/index.ts"),
        "export type MCP = {};",
    );
    write_file(
        tmp.path(),
        &format!("{base}/application/commands/run.ts"),
        "export function run() {}",
    );
    write_file(tmp.path(), &format!("{base}/adapters/inbound/.gitkeep"), "");
    write_file(
        tmp.path(),
        &format!("{base}/adapters/outbound/.gitkeep"),
        "",
    );
    write_file(tmp.path(), &format!("{base}/ports/inbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/ports/outbound/.gitkeep"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // Should have no errors — valid hex-in-hex
    let mcp_errors: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("mcp") || e.file.as_deref().unwrap_or("").contains("mcp"))
        .collect();
    assert!(
        mcp_errors.is_empty(),
        "valid hex-in-hex should produce 0 errors, got: {mcp_errors:#?}"
    );
}

#[test]
fn hex_in_hex_missing_layer() {
    let tmp = copy_fixture();
    // Add hex-in-hex but missing domain/
    let base = "apps/admin/src/modules/adapters/inbound/mcp/modules";
    // No domain/ — should error
    write_file(
        tmp.path(),
        &format!("{base}/application/commands/run.ts"),
        "export function run() {}",
    );
    write_file(tmp.path(), &format!("{base}/adapters/inbound/.gitkeep"), "");
    write_file(
        tmp.path(),
        &format!("{base}/adapters/outbound/.gitkeep"),
        "",
    );
    write_file(tmp.path(), &format!("{base}/ports/inbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/ports/outbound/.gitkeep"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("missing") && e.title.contains("domain")),
        "expected error about missing domain/ in hex-in-hex, got: {errors:#?}"
    );
}

#[test]
fn nested_ts_files_satisfy_leaf() {
    let tmp = copy_fixture();
    // Add deeply nested .ts file in a new subdir
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/events/user/created.ts",
        "export class UserCreated {}",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let leaf: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("no .ts/.tsx files") && e.title.contains("events"))
        .collect();
    assert!(
        leaf.is_empty(),
        "nested .ts files should satisfy leaf check, got: {leaf:#?}"
    );
}
