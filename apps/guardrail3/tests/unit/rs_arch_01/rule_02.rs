use super::helpers::{
    INNER_HEX, RUST_APPS, arch_errors, assert_inner_hex, assert_no_packages, assert_no_ts_apps,
    assert_per_app, copy_fixture, remove_dir, remove_file, run_check, write_file,
};
use guardrail3::domain::report::CheckResult;

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

/// Assert every error has a file field containing "crates".
fn assert_file_field_crates(errors: &[&CheckResult]) {
    for err in errors {
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
}

// ============================================================================
// Group A: Missing required dirs
// ============================================================================

#[test]
fn missing_domain_everywhere() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
    }
    // Also remove domain/ from admin's src/modules/ to verify TS apps are NOT flagged
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (3 outer + 1 inner hex), got {}: {errors:#?}",
        errors.len()
    );
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
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_app_everywhere() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/app"));
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_ports_everywhere() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/ports"));
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_adapters_everywhere() {
    let tmp = copy_fixture();
    // Removing adapters/ from outer backend DESTROYS the inner hex path
    // (it's under backend/crates/adapters/inbound/mcp/crates/).
    // So we only get 3 errors (one per outer app), not 4.
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters"));
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    // No inner hex check: removing adapters/ destroys the path
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_adapters_inner_hex_only() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{INNER_HEX}/adapters"));
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_two_dirs_everywhere() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        remove_dir(tmp.path(), &format!("{dir}/ports"));
    }
    // Also break TS admin (should not be flagged)
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    remove_dir(tmp.path(), "apps/admin/src/modules/ports");

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        8,
        "expected 8 errors (2 per location * 4 locations), got {}: {errors:#?}",
        errors.len()
    );
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_all_four_with_gitkeep() {
    let tmp = copy_fixture();
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
    let errors = arch_errors(&results);
    // Rule 2 errors:
    // - devctl: 4 missing (adapters, app, domain, ports)
    // - worker: 4 missing
    // - backend outer: 3 missing (app, domain, ports) — adapters/ still exists
    // - backend inner hex: 4 missing (adapters, app, domain, ports)
    // Total Rule 2: 15 errors
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
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    // Per-app attribution
    for app in RUST_APPS {
        assert!(
            rule2_missing.iter().any(|e| e.title.contains(app)),
            "expected missing-dir error for app `{app}`, got: {rule2_missing:#?}"
        );
    }
    // Inner hex attribution
    assert!(
        rule2_missing
            .iter()
            .any(|e| e.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "expected at least one error from inner hex (mcp/crates), got: {rule2_missing:#?}"
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// Group B: Unexpected dirs
// ============================================================================

#[test]
fn unexpected_dir_everywhere() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/utils"))).expect("mkdir");
    }
    // Also add to admin's src/modules/ (should not be flagged)
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/utils")).expect("mkdir");

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn unexpected_dir_wrong_case() {
    let tmp = copy_fixture();
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
        // Use a truly distinct wrong-case name that doesn't collide: "domaine" (typo).
        for dir in ALL_CRATES_DIRS {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/domaine"))).expect("mkdir");
        }
        let results = run_check(tmp.path());
        let errors = arch_errors(&results);
        assert_eq!(
            errors.len(),
            4,
            "expected 4 errors (domaine/ is unexpected), got {}: {errors:#?}",
            errors.len()
        );
        for err in &errors {
            assert!(
                err.title.contains("unexpected") && err.title.contains("domaine"),
                "expected title mentioning 'unexpected' and 'domaine', got: '{}'",
                err.title
            );
        }
        assert_file_field_crates(&errors);
        assert_per_app(&errors);
        assert_inner_hex(&errors);
        assert_no_packages(&errors);
    } else {
        // Case-sensitive FS: Domain/ is distinct from domain/
        for dir in ALL_CRATES_DIRS.iter().skip(1) {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/Domain"))).expect("mkdir");
        }
        let results = run_check(tmp.path());
        let errors = arch_errors(&results);
        assert_eq!(
            errors.len(),
            4,
            "expected 4 errors (Domain/ is unexpected; domain/ is valid), got {}: {errors:#?}",
            errors.len()
        );
        for err in &errors {
            assert!(
                err.title.contains("unexpected") && err.title.contains("Domain"),
                "expected title mentioning 'unexpected' and 'Domain', got: '{}'",
                err.title
            );
        }
        assert_file_field_crates(&errors);
        assert_per_app(&errors);
        assert_inner_hex(&errors);
        assert_no_packages(&errors);
    }
    // TS negative assertions (both branches)
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn multiple_unexpected_dirs() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/utils"))).expect("mkdir");
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/helpers"))).expect("mkdir");
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/config"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        12,
        "expected 12 errors (3 unexpected dirs * 4 locations), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("unexpected directory"),
            "expected title mentioning 'unexpected directory', got: '{}'",
            err.title
        );
        assert!(
            err.title.contains("utils")
                || err.title.contains("helpers")
                || err.title.contains("config"),
            "expected title mentioning utils/helpers/config, got: '{}'",
            err.title
        );
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn unexpected_dir_named_inbound() {
    let tmp = copy_fixture();
    // "inbound" is valid deeper in the tree (adapters/inbound) but NOT at crates/ root
    for dir in ALL_CRATES_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/inbound"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (inbound/ is unexpected at crates/ root), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("unexpected") && err.title.contains("inbound"),
            "expected title mentioning 'unexpected' and 'inbound', got: '{}'",
            err.title
        );
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn hidden_dir_unexpected() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/.hidden"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (.hidden/ is unexpected), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("unexpected") && err.title.contains(".hidden"),
            "expected title mentioning 'unexpected' and '.hidden', got: '{}'",
            err.title
        );
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// Group C: Loose files
// ============================================================================

#[test]
fn loose_rs_file_everywhere() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    // Also add to TS apps (should not be flagged)
    write_file(tmp.path(), "apps/admin/src/modules/mod.rs", "// stray");
    write_file(tmp.path(), "apps/landing/src/mod.rs", "// stray");

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn loose_cargo_toml_in_crates() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        write_file(
            tmp.path(),
            &format!("{dir}/Cargo.toml"),
            "[package]\nname = \"stray\"",
        );
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn loose_gitignore_not_gitkeep() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/.gitignore"), "target/");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        4,
        "expected 4 errors (.gitignore is not .gitkeep), got {}: {errors:#?}",
        errors.len()
    );
    for err in &errors {
        assert!(
            err.title.contains("loose files"),
            "expected title mentioning 'loose files', got: '{}'",
            err.title
        );
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn multiple_loose_files() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
        write_file(tmp.path(), &format!("{dir}/lib.rs"), "// stray");
        write_file(tmp.path(), &format!("{dir}/README.md"), "# readme");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn gitkeep_allowed() {
    let tmp = copy_fixture();
    // Add .gitkeep to all 4 crates/ dirs alongside existing content
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        0,
        "expected 0 errors (.gitkeep is allowed), got {}: {errors:#?}",
        errors.len()
    );
}

#[test]
fn gitkeep_alongside_loose_files() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
        let file_list = err.message.split("Only").next().unwrap_or(&err.message);
        assert!(
            !file_list.contains(".gitkeep"),
            "expected .gitkeep NOT listed as a bad file, got file list: '{file_list}'"
        );
    }
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// Group D: Combinations
// ============================================================================

#[test]
fn missing_dir_plus_unexpected_dir() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/utils"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        8,
        "expected 8 errors (4 missing + 4 unexpected), got {}: {errors:#?}",
        errors.len()
    );
    let missing: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("domain"))
        .collect();
    let unexpected: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("unexpected") && e.title.contains("utils"))
        .collect();
    assert_eq!(
        missing.len(),
        4,
        "expected 4 missing domain/ errors, got: {missing:#?}"
    );
    assert_eq!(
        unexpected.len(),
        4,
        "expected 4 unexpected utils/ errors, got: {unexpected:#?}"
    );
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_dir_plus_loose_file() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/ports"));
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        8,
        "expected 8 errors (4 missing + 4 loose), got {}: {errors:#?}",
        errors.len()
    );
    let missing: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("ports"))
        .collect();
    let loose: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("loose files"))
        .collect();
    assert_eq!(
        missing.len(),
        4,
        "expected 4 missing ports/ errors, got: {missing:#?}"
    );
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose file errors, got: {loose:#?}"
    );
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn all_three_violations() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/utils"))).expect("mkdir");
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        12,
        "expected 12 errors (4 missing + 4 unexpected + 4 loose), got {}: {errors:#?}",
        errors.len()
    );
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
    assert_eq!(
        missing.len(),
        4,
        "expected 4 missing domain/ errors, got: {missing:#?}"
    );
    assert_eq!(
        unexpected.len(),
        4,
        "expected 4 unexpected utils/ errors, got: {unexpected:#?}"
    );
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose file errors, got: {loose:#?}"
    );
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn different_breakage_per_app() {
    let tmp = copy_fixture();
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
    let errors = arch_errors(&results);
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

    assert_file_field_crates(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// Group E: Edge cases
// ============================================================================

#[test]
fn required_dir_replaced_with_file() {
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        // Create a FILE named "domain" (not a directory)
        write_file(tmp.path(), &format!("{dir}/domain"), "not a directory");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // list_dir_names skips files, so "domain" (file) is invisible.
    // check_02: missing domain/ = 4 errors. check_loose_files: "domain" file = 4 errors.
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
    let r2: Vec<_> = rule2_errors(&errors);
    assert_eq!(
        r2.len(),
        8,
        "expected exactly 8 Rule 2 errors (4 missing + 4 loose), got {}: {r2:#?}",
        r2.len()
    );
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
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn required_dir_replaced_with_symlink() {
    let tmp = copy_fixture();
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
    let errors = arch_errors(&results);
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
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn empty_required_dir_still_present() {
    let tmp = copy_fixture();
    // Empty out domain/ contents in all 4 (but dir still exists)
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/domain"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // Rule 2 only checks dir exists in list_dir_names — dir is present, so no Rule 2 error.
    let rule2_errs: Vec<_> = errors
        .iter()
        .filter(|e| {
            e.title.contains("missing")
                && e.title.contains("crates/")
                && (e.title.contains("/domain/")
                    || e.title.contains("/app/")
                    || e.title.contains("/ports/")
                    || e.title.contains("/adapters/"))
                || e.title.contains("unexpected directory")
                || e.title.contains("loose files in crates/")
        })
        .collect();
    assert_eq!(
        rule2_errs.len(),
        0,
        "expected 0 Rule 2 errors (dir exists even if empty), got: {rule2_errs:#?}"
    );
    assert_no_packages(&errors);
}

#[test]
fn ts_app_with_cargo_toml_gets_checked() {
    let tmp = copy_fixture();
    // Add Cargo.toml to admin (TS app). Now it becomes a "Rust app" for the check.
    write_file(
        tmp.path(),
        "apps/admin/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"",
    );
    // admin has src/ (triggers check_12 src/ ban) and no crates/ (triggers Rule 1)
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let admin_errs: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("admin"))
        .collect();
    assert_eq!(
        admin_errs.len(),
        2,
        "expected 2 admin errors (missing crates/ + src/ ban), got {}: {admin_errs:#?}",
        admin_errs.len()
    );
    assert!(
        admin_errs
            .iter()
            .any(|e| e.title.contains("missing crates/")),
        "admin should have 'missing crates/' error, got: {admin_errs:#?}"
    );
    assert!(
        admin_errs.iter().any(|e| e.title.contains("src/")),
        "admin should have 'src/' error (check_12), got: {admin_errs:#?}"
    );
    assert_no_packages(&errors);
}

#[test]
fn app_without_cargo_toml_skipped() {
    let tmp = copy_fixture();
    // Remove Cargo.toml from devctl. Break its crates/ structure.
    remove_file(tmp.path(), "apps/devctl/Cargo.toml");
    // Add an unexpected dir to make sure no errors appear if we DID check
    std::fs::create_dir_all(tmp.path().join("apps/devctl/crates/utils")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl")),
        "devctl without Cargo.toml should not be checked, got devctl errors in: {errors:#?}"
    );
    assert_no_packages(&errors);
}

// ============================================================================
// Group F: Inner hex isolation
// ============================================================================

#[test]
fn loose_file_inner_hex_only() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{INNER_HEX}/mod.rs"), "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
        errors[0].title.contains("mcp/crates")
            || errors[0].title.contains("adapters/inbound/mcp/crates"),
        "expected inner hex title to contain full nested path 'mcp/crates', got: '{}'",
        errors[0].title
    );
    assert!(
        errors[0]
            .file
            .as_deref()
            .unwrap_or("")
            .contains("mcp/crates"),
        "expected file field referencing mcp/crates, got: '{}'",
        errors[0].file.as_deref().unwrap_or("")
    );
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl")),
        "devctl should have no errors, got: {errors:#?}"
    );
    assert!(
        !errors.iter().any(|e| e.title.contains("worker")),
        "worker should have no errors, got: {errors:#?}"
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn unexpected_dir_inner_hex_only() {
    let tmp = copy_fixture();
    std::fs::create_dir_all(tmp.path().join(format!("{INNER_HEX}/utils"))).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
        errors[0].title.contains("mcp/crates")
            || errors[0].title.contains("adapters/inbound/mcp/crates"),
        "expected inner hex title to contain full nested path 'mcp/crates', got: '{}'",
        errors[0].title
    );
    assert!(
        errors[0]
            .file
            .as_deref()
            .unwrap_or("")
            .contains("mcp/crates"),
        "expected file field referencing mcp/crates, got: '{}'",
        errors[0].file.as_deref().unwrap_or("")
    );
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl")),
        "devctl should have no errors, got: {errors:#?}"
    );
    assert!(
        !errors.iter().any(|e| e.title.contains("worker")),
        "worker should have no errors, got: {errors:#?}"
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn near_miss_dir_names() {
    let tmp = copy_fixture();
    let near_misses = ["domains", "adapter", "port", "application"];
    for dir in ALL_CRATES_DIRS {
        for name in &near_misses {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/{name}"))).expect("mkdir");
        }
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
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
    for err in &unexpected {
        assert!(
            err.file.as_deref().unwrap_or("").contains("crates"),
            "expected file field containing 'crates', got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// Group G: New scenarios
// ============================================================================

#[test]
fn file_coexists_with_same_named_dir() {
    let tmp = copy_fixture();
    // On most filesystems, a file and dir cannot share the same name.
    // Instead, create files whose names echo the required dir names with a suffix,
    // alongside files with completely unrelated names. The key behavior under test:
    // loose files are flagged while the real dirs remain recognized as present.
    for dir in ALL_CRATES_DIRS {
        write_file(tmp.path(), &format!("{dir}/adapters.bak"), "not a dir");
        write_file(tmp.path(), &format!("{dir}/app.old"), "not a dir");
        write_file(tmp.path(), &format!("{dir}/domain.txt"), "not a dir");
        write_file(tmp.path(), &format!("{dir}/ports.rs"), "not a dir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);

    // Should have 4 loose-file errors (1 per location, each listing all 4 files)
    let loose: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("loose files"))
        .collect();
    assert_eq!(
        loose.len(),
        4,
        "expected 4 loose file errors (1 per location), got {}: {loose:#?}",
        loose.len()
    );
    // Each message should mention the 4 file names
    for err in &loose {
        for name in &["adapters.bak", "app.old", "domain.txt", "ports.rs"] {
            assert!(
                err.message.contains(name),
                "expected message to list '{name}', got: '{}'",
                err.message
            );
        }
    }
    // No missing-dir errors: the dirs still exist
    let missing: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("/ directory"))
        .collect();
    assert_eq!(
        missing.len(),
        0,
        "expected 0 missing-dir errors (dirs still exist), got: {missing:#?}"
    );
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn unicode_lookalike_dir_name() {
    let tmp = copy_fixture();
    // Create dir named "d\u{200B}omain" (zero-width space) in all 4 locations.
    // This looks like "domain" but is a different string. The real domain/ still exists,
    // so only 4 unexpected-dir errors should fire (no missing-dir errors).
    let lookalike = "d\u{200B}omain";
    for dir in ALL_CRATES_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/{lookalike}"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let unexpected: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("unexpected directory"))
        .collect();
    assert_eq!(
        unexpected.len(),
        4,
        "expected 4 unexpected-dir errors for unicode lookalike, got {}: {unexpected:#?}",
        unexpected.len()
    );
    // No missing-dir errors (real domain/ still exists)
    let missing: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("domain"))
        .collect();
    assert_eq!(
        missing.len(),
        0,
        "expected 0 missing domain/ errors (real dir still exists), got: {missing:#?}"
    );
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn gitkeep_as_directory() {
    let tmp = copy_fixture();
    // Create .gitkeep as a DIRECTORY (mkdir .gitkeep) in all 4 locations.
    // The file .gitkeep exemption only applies to files, not dirs.
    // list_dir_names will include ".gitkeep" as a dir name, which is unexpected.
    for dir in ALL_CRATES_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/.gitkeep"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let unexpected: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("unexpected directory") && e.title.contains(".gitkeep"))
        .collect();
    assert_eq!(
        unexpected.len(),
        4,
        "expected 4 unexpected-dir errors (.gitkeep dir is not in expected set), got {}: {unexpected:#?}",
        unexpected.len()
    );
    assert_file_field_crates(&errors);
    assert_per_app(&errors);
    assert_inner_hex(&errors);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn packages_not_checked() {
    let tmp = copy_fixture();
    // Add broken structure to packages/shared-types: create crates/ with unexpected dirs.
    // packages/ is not under apps/, so check_hex_arch_structure should ignore it entirely.
    std::fs::create_dir_all(tmp.path().join("packages/shared-types/crates/utils")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("packages/shared-types/crates/garbage"))
        .expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("packages/ui-kit/crates/nonsense")).expect("mkdir");

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // Assert 0 R-ARCH-01 errors from packages
    assert!(
        !errors.iter().any(|e| {
            let t = &e.title;
            let f = e.file.as_deref().unwrap_or("");
            t.contains("shared-types") || t.contains("ui-kit") || f.contains("packages/")
        }),
        "packages should not produce any R-ARCH-01 errors, got: {errors:#?}"
    );
    // Golden fixture is clean, so total errors should be 0
    assert_eq!(
        errors.len(),
        0,
        "expected 0 errors (golden is clean + packages not checked), got {}: {errors:#?}",
        errors.len()
    );
}

#[test]
fn new_app_gets_checked() {
    let tmp = copy_fixture();
    // Create apps/scheduler/ with Cargo.toml and a broken crates/ structure
    write_file(
        tmp.path(),
        "apps/scheduler/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"",
    );
    // Create crates/ with only domain/ (missing adapters/, app/, ports/)
    std::fs::create_dir_all(tmp.path().join("apps/scheduler/crates/domain")).expect("mkdir");
    write_file(tmp.path(), "apps/scheduler/crates/domain/.gitkeep", "");

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);

    // scheduler should have errors for missing adapters/, app/, ports/
    let sched_errs: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("scheduler"))
        .collect();
    assert_eq!(
        sched_errs.len(),
        3,
        "expected 3 scheduler errors (missing adapters/, app/, ports/), got {}: {sched_errs:#?}",
        sched_errs.len()
    );
    for exp_dir in &["adapters", "app", "ports"] {
        assert!(
            sched_errs
                .iter()
                .any(|e| e.title.contains("missing") && e.title.contains(exp_dir)),
            "expected scheduler error for missing '{exp_dir}/', got: {sched_errs:#?}"
        );
    }
    // Existing apps should still be clean (no errors for devctl, backend, worker)
    let existing_app_errs: Vec<_> = errors
        .iter()
        .filter(|e| {
            e.title.contains("devctl") || e.title.contains("backend") || e.title.contains("worker")
        })
        .collect();
    assert_eq!(
        existing_app_errs.len(),
        0,
        "existing apps should still be clean, got: {existing_app_errs:#?}"
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn permission_denied_crates() {
    let tmp = copy_fixture();
    let crates_path = tmp.path().join("apps/devctl/crates");

    // chmod 000 on devctl's crates/ — list_dir returns empty.
    // check_01 sees empty list_dir result => "missing crates/" (1 error, early return).
    // check_02 never runs because check_01 returns false.
    use std::os::unix::fs::PermissionsExt;
    let perms_none = std::fs::Permissions::from_mode(0o000);
    std::fs::set_permissions(&crates_path, perms_none).expect("chmod 000");

    let results = run_check(tmp.path());

    // Restore permissions before any assertions (so tempdir cleanup works)
    let perms_restore = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(&crates_path, perms_restore).expect("chmod 755");

    let errors = arch_errors(&results);

    // devctl crates/ is unreadable: list_dir on crates/ returns empty,
    // so check_01 fires "missing crates/ directory" (1 error) and returns early.
    let devctl_errs: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("devctl"))
        .collect();
    assert_eq!(
        devctl_errs.len(),
        1,
        "expected 1 devctl error ('missing crates/' when unreadable), got {}: {devctl_errs:#?}",
        devctl_errs.len()
    );
    assert!(
        devctl_errs[0].title.contains("missing") && devctl_errs[0].title.contains("crates/"),
        "expected 'missing crates/' in devctl error title, got: '{}'",
        devctl_errs[0].title
    );
    // Other apps should be clean
    let non_devctl_errs: Vec<_> = errors
        .iter()
        .filter(|e| !e.title.contains("devctl"))
        .collect();
    assert_eq!(
        non_devctl_errs.len(),
        0,
        "expected 0 errors for non-devctl apps, got: {non_devctl_errs:#?}"
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn maximally_complex_single_location() {
    let tmp = copy_fixture();
    // In devctl/crates/:
    // - adapters/ real (keep)
    // - app/ removed, replaced with symlink to /dev/null (dangling/non-dir target)
    // - domain/ missing entirely (remove)
    // - ports/ real (keep)
    // - unexpected utils/ dir
    // - loose mod.rs file
    // - .gitkeep file (allowed, should not be flagged)

    let devctl_crates = "apps/devctl/crates";

    // Remove app/ dir and replace with symlink to /dev/null (a non-directory)
    remove_dir(tmp.path(), &format!("{devctl_crates}/app"));
    std::os::unix::fs::symlink("/dev/null", tmp.path().join(format!("{devctl_crates}/app")))
        .expect("symlink");

    // Remove domain/ entirely
    remove_dir(tmp.path(), &format!("{devctl_crates}/domain"));

    // Add unexpected utils/ dir
    std::fs::create_dir_all(tmp.path().join(format!("{devctl_crates}/utils"))).expect("mkdir");

    // Add loose mod.rs
    write_file(tmp.path(), &format!("{devctl_crates}/mod.rs"), "// stray");

    // Add .gitkeep (allowed)
    write_file(tmp.path(), &format!("{devctl_crates}/.gitkeep"), "");

    let results = run_check(tmp.path());
    let errors = arch_errors(&results);

    // Only devctl should have errors
    let devctl_errs: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("devctl"))
        .collect();

    // Expected errors for devctl from check_02 (Rule 2):
    // 1. missing app/ directory (symlink is not a dir per file_type())
    // 2. missing domain/ directory (removed entirely)
    // 3. unexpected directory utils/
    // 4. loose files: "app" (symlink) + "mod.rs" = 1 loose files error (lists both)
    //
    // check_05/check_06 also run on crates/app/ and crates/domain/ but both are
    // absent/non-dir, so list_dir returns empty and no additional errors fire.
    let missing_app: Vec<_> = devctl_errs
        .iter()
        .filter(|e| {
            e.title.contains("missing")
                && e.title.contains("app/")
                && e.title.contains("crates/app/")
        })
        .collect();
    let missing_domain: Vec<_> = devctl_errs
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("domain"))
        .collect();
    let unexpected_utils: Vec<_> = devctl_errs
        .iter()
        .filter(|e| e.title.contains("unexpected") && e.title.contains("utils"))
        .collect();
    let loose: Vec<_> = devctl_errs
        .iter()
        .filter(|e| e.title.contains("loose files"))
        .collect();

    assert!(
        !missing_app.is_empty(),
        "expected at least 1 missing app/ error, got: {devctl_errs:#?}"
    );
    assert_eq!(
        missing_domain.len(),
        1,
        "expected 1 missing domain/ error, got: {missing_domain:#?}"
    );
    assert_eq!(
        unexpected_utils.len(),
        1,
        "expected 1 unexpected utils/ error, got: {unexpected_utils:#?}"
    );
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose files error, got: {loose:#?}"
    );

    // Loose files message should mention "app" (symlink) and "mod.rs" but NOT ".gitkeep"
    let loose_msg = &loose[0].message;
    assert!(
        loose_msg.contains("app"),
        "expected loose files message to list 'app' (symlink), got: '{loose_msg}'"
    );
    assert!(
        loose_msg.contains("mod.rs"),
        "expected loose files message to list 'mod.rs', got: '{loose_msg}'"
    );
    let file_list = loose_msg.split("Only").next().unwrap_or(loose_msg);
    assert!(
        !file_list.contains(".gitkeep"),
        "expected .gitkeep NOT in loose file list, got: '{file_list}'"
    );

    // Verify Rule 2 errors present: at least 2 missing + 1 unexpected + 1 loose = 4
    let devctl_rule2: Vec<_> = devctl_errs
        .iter()
        .filter(|e| {
            (e.title.contains("missing") && e.title.contains("/ directory"))
                || e.title.contains("unexpected directory")
                || e.title.contains("loose files in")
        })
        .collect();
    assert!(
        devctl_rule2.len() == 4,
        "expected exactly 4 Rule 2 errors for devctl (2 missing + 1 unexpected + 1 loose), got {}: {devctl_rule2:#?}",
        devctl_rule2.len()
    );

    // Other apps should be clean
    let other_errs: Vec<_> = errors
        .iter()
        .filter(|e| !e.title.contains("devctl"))
        .collect();
    assert_eq!(
        other_errs.len(),
        0,
        "expected 0 errors for non-devctl apps, got: {other_errs:#?}"
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// -----------------------------------------------------------------------
// Round 2 adversarial scenarios
// -----------------------------------------------------------------------

#[test]
fn renamed_required_dir_to_near_miss() {
    // Agent renames domain/ to domains/ — missing + unexpected with related names
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/domains"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    // Each location: missing domain/ + unexpected domains/ = 2. 4 locations = 8
    assert_eq!(r2.len(), 8, "expected 8 rule2 errors, got: {r2:#?}");
    let missing: Vec<_> = r2
        .iter()
        .filter(|e| e.title.contains("missing") && e.title.contains("domain"))
        .collect();
    let unexpected: Vec<_> = r2
        .iter()
        .filter(|e| e.title.contains("unexpected") && e.title.contains("domains"))
        .collect();
    assert_eq!(missing.len(), 4, "expected 4 missing domain errors");
    assert_eq!(unexpected.len(), 4, "expected 4 unexpected domains errors");
    let r2_flat: Vec<&CheckResult> = r2.iter().map(|e| **e).collect();
    assert_per_app(&r2_flat);
    assert_no_packages(&errors);
    assert_no_ts_apps(&errors);
    assert_file_field_crates(&r2_flat);
}

#[test]
fn inner_hex_all_three_violations_outer_clean() {
    // Only inner hex is broken (all 3 violation types), outer apps clean
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{INNER_HEX}/domain"));
    std::fs::create_dir_all(tmp.path().join(format!("{INNER_HEX}/utils"))).expect("mkdir");
    write_file(tmp.path(), &format!("{INNER_HEX}/mod.rs"), "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 3, "expected 3 inner hex errors, got: {r2:#?}");
    // All from inner hex
    for err in &r2 {
        assert!(
            err.file.as_deref().unwrap_or("").contains("mcp/crates"),
            "error should be from inner hex, got file: {:?}",
            err.file
        );
    }
    // All three categories
    assert!(
        r2.iter().any(|e| e.title.contains("missing")),
        "expected missing error"
    );
    assert!(
        r2.iter().any(|e| e.title.contains("unexpected")),
        "expected unexpected error"
    );
    assert!(
        r2.iter().any(|e| e.title.contains("loose")),
        "expected loose files error"
    );
    // Outer apps clean
    assert!(
        !r2.iter().any(|e| e.title.contains("devctl")),
        "devctl should be clean"
    );
    assert!(
        !r2.iter().any(|e| e.title.contains("worker")),
        "worker should be clean"
    );
    assert_no_packages(&errors);
    assert_no_ts_apps(&errors);
}

#[test]
fn nested_unexpected_dir_tree() {
    // Deep tree inside unexpected dir — only top-level flagged, no recursion into garbage
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        let deep = format!("{dir}/utils/helpers/deep");
        std::fs::create_dir_all(tmp.path().join(&deep)).expect("mkdir");
        write_file(tmp.path(), &format!("{deep}/lib.rs"), "// buried");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(
        r2.len(),
        4,
        "expected 4 unexpected dir errors, got: {r2:#?}"
    );
    for err in &r2 {
        assert!(
            err.title.contains("unexpected") && err.title.contains("utils"),
            "expected unexpected utils error, got: {}",
            err.title
        );
        // Should NOT mention deeper levels
        assert!(
            !err.title.contains("helpers"),
            "should not recurse into unexpected dir"
        );
        assert!(
            !err.title.contains("deep"),
            "should not recurse into unexpected dir"
        );
    }
    let r2_flat: Vec<&CheckResult> = r2.iter().map(|e| **e).collect();
    assert_per_app(&r2_flat);
    assert_no_packages(&errors);
    assert_no_ts_apps(&errors);
    assert_file_field_crates(&r2_flat);
}

#[test]
fn inner_hex_error_label_prefix_is_nested() {
    // Verify inner hex error title contains the FULL nested path, not truncated
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{INNER_HEX}/domain"));
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let r2 = rule2_errors(&errors);
    assert_eq!(r2.len(), 1, "expected 1 error, got: {r2:#?}");
    let title = &r2[0].title;
    // Title must contain the full nested label prefix, not just "crates/domain/"
    assert!(
        title.contains("mcp/crates/domain"),
        "inner hex title must contain full nested path 'mcp/crates/domain', got: '{title}'"
    );
    // Must NOT be just "crates/domain/" at the outer level
    assert!(
        title.contains("adapters/inbound/mcp/crates"),
        "inner hex title must show full recursion path, got: '{title}'"
    );
    assert_no_packages(&errors);
    assert_no_ts_apps(&errors);
}

#[test]
fn idempotent_results() {
    // Running the check twice on the same broken fixture produces identical results
    let tmp = copy_fixture();
    for dir in ALL_CRATES_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
    }
    let results1 = run_check(tmp.path());
    let results2 = run_check(tmp.path());
    let errors1 = arch_errors(&results1);
    let errors2 = arch_errors(&results2);
    assert_eq!(
        errors1.len(),
        errors2.len(),
        "two runs should produce identical error counts: {} vs {}",
        errors1.len(),
        errors2.len()
    );
    // Verify same titles (order may differ, so sort)
    let mut titles1: Vec<_> = errors1.iter().map(|e| &e.title).collect();
    let mut titles2: Vec<_> = errors2.iter().map(|e| &e.title).collect();
    titles1.sort();
    titles2.sort();
    assert_eq!(
        titles1, titles2,
        "two runs should produce identical error titles"
    );
}
