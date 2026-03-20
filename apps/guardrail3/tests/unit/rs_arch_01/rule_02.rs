use super::helpers::{
    arch_01_errors, copy_golden, remove_dir, remove_file, run_check, write_file,
};

const RUST_APPS: &[&str] = &["devctl", "backend", "worker"];
const INNER_HEX: &str = "apps/backend/crates/adapters/inbound/mcp/crates";

/// All 4 crates/ locations in the golden fixture.
const ALL_CRATES_DIRS: &[&str] = &[
    "apps/devctl/crates",
    "apps/backend/crates",
    "apps/worker/crates",
    "apps/backend/crates/adapters/inbound/mcp/crates",
];

/// Filter to only Rule 2 errors by title pattern.
/// Rule 2 errors match: "missing .../ directory", "unexpected directory .../", "loose files in .../".
fn rule2_errors<'a>(errors: &'a [&CheckResult]) -> Vec<&'a &'a CheckResult> {
    errors
        .iter()
        .filter(|e| {
            (e.title.contains("missing") && e.title.contains("/ directory"))
                || e.title.contains("unexpected directory")
                || e.title.contains("loose files in")
        })
        .collect()
}

use guardrail3::domain::report::CheckResult;

// ==========================================================================
// Group A: Missing required dirs
// ==========================================================================

#[test]
fn missing_domain_everywhere() {
    let tmp = copy_golden();
    // Remove domain/ from all 4 crates/ dirs
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
    }
    // Also remove domain/ from admin's src/modules/ to verify TS apps are NOT flagged
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (3 outer + 1 inner hex), got {}: {errors:#?}",
        errors.len()
    );
    // Every error should mention "domain"
    for err in &errors {
        assert!(
            err.title.contains("domain"),
            "expected title mentioning 'domain', got: '{}'",
            err.title
        );
        assert!(
            err.title.contains("missing"),
            "expected title mentioning 'missing', got: '{}'",
            err.title
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // No errors should mention admin or landing
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn missing_app_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/app"));
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (3 outer + 1 inner hex), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("app/"),
            "expected title mentioning 'app/', got: '{}'",
            err.title
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn missing_ports_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/ports"));
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (3 outer + 1 inner hex), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("ports"),
            "expected title mentioning 'ports', got: '{}'",
            err.title
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn missing_adapters_everywhere() {
    let tmp = copy_golden();
    // Removing adapters/ from outer backend DESTROYS the inner hex path
    // (it's under backend/crates/adapters/inbound/mcp/crates/).
    // So we only get 3 errors (one per outer app), not 4.
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters"));
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        3,
        "expected 3 errors (outer only; inner hex destroyed when backend adapters/ removed), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("adapters"),
            "expected title mentioning 'adapters', got: '{}'",
            err.title
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn missing_adapters_inner_hex_only() {
    let tmp = copy_golden();
    // Remove adapters/ from the inner hex only
    remove_dir(tmp.path(), &format!("{INNER_HEX}/adapters"));
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 error (inner hex adapters/ only), got {}: {errors:#?}",
        errors.len()
    );
    assert!(
        errors[0].title.contains("adapters"),
        "expected title mentioning 'adapters', got: '{}'",
        errors[0].title
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("mcp/crates"),
        "expected file field referencing mcp/crates, got: '{file}'"
    );
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn missing_two_dirs_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        remove_dir(tmp.path(), &format!("{dir}/ports"));
    }
    // Also break TS admin (should not be flagged)
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    remove_dir(tmp.path(), "apps/admin/src/modules/ports");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        8,
        "expected 8 errors (2 per location * 4 locations), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn missing_all_four_with_gitkeep() {
    let tmp = copy_golden();
    // Strategy: remove all 4 from the 3 outer apps. For devctl and worker, add .gitkeep.
    // For backend, removing adapters/ kills the inner hex path. But we want to test
    // the inner hex too. So handle backend specially:
    // - Remove app/, domain/, ports/ from outer backend
    // - Remove all 4 from inner hex, add .gitkeep there
    // - Keep adapters/ but it's now mostly empty (inner hex stripped)

    // devctl + worker: straightforward
    for app in &["devctl", "worker"] {
        let dir = format!("apps/{app}/crates");
        remove_dir(tmp.path(), &format!("{dir}/adapters"));
        remove_dir(tmp.path(), &format!("{dir}/app"));
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        remove_dir(tmp.path(), &format!("{dir}/ports"));
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
    }
    // backend outer: remove app/, domain/, ports/ (keep adapters/ for inner hex path)
    remove_dir(tmp.path(), "apps/backend/crates/app");
    remove_dir(tmp.path(), "apps/backend/crates/domain");
    remove_dir(tmp.path(), "apps/backend/crates/ports");
    write_file(tmp.path(), "apps/backend/crates/.gitkeep", "");
    // backend inner hex: remove all 4, add .gitkeep
    remove_dir(tmp.path(), &format!("{INNER_HEX}/adapters"));
    remove_dir(tmp.path(), &format!("{INNER_HEX}/app"));
    remove_dir(tmp.path(), &format!("{INNER_HEX}/domain"));
    remove_dir(tmp.path(), &format!("{INNER_HEX}/ports"));
    write_file(tmp.path(), &format!("{INNER_HEX}/.gitkeep"), "");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // Rule 2 errors:
    // - devctl: 4 missing (adapters, app, domain, ports)
    // - worker: 4 missing
    // - backend outer: 3 missing (app, domain, ports) — adapters/ still exists
    // - backend inner hex: 4 missing (adapters, app, domain, ports)
    // Total Rule 2: 15 errors
    // But other rules also fire: check_03 for backend adapters/ missing outbound/,
    // check_05 for empty containers, etc.
    // Filter to only Rule 2 errors (missing required dir at crates/ level)
    let rule2_missing: Vec<_> = errors
        .iter()
        .filter(|e| {
            e.title.contains("missing")
                && (e.title.ends_with("adapters/ directory")
                    || e.title.ends_with("app/ directory")
                    || e.title.ends_with("domain/ directory")
                    || e.title.ends_with("ports/ directory"))
        })
        .collect();
    assert_eq!(
        rule2_missing.len(),
        15,
        "expected 15 Rule 2 missing-dir errors (4+4+3+4), got {}: {rule2_missing:#?}",
        rule2_missing.len()
    );
    for err in &rule2_missing {
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

// ==========================================================================
// Group B: Unexpected dirs
// ==========================================================================

#[test]
fn unexpected_dir_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/utils"))).expect("mkdir");
    }
    // Also add to admin's src/modules/ (should not be flagged)
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/utils")).expect("mkdir");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (1 per crates/ location), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("unexpected") && err.title.contains("utils"),
            "expected title mentioning 'unexpected' and 'utils', got: '{}'",
            err.title
        );
        // File field assertion: unexpected dir file points to the dir itself
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn unexpected_dir_wrong_case() {
    let tmp = copy_golden();
    // On case-insensitive FS (macOS), Domain/ and domain/ are the same path.
    // Detect this: try creating Domain/ and see if it's distinct from domain/.
    let test_path = tmp.path().join(format!("{}/Domain", ALL_CRATES_DIRS[0]));
    std::fs::create_dir_all(&test_path).expect("mkdir");
    let is_case_sensitive = test_path.exists()
        && tmp
            .path()
            .join(format!("{}/domain", ALL_CRATES_DIRS[0]))
            .exists()
        && std::fs::read_dir(tmp.path().join(ALL_CRATES_DIRS[0]))
            .expect("readdir")
            .filter_map(|e| e.ok())
            .filter(|e| {
                let n = e.file_name().to_string_lossy().to_string();
                n == "Domain" || n == "domain"
            })
            .count()
            == 2;
    // Clean up test probe
    if !is_case_sensitive {
        // Case-insensitive: Domain/ == domain/. Skip this test variant.
        // Instead, test with a truly distinct wrong-case name that doesn't collide.
        // Use "DOMAIN" which on case-insensitive FS maps to "domain" — same issue.
        // On case-insensitive FS, we cannot have both. Just verify the check would
        // flag a near-miss name like "domaine" (typo).
        for dir in ALL_CRATES_DIRS {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/domaine"))).expect("mkdir");
        }
        let results = run_check(tmp.path());
        let errors = arch_01_errors(&results);
        assert_eq!(
            errors.len(),
            4,
            "expected 4 errors (domaine/ is unexpected), got {}: {errors:#?}",
            errors.len()
        );
        for err in &errors {
            assert!(
                err.title.contains("domaine"),
                "expected title mentioning 'domaine', got: '{}'",
                err.title
            );
            // File field assertion
            assert!(
                err.file.as_deref().unwrap_or("").contains("crates"),
                "expected file field containing 'crates', got: '{}'",
                err.file.as_deref().unwrap_or("")
            );
        }
    } else {
        // Case-sensitive FS: Domain/ is distinct from domain/
        for dir in ALL_CRATES_DIRS.iter().skip(1) {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/Domain"))).expect("mkdir");
        }
        let results = run_check(tmp.path());
        let errors = arch_01_errors(&results);
        assert_eq!(
            errors.len(),
            4,
            "expected 4 errors (Domain/ is unexpected; domain/ is valid), got {}: {errors:#?}",
            errors.len()
        );
        for err in &errors {
            assert!(
                err.title.contains("Domain"),
                "expected title mentioning 'Domain', got: '{}'",
                err.title
            );
            // File field assertion
            assert!(
                err.file.as_deref().unwrap_or("").contains("crates"),
                "expected file field containing 'crates', got: '{}'",
                err.file.as_deref().unwrap_or("")
            );
        }
    }
    // TS negative assertions (both branches)
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn multiple_unexpected_dirs() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/utils"))).expect("mkdir");
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/helpers"))).expect("mkdir");
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/config"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        12,
        "expected 12 errors (3 unexpected dirs * 4 locations), got {}: {errors:#?}",
        errors.len()
    );
    // Title content assertions
    for err in &errors {
        assert!(
            err.title.contains("unexpected directory"),
            "expected title mentioning 'unexpected directory', got: '{}'",
            err.title
        );
        assert!(
            err.title.contains("utils") || err.title.contains("helpers") || err.title.contains("config"),
            "expected title mentioning utils/helpers/config, got: '{}'",
            err.title
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn unexpected_dir_named_inbound() {
    let tmp = copy_golden();
    // "inbound" is valid deeper in the tree (adapters/inbound) but NOT at crates/ root
    for dir in ALL_CRATES_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/inbound"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (inbound/ is unexpected at crates/ root), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("inbound"),
            "expected title mentioning 'inbound', got: '{}'",
            err.title
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn hidden_dir_unexpected() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/.hidden"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (.hidden/ is unexpected), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains(".hidden"),
            "expected title mentioning '.hidden', got: '{}'",
            err.title
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

// ==========================================================================
// Group C: Loose files
// ==========================================================================

#[test]
fn loose_rs_file_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    // Also add to TS apps (should not be flagged)
    write_file(tmp.path(), "apps/admin/src/modules/mod.rs", "// stray");
    write_file(tmp.path(), "apps/landing/src/mod.rs", "// stray");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (1 per crates/ location), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("loose files"),
            "expected title mentioning 'loose files', got: '{}'",
            err.title
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn loose_cargo_toml_in_crates() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        write_file(
            tmp.path(),
            &format!("{dir}/Cargo.toml"),
            "[package]\nname = \"stray\"",
        );
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (loose Cargo.toml in each crates/), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("loose files"),
            "expected title mentioning 'loose files', got: '{}'",
            err.title
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn loose_gitignore_not_gitkeep() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/.gitignore"), "target/");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (.gitignore is not .gitkeep), got {}: {errors:#?}",
        errors.len()
    );
    // Title content assertions
    for err in &errors {
        assert!(
            err.title.contains("loose files"),
            "expected title mentioning 'loose files', got: '{}'",
            err.title
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn multiple_loose_files() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
        write_file(tmp.path(), &format!("{dir}/lib.rs"), "// stray");
        write_file(tmp.path(), &format!("{dir}/README.md"), "# readme");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // check_loose_files produces 1 error per directory listing ALL bad files
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (1 per dir listing all loose files), got {}: {errors:#?}",
        errors.len()
    );
    // The message (not title) should list all 3 filenames
    for err in &errors {
        assert!(
            err.message.contains("mod.rs"),
            "expected message containing 'mod.rs', got: '{}'",
            err.message
        );
        assert!(
            err.message.contains("lib.rs"),
            "expected message containing 'lib.rs', got: '{}'",
            err.message
        );
        assert!(
            err.message.contains("README.md"),
            "expected message containing 'README.md', got: '{}'",
            err.message
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn gitkeep_allowed() {
    let tmp = copy_golden();
    // Add .gitkeep to all 4 crates/ dirs alongside existing content
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        0,
        "expected 0 errors (.gitkeep is allowed), got {}: {errors:#?}",
        errors.len()
    );
}

#[test]
fn gitkeep_alongside_loose_files() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (mod.rs flagged, .gitkeep not), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.message.contains("mod.rs"),
            "expected message containing 'mod.rs', got: '{}'",
            err.message
        );
        // The message lists bad files before the "Only `.gitkeep` is allowed" template text.
        // Extract the file list portion (before "Only") and verify .gitkeep is not in it.
        let file_list = err
            .message
            .split("Only")
            .next()
            .unwrap_or(&err.message);
        assert!(
            !file_list.contains(".gitkeep"),
            "expected .gitkeep NOT listed as a bad file, got file list: '{file_list}'"
        );
        // File field assertion
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

// ==========================================================================
// Group D: Combinations
// ==========================================================================

#[test]
fn missing_dir_plus_unexpected_dir() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/utils"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        8,
        "expected 8 errors (4 missing + 4 unexpected), got {}: {errors:#?}",
        errors.len()
    );
    // Title content assertions
    let missing: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("domain"))
        .collect();
    let unexpected: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("unexpected") && e.title.contains("utils"))
        .collect();
    assert_eq!(missing.len(), 4, "expected 4 missing domain/ errors, got: {missing:#?}");
    assert_eq!(unexpected.len(), 4, "expected 4 unexpected utils/ errors, got: {unexpected:#?}");
    // File field assertion
    for err in &errors {
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn missing_dir_plus_loose_file() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/ports"));
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        8,
        "expected 8 errors (4 missing + 4 loose), got {}: {errors:#?}",
        errors.len()
    );
    // Title content assertions
    let missing: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("ports"))
        .collect();
    let loose: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("loose files"))
        .collect();
    assert_eq!(missing.len(), 4, "expected 4 missing ports/ errors, got: {missing:#?}");
    assert_eq!(loose.len(), 4, "expected 4 loose file errors, got: {loose:#?}");
    // File field assertion
    for err in &errors {
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn all_three_violations() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/utils"))).expect("mkdir");
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        12,
        "expected 12 errors (4 missing + 4 unexpected + 4 loose), got {}: {errors:#?}",
        errors.len()
    );
    // Title content assertions
    let missing: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("domain"))
        .collect();
    let unexpected: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("unexpected") && e.title.contains("utils"))
        .collect();
    let loose: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("loose files"))
        .collect();
    assert_eq!(missing.len(), 4, "expected 4 missing domain/ errors, got: {missing:#?}");
    assert_eq!(unexpected.len(), 4, "expected 4 unexpected utils/ errors, got: {unexpected:#?}");
    assert_eq!(loose.len(), 4, "expected 4 loose file errors, got: {loose:#?}");
    // File field assertion
    for err in &errors {
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn different_breakage_per_app() {
    let tmp = copy_golden();
    // devctl: missing domain/
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    // worker: unexpected utils/
    std::fs::create_dir_all(tmp.path().join("apps/worker/crates/utils")).expect("mkdir");
    // backend outer: loose mod.rs
    write_file(tmp.path(), "apps/backend/crates/mod.rs", "// stray");
    // inner hex: missing app/
    remove_dir(tmp.path(), &format!("{INNER_HEX}/app"));
    // Also break TS admin (should not be flagged)
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (1 per location), got {}: {errors:#?}",
        errors.len()
    );

    // devctl: missing domain/
    let devctl_err = errors
        .iter()
        .find(|e| e.title.contains("devctl"))
        .unwrap_or_else(|| panic!("expected devctl error, got: {errors:#?}"));
    assert!(
        devctl_err.title.contains("missing") && devctl_err.title.contains("domain"),
        "expected devctl error about missing domain, got: '{}'",
        devctl_err.title
    );

    // worker: unexpected utils/
    let worker_err = errors
        .iter()
        .find(|e| e.title.contains("worker"))
        .unwrap_or_else(|| panic!("expected worker error, got: {errors:#?}"));
    assert!(
        worker_err.title.contains("unexpected") && worker_err.title.contains("utils"),
        "expected worker error about unexpected utils, got: '{}'",
        worker_err.title
    );

    // backend: loose file AND inner hex missing app/ = 2 errors for backend
    let backend_errs: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("backend"))
        .collect();
    assert_eq!(
        backend_errs.len(),
        2,
        "expected 2 backend errors (loose file + inner hex missing app/), got: {backend_errs:#?}"
    );

    // File field assertions
    for err in &errors {
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }

    // No admin or landing errors
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

// ==========================================================================
// Group E: Edge cases
// ==========================================================================

#[test]
fn required_dir_replaced_with_file() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        // Create a FILE named "domain" (not a directory)
        write_file(tmp.path(), &format!("{dir}/domain"), "not a directory");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // Rule 2 effects: list_dir_names skips files, so "domain" (file) is invisible.
    // check_02: missing domain/ = 4 errors. check_loose_files: "domain" file = 4 errors.
    // But other rules also fire: check_05 sees domain/ metadata exists (file),
    // list_dir_names returns empty => "empty container" error. check_06 also fires.
    // We filter to ONLY Rule 2 errors and assert exact count of 8.
    let missing: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("domain"))
        .collect();
    let loose: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("loose files"))
        .collect();
    assert_eq!(
        missing.len(),
        4,
        "expected 4 missing domain/ errors, got: {missing:#?}"
    );
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose file errors, got: {loose:#?}"
    );
    // Filter to only Rule 2 errors by title pattern and assert exact count
    let r2: Vec<_> = rule2_errors(&errors);
    assert_eq!(
        r2.len(),
        8,
        "expected exactly 8 Rule 2 errors (4 missing + 4 loose), got {}: {r2:#?}",
        r2.len()
    );
    // File field assertions
    for err in &missing {
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    for err in &loose {
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn required_dir_replaced_with_symlink() {
    let tmp = copy_golden();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        // Create a symlink named "domain" -> app/ (DirEntry::file_type() does NOT follow symlinks)
        std::os::unix::fs::symlink(
            tmp.path().join(format!("{dir}/app")),
            tmp.path().join(format!("{dir}/domain")),
        )
        .expect("symlink");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // DirEntry::file_type() returns the symlink type, not the target type.
    // On Unix, symlink to dir returns symlink type (not dir). So domain (symlink) won't
    // appear in list_dir_names. check_loose_files checks !ft.is_dir() which is true for
    // symlink, and the name != ".gitkeep", so the symlink WILL be flagged as a loose file.
    // Result: 4 missing domain/ + 4 loose file "domain" = 8 errors.
    assert_eq!(
        errors.len(),
        8,
        "expected 8 errors (4 missing domain/ + 4 loose 'domain' symlink), got {}: {errors:#?}",
        errors.len()
    );
    // Title assertions: 4 missing + 4 loose
    let missing: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("domain"))
        .collect();
    let loose: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("loose files"))
        .collect();
    assert_eq!(
        missing.len(),
        4,
        "expected 4 missing domain/ errors, got: {missing:#?}"
    );
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose file errors, got: {loose:#?}"
    );
    // File field assertions
    for err in &errors {
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn empty_required_dir_still_present() {
    let tmp = copy_golden();
    // Empty out domain/ contents in all 4 (but dir still exists)
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/domain"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // Rule 2 only checks dir exists in list_dir_names — dir is present, so no Rule 2 error.
    // Other rules (check_05, check_06) may fire for empty domain/, but those are separate rules.
    // Assert that NO Rule 2-type errors exist.
    let rule2_errors: Vec<_> = errors
        .iter()
        .filter(|e| {
            e.title.contains("missing") && e.title.contains("crates/")
                && (e.title.contains("/domain/")
                    || e.title.contains("/app/")
                    || e.title.contains("/ports/")
                    || e.title.contains("/adapters/"))
                || e.title.contains("unexpected directory")
                || e.title.contains("loose files in crates/")
        })
        .collect();
    assert_eq!(
        rule2_errors.len(),
        0,
        "expected 0 Rule 2 errors (dir exists even if empty), got: {rule2_errors:#?}"
    );
}

#[test]
fn ts_app_with_cargo_toml_gets_checked() {
    let tmp = copy_golden();
    // Add Cargo.toml to admin (TS app). Now it becomes a "Rust app" for the check.
    write_file(
        tmp.path(),
        "apps/admin/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"",
    );
    // admin has src/ (triggers check_12 src/ ban) and no crates/ (triggers Rule 1)
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // admin should now appear in errors
    let admin_errs: Vec<_> = errors.iter().filter(|e| e.title.contains("admin")).collect();
    // Admin gets Cargo.toml, has no crates/ dir => check_01 fires (1 error "missing crates/").
    // Admin also has src/ which triggers check_12 (1 error "has src/"). Total for admin: 2 errors.
    assert_eq!(
        admin_errs.len(),
        2,
        "expected 2 admin errors (missing crates/ + src/ ban), got {}: {admin_errs:#?}",
        admin_errs.len()
    );
    // Should have both a "missing crates/" error and a "src/" error
    assert!(
        admin_errs.iter().any(|e| e.title.contains("missing crates/")),
        "admin should have 'missing crates/' error, got: {admin_errs:#?}"
    );
    assert!(
        admin_errs.iter().any(|e| e.title.contains("src/")),
        "admin should have 'src/' error (check_12), got: {admin_errs:#?}"
    );
}

#[test]
fn app_without_cargo_toml_skipped() {
    let tmp = copy_golden();
    // Remove Cargo.toml from devctl. Break its crates/ structure.
    remove_file(tmp.path(), "apps/devctl/Cargo.toml");
    // Add an unexpected dir to make sure no errors appear if we DID check
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/utils")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // devctl should NOT be checked since it has no Cargo.toml
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl")),
        "devctl without Cargo.toml should not be checked, got devctl errors in: {errors:#?}"
    );
}

// ==========================================================================
// Group F: Inner hex isolation
// ==========================================================================

#[test]
fn loose_file_inner_hex_only() {
    let tmp = copy_golden();
    // Add mod.rs to ONLY mcp/crates/
    write_file(tmp.path(), &format!("{INNER_HEX}/mod.rs"), "// stray");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 error (loose file in inner hex only), got {}: {errors:#?}",
        errors.len()
    );
    assert!(
        errors[0].title.contains("loose files"),
        "expected title mentioning 'loose files', got: '{}'",
        errors[0].title
    );
    assert!(
        errors[0].file.as_deref().unwrap_or("").contains("mcp/crates"),
        "expected file field referencing mcp/crates, got: '{}'",
        errors[0].file.as_deref().unwrap_or("")
    );
    // Assert outer apps clean: no errors for devctl, worker, or outer backend crates/
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl")),
        "devctl should have no errors, got: {errors:#?}"
    );
    assert!(
        !errors.iter().any(|e| e.title.contains("worker")),
        "worker should have no errors, got: {errors:#?}"
    );
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn unexpected_dir_inner_hex_only() {
    let tmp = copy_golden();
    // Add utils/ to ONLY mcp/crates/
    std::fs::create_dir_all(tmp.path().join(format!("{INNER_HEX}/utils"))).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 error (unexpected dir in inner hex only), got {}: {errors:#?}",
        errors.len()
    );
    assert!(
        errors[0].title.contains("unexpected") && errors[0].title.contains("utils"),
        "expected title mentioning 'unexpected' and 'utils', got: '{}'",
        errors[0].title
    );
    assert!(
        errors[0].file.as_deref().unwrap_or("").contains("mcp/crates"),
        "expected file field referencing mcp/crates, got: '{}'",
        errors[0].file.as_deref().unwrap_or("")
    );
    // Assert outer apps clean
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl")),
        "devctl should have no errors, got: {errors:#?}"
    );
    assert!(
        !errors.iter().any(|e| e.title.contains("worker")),
        "worker should have no errors, got: {errors:#?}"
    );
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn near_miss_dir_names() {
    let tmp = copy_golden();
    // Add near-miss dir names to all 4 locations:
    // "domains/" (extra s), "adapter/" (missing s), "port/" (missing s),
    // "application/" (wrong name, TS name)
    let near_misses = ["domains", "adapter", "port", "application"];
    for dir in ALL_CRATES_DIRS {
        for name in &near_misses {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/{name}"))).expect("mkdir");
        }
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // 4 near-miss dirs * 4 locations = 16 unexpected errors
    let unexpected: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("unexpected directory"))
        .collect();
    assert_eq!(
        unexpected.len(),
        16,
        "expected 16 unexpected errors (4 near-miss dirs * 4 locations), got {}: {unexpected:#?}",
        unexpected.len()
    );
    // Verify each near-miss name appears in errors.
    // Title ends with "{name}/" so use ends_with to avoid substring matches
    // (e.g., "adapter/" as substring of "adapters/" in the label prefix).
    for name in &near_misses {
        let suffix = format!("{name}/");
        let count = unexpected
            .iter()
            .filter(|e| e.title.ends_with(&suffix))
            .count();
        assert_eq!(
            count, 4,
            "expected 4 errors for '{name}/', got {count}: {unexpected:#?}"
        );
    }
    // File field assertions
    for err in &unexpected {
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // TS negative assertions
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}
