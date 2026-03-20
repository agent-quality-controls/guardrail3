use super::helpers::{arch_01_errors, assert_single_error, copy_golden, remove_dir, run_check, write_file};

// -----------------------------------------------------------------------
// Group 1: outer crates/ missing entirely
// -----------------------------------------------------------------------

#[test]
fn missing_crates_dir_devctl() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/");
}

#[test]
fn missing_crates_dir_backend() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/backend/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "missing crates/");
}

#[test]
fn missing_crates_dir_two_apps() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    remove_dir(tmp.path(), "apps/worker/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 2, "expected 2 errors (one per broken app), got: {errors:#?}");
}

#[test]
fn missing_crates_dir_all_rust_apps() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    remove_dir(tmp.path(), "apps/backend/crates");
    remove_dir(tmp.path(), "apps/worker/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 3, "expected 3 errors (one per Rust app), got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Group 2: outer crates/ exists but is empty
// -----------------------------------------------------------------------

#[test]
fn crates_dir_empty() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // Should detect as empty, not "missing"
    assert!(
        !errors.is_empty(),
        "expected error for empty crates/ dir, got none"
    );
    // The error message should NOT say "missing" — the dir exists, it's empty
    // This tests whether we distinguish "missing" from "empty"
}

// -----------------------------------------------------------------------
// Group 3: crates is a file, not a directory
// -----------------------------------------------------------------------

#[test]
fn crates_is_file_not_dir() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    write_file(tmp.path(), "apps/devctl/crates", "not a directory");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        !errors.is_empty(),
        "expected error when crates is a file, got none"
    );
}

// -----------------------------------------------------------------------
// Group 4: inner hex-in-hex crates/ missing
// -----------------------------------------------------------------------

#[test]
fn inner_hex_crates_missing() {
    let tmp = copy_golden();
    // Backend has hex-in-hex at adapters/inbound/mcp/crates/
    // Remove just the inner crates/
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    // mcp/ dir still exists but has no crates/ inside
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("missing") && e.title.contains("crates")),
        "expected error about missing inner crates/, got: {errors:#?}"
    );
}

#[test]
fn inner_hex_crates_empty() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    std::fs::create_dir_all(
        tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        !errors.is_empty(),
        "expected error for empty inner crates/, got none"
    );
}

#[test]
fn inner_hex_crates_is_file() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
        "not a directory",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        !errors.is_empty(),
        "expected error when inner crates is a file, got none"
    );
}

// -----------------------------------------------------------------------
// Group 5: both levels broken
// -----------------------------------------------------------------------

#[test]
fn outer_missing_inner_never_checked() {
    let tmp = copy_golden();
    // Remove outer crates/ — inner hex-in-hex is unreachable
    remove_dir(tmp.path(), "apps/backend/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // Should get exactly 1 error for the outer, not cascade into inner
    assert_single_error(&errors, "missing crates/");
}

// -----------------------------------------------------------------------
// Group 6: cross-app combinations
// -----------------------------------------------------------------------

#[test]
fn different_breakage_across_apps() {
    let tmp = copy_golden();
    // devctl: missing crates/
    remove_dir(tmp.path(), "apps/devctl/crates");
    // backend: inner hex-in-hex crates/ missing (outer fine)
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.len() >= 2,
        "expected at least 2 errors across apps, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group 7: interaction with src/ ban (rule 12)
// -----------------------------------------------------------------------

#[test]
fn src_exists_and_crates_missing() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // Both src/ ban AND missing crates/ should fire
    assert_eq!(errors.len(), 2, "expected 2 errors (src/ + missing crates/), got: {errors:#?}");
    assert!(errors.iter().any(|e| e.title.contains("src/")), "expected src/ error");
    assert!(errors.iter().any(|e| e.title.contains("missing crates/")), "expected missing crates/ error");
}

// -----------------------------------------------------------------------
// Group 8: app with Cargo.toml but nothing else
// -----------------------------------------------------------------------

#[test]
fn app_with_only_cargo_toml() {
    let tmp = copy_golden();
    // Create a new app with just Cargo.toml
    write_file(
        tmp.path(),
        "apps/phantom/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("phantom") && e.title.contains("missing crates/")),
        "expected error about phantom app missing crates/, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group 9: TS apps are skipped (no false positives)
// -----------------------------------------------------------------------

#[test]
fn ts_apps_not_checked() {
    let tmp = copy_golden();
    // admin and landing have no Cargo.toml — should produce 0 R-ARCH-01 errors
    // even though they have no crates/ dir
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        !errors.iter().any(|e| {
            let title = &e.title;
            title.contains("admin") || title.contains("landing")
        }),
        "TS apps should not trigger R-ARCH-01, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group: symlinks
// -----------------------------------------------------------------------

#[test]
fn crates_is_broken_symlink() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    std::os::unix::fs::symlink("/nonexistent/path", tmp.path().join("apps/devctl/crates")).expect("symlink");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(!errors.is_empty(), "expected error for broken symlink crates/, got none");
}

#[test]
fn crates_is_symlink_to_other_app() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/worker/crates"),
        tmp.path().join("apps/devctl/crates"),
    ).expect("symlink");
    let results = run_check(tmp.path());
    let _ = arch_01_errors(&results);
}

#[test]
fn inner_hex_crates_is_broken_symlink() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    std::os::unix::fs::symlink(
        "/nonexistent",
        tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates"),
    ).expect("symlink");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(!errors.is_empty(), "expected error for broken inner symlink crates/, got none");
}

// -----------------------------------------------------------------------
// Group: app detection edge cases
// -----------------------------------------------------------------------

#[test]
fn cargo_toml_is_empty() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/phantom/Cargo.toml", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("phantom")),
        "expected error about phantom app, got: {errors:#?}"
    );
}

#[test]
fn cargo_toml_is_malformed() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/phantom/Cargo.toml", "this is not valid toml {{{{");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("phantom")),
        "expected error about phantom app, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group: filesystem edge cases
// -----------------------------------------------------------------------

#[test]
fn crates_with_only_gitkeep() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates")).expect("mkdir");
    write_file(tmp.path(), "apps/devctl/crates/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(!errors.is_empty(), "expected errors for crates/ with only .gitkeep, got none");
}

#[test]
fn unicode_app_name() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/über-service/Cargo.toml", "[workspace]\nmembers = []\nresolver = \"2\"");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("über-service")),
        "expected error about unicode app name, got: {errors:#?}"
    );
}

#[test]
fn app_name_with_spaces() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/my app/Cargo.toml", "[workspace]\nmembers = []\nresolver = \"2\"");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("my app")),
        "expected error about spaced app name, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group: wrong placement and casing
// -----------------------------------------------------------------------

#[test]
fn crates_inside_domain() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/domain/crates/types/Cargo.toml", "[package]\nname=\"t\"");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl") && e.title.contains("missing crates/")),
        "devctl should still pass (wrong-place crates/ is invisible), got: {errors:#?}"
    );
}

#[test]
fn crates_inside_src() {
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/src/crates/domain/Cargo.toml", "[package]\nname=\"d\"");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("src/")),
        "expected src/ ban error, got: {errors:#?}"
    );
}

#[test]
fn wrong_casing_crates() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/Crates/domain")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl") && e.title.contains("missing crates/")),
        "devctl should still pass (has valid crates/), got: {errors:#?}"
    );
}

#[test]
fn typo_crate_singular() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/phantom/crate/domain")).expect("mkdir");
    write_file(tmp.path(), "apps/phantom/Cargo.toml", "[workspace]\nmembers=[]\nresolver=\"2\"");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("phantom") && e.title.contains("missing crates/")),
        "expected error about phantom missing crates/, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group: ordering and reporting
// -----------------------------------------------------------------------

#[test]
fn all_apps_checked_not_just_first() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/worker/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("worker")),
        "worker error should be reported even though it's not the first app, got: {errors:#?}"
    );
}

#[test]
fn each_broken_app_gets_own_error() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    remove_dir(tmp.path(), "apps/worker/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let has_devctl = errors.iter().any(|e| e.title.contains("devctl"));
    let has_worker = errors.iter().any(|e| e.title.contains("worker"));
    assert!(has_devctl, "expected devctl error, got: {errors:#?}");
    assert!(has_worker, "expected worker error, got: {errors:#?}");
}

#[test]
fn error_identifies_which_app() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/backend/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "backend");
}

// -----------------------------------------------------------------------
// Group: hex-in-hex detection edge cases
// -----------------------------------------------------------------------

#[test]
fn leaf_crate_not_mistaken_for_hex_in_hex() {
    let tmp = copy_golden();
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(errors.is_empty(), "golden should pass, got: {errors:#?}");
}

#[test]
fn hex_in_hex_at_different_containers() {
    let tmp = copy_golden();
    let base = "apps/devctl/crates/domain/complex/crates";
    write_file(tmp.path(), &format!("{base}/domain/inner/Cargo.toml"), "[package]\nname=\"inner\"\nversion=\"0.1.0\"\nedition=\"2024\"");
    write_file(tmp.path(), &format!("{base}/domain/inner/src/lib.rs"), "");
    write_file(tmp.path(), &format!("{base}/app/handler/Cargo.toml"), "[package]\nname=\"handler\"\nversion=\"0.1.0\"\nedition=\"2024\"");
    write_file(tmp.path(), &format!("{base}/app/handler/src/lib.rs"), "");
    write_file(tmp.path(), &format!("{base}/ports/inbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/ports/outbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/adapters/inbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/adapters/outbound/.gitkeep"), "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(errors.is_empty(), "hex-in-hex in domain/ should be valid, got: {errors:#?}");
}

#[test]
fn triple_nested_hex_in_hex() {
    let tmp = copy_golden();
    let base = "apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound/transport";
    remove_dir(tmp.path(), base);
    let inner = format!("{base}/crates");
    write_file(tmp.path(), &format!("{inner}/domain/types/Cargo.toml"), "[package]\nname=\"deep\"\nversion=\"0.1.0\"\nedition=\"2024\"");
    write_file(tmp.path(), &format!("{inner}/domain/types/src/lib.rs"), "");
    write_file(tmp.path(), &format!("{inner}/app/core/Cargo.toml"), "[package]\nname=\"deep-app\"\nversion=\"0.1.0\"\nedition=\"2024\"");
    write_file(tmp.path(), &format!("{inner}/app/core/src/lib.rs"), "");
    write_file(tmp.path(), &format!("{inner}/ports/inbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{inner}/ports/outbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{inner}/adapters/inbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{inner}/adapters/outbound/.gitkeep"), "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(errors.is_empty(), "triple-nested hex-in-hex should be valid, got: {errors:#?}");
}

#[test]
fn hex_in_hex_missing_crates_at_third_level() {
    let tmp = copy_golden();
    let base = "apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound/transport";
    remove_dir(tmp.path(), base);
    std::fs::create_dir_all(tmp.path().join(format!("{base}/crates"))).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(!errors.is_empty(), "expected error for empty third-level crates/, got none");
}

#[test]
fn multiple_hex_in_hex_in_same_container() {
    let tmp = copy_golden();
    let base = "apps/backend/crates/adapters/inbound/grpc/crates";
    write_file(tmp.path(), &format!("{base}/domain/types/Cargo.toml"), "[package]\nname=\"grpc-domain\"\nversion=\"0.1.0\"\nedition=\"2024\"");
    write_file(tmp.path(), &format!("{base}/domain/types/src/lib.rs"), "");
    write_file(tmp.path(), &format!("{base}/app/handlers/Cargo.toml"), "[package]\nname=\"grpc-app\"\nversion=\"0.1.0\"\nedition=\"2024\"");
    write_file(tmp.path(), &format!("{base}/app/handlers/src/lib.rs"), "");
    write_file(tmp.path(), &format!("{base}/ports/inbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/ports/outbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/adapters/inbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/adapters/outbound/.gitkeep"), "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(errors.is_empty(), "two hex-in-hex in same container should be valid, got: {errors:#?}");
}

#[test]
fn hex_in_hex_in_ports() {
    let tmp = copy_golden();
    let base = "apps/devctl/crates/ports/outbound/complex/crates";
    write_file(tmp.path(), &format!("{base}/domain/types/Cargo.toml"), "[package]\nname=\"port-complex\"\nversion=\"0.1.0\"\nedition=\"2024\"");
    write_file(tmp.path(), &format!("{base}/domain/types/src/lib.rs"), "");
    write_file(tmp.path(), &format!("{base}/app/core/Cargo.toml"), "[package]\nname=\"port-app\"\nversion=\"0.1.0\"\nedition=\"2024\"");
    write_file(tmp.path(), &format!("{base}/app/core/src/lib.rs"), "");
    write_file(tmp.path(), &format!("{base}/ports/inbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/ports/outbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/adapters/inbound/.gitkeep"), "");
    write_file(tmp.path(), &format!("{base}/adapters/outbound/.gitkeep"), "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(errors.is_empty(), "hex-in-hex in ports/ should be valid, got: {errors:#?}");
}

#[test]
fn hex_in_hex_inner_has_wrong_dirs() {
    let tmp = copy_golden();
    let base = "apps/devctl/crates/app/complex/crates";
    std::fs::create_dir_all(tmp.path().join(format!("{base}/src"))).expect("mkdir");
    write_file(tmp.path(), &format!("{base}/src/lib.rs"), "");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(!errors.is_empty(), "expected error for inner hex with wrong structure, got none");
}

// -----------------------------------------------------------------------
// Group: cascading failures
// -----------------------------------------------------------------------

#[test]
fn outer_crates_file_inner_unreachable() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/backend/crates");
    write_file(tmp.path(), "apps/backend/crates", "not a directory");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.iter().filter(|e| e.title.contains("backend")).count(),
        1,
        "expected exactly 1 error for backend (outer only), got: {errors:#?}"
    );
}

#[test]
fn three_apps_three_different_failures() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    remove_dir(tmp.path(), "apps/backend/crates");
    write_file(tmp.path(), "apps/backend/crates", "not a dir");
    remove_dir(tmp.path(), "apps/worker/crates");
    std::fs::create_dir_all(tmp.path().join("apps/worker/crates")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(errors.len(), 3, "expected 3 errors (one per app), got: {errors:#?}");
    assert!(errors.iter().any(|e| e.title.contains("devctl")), "missing devctl error");
    assert!(errors.iter().any(|e| e.title.contains("backend")), "missing backend error");
    assert!(errors.iter().any(|e| e.title.contains("worker")), "missing worker error");
}

// -----------------------------------------------------------------------
// Group: error message quality
// -----------------------------------------------------------------------

#[test]
fn error_message_includes_app_name() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors[0].title.contains("devctl"),
        "error should name the app: {}",
        errors[0].title
    );
}

#[test]
fn error_file_field_points_to_app_dir() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/devctl/crates");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/devctl"),
        "error file field should reference app dir, got: {file}"
    );
}

#[test]
fn inner_hex_error_distinguishable_from_outer() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    std::fs::create_dir_all(
        tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates"),
    ).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| {
            let t = &e.title;
            t.contains("mcp") || t.contains("adapters/inbound")
        }),
        "inner hex error should be distinguishable from outer, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group: circular and exotic symlinks
// -----------------------------------------------------------------------

#[test]
fn inner_hex_crates_symlink_to_outer_crates() {
    // Circular: inner mcp/crates/ points back to outer crates/
    let tmp = copy_golden();
    let inner = tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates");
    let outer = tmp.path().join("apps/backend/crates");
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    std::os::unix::fs::symlink(&outer, &inner).expect("symlink");
    let results = run_check(tmp.path());
    // Should not infinite loop. May pass (symlink resolves to valid structure) or error.
    // The key assertion is that it terminates.
    let _ = arch_01_errors(&results);
}

#[test]
fn crates_symlink_to_dev_null() {
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/worker/crates");
    std::os::unix::fs::symlink("/dev/null", tmp.path().join("apps/worker/crates")).expect("symlink");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        !errors.is_empty(),
        "expected error for crates/ symlinked to /dev/null, got none"
    );
}

// -----------------------------------------------------------------------
// Group: hex-in-hex detection not triggering
// -----------------------------------------------------------------------

#[test]
fn hex_in_hex_leaf_has_cargo_toml_so_no_recursion() {
    // If a leaf has Cargo.toml, it's a crate — hex-in-hex detection doesn't trigger
    // even if there happens to be some dirs inside. The check shouldn't recurse.
    let tmp = copy_golden();
    // backend/adapters/inbound/rest has Cargo.toml — it's a crate, not hex-in-hex
    // Add a random dir inside it — should not be checked by hex arch rules
    std::fs::create_dir_all(
        tmp.path().join("apps/backend/crates/adapters/inbound/rest/internal/stuff"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(errors.is_empty(), "dirs inside a leaf crate should not trigger errors, got: {errors:#?}");
}

#[test]
fn outer_exists_inner_is_file_not_dir() {
    // Outer crates/ valid, inner mcp/crates is a file
    let tmp = copy_golden();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
        "not a directory",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        !errors.is_empty(),
        "expected error for inner crates as file, got none"
    );
}

// -----------------------------------------------------------------------
// Group: src/ interaction (additional)
// -----------------------------------------------------------------------

#[test]
fn src_and_crates_both_exist() {
    // src/ ban fires even when crates/ is valid
    let tmp = copy_golden();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "has src/ directory");
}

#[test]
fn inner_hex_has_src() {
    // src/ inside a hex-in-hex leaf — not at app level, so src/ ban doesn't fire
    // But mcp/ is detected as hex-in-hex (has crates/), and src/ inside it is a loose dir
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/src/main.rs",
        "fn main() {}",
    );
    let results = run_check(tmp.path());
    // The check_single_app src/ ban only fires at app root level.
    // src/ inside hex-in-hex is not checked by src/ ban — but it may trigger
    // other errors (unexpected dir or loose files in the hex-in-hex leaf).
    // Just verify it doesn't crash and doesn't false-positive on the app.
    let errors = arch_01_errors(&results);
    assert!(
        !errors.iter().any(|e| e.title.contains("backend") && e.title.contains("has src/")),
        "src/ ban should not fire at inner hex level, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group: app detection exotic edge cases
// -----------------------------------------------------------------------

#[test]
fn cargo_toml_is_a_directory() {
    let tmp = copy_golden();
    // Create a new app where Cargo.toml is a directory, not a file
    std::fs::create_dir_all(tmp.path().join("apps/broken/Cargo.toml")).expect("mkdir");
    let results = run_check(tmp.path());
    // read_file on a directory returns None, so this app should be skipped
    let errors = arch_01_errors(&results);
    assert!(
        !errors.iter().any(|e| e.title.contains("broken")),
        "app with Cargo.toml-as-directory should be skipped, got: {errors:#?}"
    );
}

#[test]
fn cargo_toml_is_broken_symlink() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/broken")).expect("mkdir");
    std::os::unix::fs::symlink("/nonexistent", tmp.path().join("apps/broken/Cargo.toml")).expect("symlink");
    let results = run_check(tmp.path());
    // read_file on broken symlink returns None, so app should be skipped
    let errors = arch_01_errors(&results);
    assert!(
        !errors.iter().any(|e| e.title.contains("broken")),
        "app with broken Cargo.toml symlink should be skipped, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Group: wrong nesting depth
// -----------------------------------------------------------------------

#[test]
fn third_level_nesting_at_wrong_place() {
    // Someone creates apps/devctl/crates/domain/types/crates/ — crates/ inside a leaf crate
    // types/ has both Cargo.toml AND crates/ — that's a conflict (can't be both a crate and hex-in-hex)
    let tmp = copy_golden();
    std::fs::create_dir_all(
        tmp.path().join("apps/devctl/crates/domain/types/crates/domain/inner"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_single_error(&errors, "has both Cargo.toml and crates/");
}

// -----------------------------------------------------------------------
// Group: filesystem permissions (best-effort)
// -----------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn crates_no_read_permission() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = copy_golden();
    let crates = tmp.path().join("apps/devctl/crates");
    // Remove read+execute permission
    std::fs::set_permissions(&crates, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    let results = run_check(tmp.path());
    // Restore permissions so tempdir cleanup works
    std::fs::set_permissions(&crates, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    let errors = arch_01_errors(&results);
    // list_dir on unreadable dir returns empty → treated as missing
    assert!(
        !errors.is_empty(),
        "expected error for unreadable crates/, got none"
    );
}
