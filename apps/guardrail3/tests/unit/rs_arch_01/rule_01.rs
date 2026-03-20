use super::helpers::{
    arch_errors, assert_single_error, copy_fixture, remove_dir, run_check, write_file,
    RUST_APPS,
};

// -----------------------------------------------------------------------
// Failure mode: crates/ directory missing entirely
// -----------------------------------------------------------------------

#[test]
fn missing_crates_dir() {
    let tmp = copy_fixture();
    // Break EVERY outer crates/ — inner hex unreachable because outer is gone
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 3, "expected 3 errors (one per Rust app), got: {errors:#?}");
    for app in RUST_APPS {
        let app_err = errors
            .iter()
            .find(|e| e.title.contains(app))
            .unwrap_or_else(|| panic!("missing error for {app}, got: {errors:#?}"));
        // Title must mention both the app name and "missing crates/"
        assert!(
            app_err.title.contains("missing crates/"),
            "error for {app} should mention 'missing crates/' in title, got: '{}'",
            app_err.title
        );
        // File field must point to the app directory
        let file = app_err.file.as_deref().unwrap_or("");
        assert!(
            file.contains(&format!("apps/{app}")),
            "error file field should reference apps/{app}, got: '{file}'"
        );
    }
}

// -----------------------------------------------------------------------
// Failure mode: crates/ exists but is empty (no sub-crates)
// -----------------------------------------------------------------------

#[test]
fn crates_dir_empty() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        std::fs::create_dir_all(tmp.path().join(format!("apps/{app}/crates"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        3,
        "expected exactly 3 errors (one per app with empty crates/), got {}: {errors:#?}",
        errors.len()
    );
    for app in RUST_APPS {
        let app_err = errors
            .iter()
            .find(|e| e.title.contains(app))
            .unwrap_or_else(|| panic!("missing error for {app}, got: {errors:#?}"));
        // Title must mention "missing crates/" — list_dir returns empty for an empty dir,
        // same as for missing/file cases
        assert!(
            app_err.title.contains("missing crates/"),
            "error for {app} should mention 'missing crates/' in title, got: '{}'",
            app_err.title
        );
        let file = app_err.file.as_deref().unwrap_or("");
        assert!(
            file.contains(&format!("apps/{app}")),
            "error file field should reference apps/{app}, got: '{file}'"
        );
    }
}

// -----------------------------------------------------------------------
// Failure mode: crates/ is a file, not a directory
// -----------------------------------------------------------------------

#[test]
fn crates_is_file_not_dir() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        write_file(tmp.path(), &format!("apps/{app}/crates"), "not a directory");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        3,
        "expected exactly 3 errors (one per app with crates-as-file), got {}: {errors:#?}",
        errors.len()
    );
    for app in RUST_APPS {
        let app_err = errors
            .iter()
            .find(|e| e.title.contains(app))
            .unwrap_or_else(|| panic!("missing error for {app}, got: {errors:#?}"));
        // Title must mention "missing crates/" — list_dir returns empty for a file,
        // same as for missing/empty cases
        assert!(
            app_err.title.contains("missing crates/"),
            "error for {app} should mention 'missing crates/' in title, got: '{}'",
            app_err.title
        );
        let file = app_err.file.as_deref().unwrap_or("");
        assert!(
            file.contains(&format!("apps/{app}")),
            "error file field should reference apps/{app}, got: '{file}'"
        );
    }
}

// -----------------------------------------------------------------------
// Failure mode: crates/ contains only .gitkeep (effectively empty)
// -----------------------------------------------------------------------

#[test]
fn crates_with_only_gitkeep() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        std::fs::create_dir_all(tmp.path().join(format!("apps/{app}/crates"))).expect("mkdir");
        write_file(tmp.path(), &format!("apps/{app}/crates/.gitkeep"), "");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // check_01 passes (crates/ has .gitkeep entry, not empty).
    // check_02 fires: missing required dirs (adapters, app, domain, ports) = 4 errors per app.
    // check_03..06 short-circuit because those dirs don't exist.
    // Total: 4 missing dirs * 3 apps = 12 errors.
    assert_eq!(
        errors.len(),
        12,
        "expected 12 errors (4 missing hex dirs * 3 apps), got {}: {errors:#?}",
        errors.len()
    );
    for app in RUST_APPS {
        let app_errors: Vec<_> = errors.iter().filter(|e| {
            e.file.as_deref().unwrap_or("").contains(&format!("apps/{app}"))
        }).collect();
        assert_eq!(
            app_errors.len(),
            4,
            "expected 4 errors for {app} (missing adapters, app, domain, ports), got {}: {app_errors:#?}",
            app_errors.len()
        );
    }
}

// -----------------------------------------------------------------------
// Failure mode: inner hex-in-hex crates/ missing (outer valid)
// -----------------------------------------------------------------------

#[test]
fn inner_hex_crates_missing() {
    let tmp = copy_fixture();
    // Keep outer crates/ valid — break ONLY the inner hex-in-hex crates/
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    // mcp/ dir still exists but has no crates/ inside
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // Only the inner hex is broken — outer structure for all apps is intact
    assert_eq!(errors.len(), 1, "expected exactly 1 error (inner hex only), got: {errors:#?}");
    assert!(
        errors[0].title.contains("missing") && errors[0].title.contains("crates"),
        "expected error about missing inner crates/, got: {errors:#?}"
    );
    // File field should reference the mcp directory
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("mcp"),
        "error file field should reference mcp dir, got: '{file}'"
    );
    // No errors for devctl or worker — their structure is intact
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl") || e.title.contains("worker")),
        "devctl/worker should be clean, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Failure mode: inner hex-in-hex crates/ empty
// -----------------------------------------------------------------------

#[test]
fn inner_hex_crates_empty() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    std::fs::create_dir_all(
        tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error for empty inner crates/, got: {errors:#?}");
    assert!(
        errors[0].title.contains("mcp") || errors[0].title.contains("adapters/inbound"),
        "expected title mentioning inner hex path, got: '{}'",
        errors[0].title
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/backend/crates/adapters/inbound/mcp"),
        "expected file field referencing inner hex path, got: '{file}'"
    );
}

// -----------------------------------------------------------------------
// Failure mode: inner hex-in-hex crates/ is a file
// -----------------------------------------------------------------------

#[test]
fn inner_hex_crates_is_file() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
        "not a directory",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error when inner crates is a file, got: {errors:#?}");
    // Title must verify "missing" keyword — list_dir on a file returns empty
    assert!(
        errors[0].title.contains("missing"),
        "expected title containing 'missing' for inner crates-as-file, got: '{}'",
        errors[0].title
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/backend/crates/adapters/inbound/mcp"),
        "expected file field referencing inner hex path, got: '{file}'"
    );
}

// -----------------------------------------------------------------------
// Failure mode: outer missing makes inner unreachable (no cascade)
// -----------------------------------------------------------------------

#[test]
fn outer_missing_inner_never_checked() {
    let tmp = copy_fixture();
    // Remove outer crates/ for ALL apps — inner hex-in-hex is unreachable
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // Should get exactly 3 errors (one per app, outer only), not cascade into inner
    assert_eq!(errors.len(), 3, "expected 3 errors (outer only, no cascade), got: {errors:#?}");
    // backend specifically: 1 error for outer, NOT 2 for outer+inner
    assert_eq!(
        errors.iter().filter(|e| e.title.contains("backend")).count(),
        1,
        "backend should have exactly 1 error (outer only), got: {errors:#?}"
    );
    // Each error's file field should reference its app directory
    for app in RUST_APPS {
        let app_err = errors
            .iter()
            .find(|e| e.title.contains(app))
            .unwrap_or_else(|| panic!("missing error for {app}, got: {errors:#?}"));
        let file = app_err.file.as_deref().unwrap_or("");
        assert!(
            file.contains(&format!("apps/{app}")),
            "error file field should reference apps/{app}, got: '{file}'"
        );
    }
}

// -----------------------------------------------------------------------
// Failure mode: outer crates/ is a file (inner unreachable)
// -----------------------------------------------------------------------

#[test]
fn outer_crates_file_inner_unreachable() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates");
    write_file(tmp.path(), "apps/backend/crates", "not a directory");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error (only backend broken), got: {errors:#?}");
    assert!(
        errors[0].title.contains("backend"),
        "expected error for backend, got: '{}'",
        errors[0].title
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/backend"),
        "expected file field referencing apps/backend, got: '{file}'"
    );
    // devctl and worker should not appear in errors
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl")),
        "devctl should not appear in errors, got: {errors:#?}"
    );
    assert!(
        !errors.iter().any(|e| e.title.contains("worker")),
        "worker should not appear in errors, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Failure mode: different breakage across ALL apps simultaneously
// -----------------------------------------------------------------------

#[test]
fn three_apps_three_different_failures() {
    let tmp = copy_fixture();
    // devctl: missing crates/ entirely
    remove_dir(tmp.path(), "apps/devctl/crates");
    // backend: crates/ is a file
    remove_dir(tmp.path(), "apps/backend/crates");
    write_file(tmp.path(), "apps/backend/crates", "not a dir");
    // worker: crates/ exists but empty
    remove_dir(tmp.path(), "apps/worker/crates");
    std::fs::create_dir_all(tmp.path().join("apps/worker/crates")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 3, "expected 3 errors (one per app), got: {errors:#?}");
    // All 3 failure modes (missing, file, empty) produce the same "missing crates/" title
    // because list_dir returns empty for all three. The test can't distinguish them by title —
    // that's a known limitation of the check code.
    for app in RUST_APPS {
        let app_err = errors
            .iter()
            .find(|e| e.title.contains(app))
            .unwrap_or_else(|| panic!("missing error for {app}, got: {errors:#?}"));
        // Each app should have exactly 1 error
        assert_eq!(
            errors.iter().filter(|e| e.title.contains(app)).count(),
            1,
            "expected exactly 1 error for {app}, got: {errors:#?}"
        );
        // Title must mention "missing crates/" for all failure modes
        assert!(
            app_err.title.contains("missing crates/"),
            "error for {app} should mention 'missing crates/' in title, got: '{}'",
            app_err.title
        );
        let file = app_err.file.as_deref().unwrap_or("");
        assert!(
            file.contains(&format!("apps/{app}")),
            "error file field should reference apps/{app}, got: '{file}'"
        );
    }
}

// -----------------------------------------------------------------------
// Failure mode: outer fine + inner broken, PLUS other apps broken
// -----------------------------------------------------------------------

#[test]
fn inner_hex_broken_plus_other_apps_missing() {
    let tmp = copy_fixture();
    // devctl: missing crates/ entirely
    remove_dir(tmp.path(), "apps/devctl/crates");
    // worker: missing crates/ entirely
    remove_dir(tmp.path(), "apps/worker/crates");
    // backend outer: VALID — but inner hex-in-hex crates/ missing
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 3, "expected exactly 3 errors (devctl + worker outer + backend inner), got: {errors:#?}");
    // devctl error
    let devctl_err = errors.iter().find(|e| e.title.contains("devctl"))
        .unwrap_or_else(|| panic!("missing devctl error, got: {errors:#?}"));
    let devctl_file = devctl_err.file.as_deref().unwrap_or("");
    assert!(
        devctl_file.contains("apps/devctl"),
        "devctl error file field should reference apps/devctl, got: '{devctl_file}'"
    );
    // worker error
    let worker_err = errors.iter().find(|e| e.title.contains("worker"))
        .unwrap_or_else(|| panic!("missing worker error, got: {errors:#?}"));
    let worker_file = worker_err.file.as_deref().unwrap_or("");
    assert!(
        worker_file.contains("apps/worker"),
        "worker error file field should reference apps/worker, got: '{worker_file}'"
    );
    // backend inner hex error — require "mcp" in title (not fallback to "backend")
    let backend_inner_err = errors.iter().find(|e| e.title.contains("mcp"))
        .unwrap_or_else(|| panic!("missing backend inner hex error with 'mcp' in title, got: {errors:#?}"));
    let backend_file = backend_inner_err.file.as_deref().unwrap_or("");
    assert!(
        backend_file.contains("apps/backend/crates/adapters/inbound/mcp"),
        "backend inner hex error file field should reference inner hex path, got: '{backend_file}'"
    );
}

// -----------------------------------------------------------------------
// Failure mode: src/ exists AND crates/ missing (two rules fire)
// -----------------------------------------------------------------------

#[test]
fn src_exists_and_crates_missing() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        write_file(tmp.path(), &format!("apps/{app}/src/main.rs"), "fn main() {}");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // Each app should have 2 errors: src/ ban + missing crates/
    assert_eq!(
        errors.len(),
        6,
        "expected exactly 6 errors (2 per app), got {}: {errors:#?}",
        errors.len()
    );
    for app in RUST_APPS {
        let src_err = errors.iter().find(|e| e.title.contains(app) && e.title.contains("src/"))
            .unwrap_or_else(|| panic!("expected src/ error for {app}, got: {errors:#?}"));
        let src_file = src_err.file.as_deref().unwrap_or("");
        assert!(
            src_file.contains(&format!("apps/{app}")),
            "src/ error file field should reference apps/{app}, got: '{src_file}'"
        );
        let crates_err = errors.iter().find(|e| e.title.contains(app) && e.title.contains("missing crates/"))
            .unwrap_or_else(|| panic!("expected missing crates/ error for {app}, got: {errors:#?}"));
        let crates_file = crates_err.file.as_deref().unwrap_or("");
        assert!(
            crates_file.contains(&format!("apps/{app}")),
            "missing crates/ error file field should reference apps/{app}, got: '{crates_file}'"
        );
    }
}

// -----------------------------------------------------------------------
// Failure mode: src/ exists but crates/ valid (only src/ ban fires)
// -----------------------------------------------------------------------

#[test]
fn src_and_crates_both_exist() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        write_file(tmp.path(), &format!("apps/{app}/src/main.rs"), "fn main() {}");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        3,
        "expected exactly 3 src/ errors (one per app), got {}: {errors:#?}",
        errors.len()
    );
    for app in RUST_APPS {
        let app_err = errors
            .iter()
            .find(|e| e.title.contains(app) && e.title.contains("src/"))
            .unwrap_or_else(|| panic!("expected src/ error for {app}, got: {errors:#?}"));
        let file = app_err.file.as_deref().unwrap_or("");
        assert!(
            file.contains(&format!("apps/{app}")),
            "error file field should reference apps/{app}, got: '{file}'"
        );
    }
    // Negative assertion: no "missing crates/" errors since crates/ is valid
    assert!(
        !errors.iter().any(|e| e.title.contains("missing crates/")),
        "should not have 'missing crates/' errors when crates/ exists, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Edge case: src/ inside hex-in-hex (not at app level — no src/ ban)
// -----------------------------------------------------------------------

#[test]
fn inner_hex_has_src() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/src/main.rs",
        "fn main() {}",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // src/ ban only fires at app level, not inside inner hex
    assert_eq!(errors.len(), 0, "src/ ban should not fire at inner hex level, got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Edge case: TS apps not checked (no false positives)
// -----------------------------------------------------------------------

#[test]
fn ts_apps_not_checked() {
    let tmp = copy_fixture();
    // admin and landing have no Cargo.toml — should produce 0 R-ARCH-01 errors
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(errors.is_empty(), "golden fixture should produce zero R-ARCH-01 errors, got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Edge case: broken symlink where crates/ should be
// -----------------------------------------------------------------------

#[test]
fn crates_is_broken_symlink() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        std::os::unix::fs::symlink(
            "/nonexistent/path",
            tmp.path().join(format!("apps/{app}/crates")),
        )
        .expect("symlink");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        3,
        "expected exactly 3 errors for broken symlink crates/, got {}: {errors:#?}",
        errors.len()
    );
    for app in RUST_APPS {
        let app_err = errors
            .iter()
            .find(|e| e.title.contains(app))
            .unwrap_or_else(|| panic!("missing error for {app}, got: {errors:#?}"));
        assert!(
            app_err.title.contains("missing crates/") || app_err.title.contains("crates/"),
            "error for {app} should mention crates/ in title, got: '{}'",
            app_err.title
        );
        let file = app_err.file.as_deref().unwrap_or("");
        assert!(
            file.contains(&format!("apps/{app}")),
            "error file field should reference apps/{app}, got: '{file}'"
        );
    }
}

// -----------------------------------------------------------------------
// Edge case: crates/ symlinked to another app's crates/ (resolves to valid dir)
// -----------------------------------------------------------------------

#[test]
fn crates_is_symlink_to_other_app() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/worker/crates"),
        tmp.path().join("apps/devctl/crates"),
    )
    .expect("symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // Symlink resolves to a valid crates/ directory — no error expected
    assert!(errors.is_empty(), "symlink to valid crates/ should be transparent, got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Edge case: inner hex crates/ is broken symlink
// -----------------------------------------------------------------------

#[test]
fn inner_hex_crates_is_broken_symlink() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    std::os::unix::fs::symlink(
        "/nonexistent",
        tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates"),
    )
    .expect("symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error for broken inner symlink crates/, got: {errors:#?}");
    assert!(
        errors[0].title.contains("mcp") || errors[0].title.contains("adapters/inbound"),
        "error title should mention inner hex path (mcp or adapters/inbound), got: '{}'",
        errors[0].title
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/backend/crates/adapters/inbound/mcp"),
        "error file field should reference inner hex path, got: '{file}'"
    );
}

// -----------------------------------------------------------------------
// Edge case: inner hex crates/ symlink to outer crates/ (resolves to valid dir)
// -----------------------------------------------------------------------

#[test]
fn inner_hex_crates_symlink_to_outer_crates() {
    let tmp = copy_fixture();
    let inner = tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates");
    let outer = tmp.path().join("apps/backend/crates");
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    std::os::unix::fs::symlink(&outer, &inner).expect("symlink");
    let results = run_check(tmp.path());
    // Must terminate without infinite loop.
    // The symlink from inner mcp/crates -> outer crates/ creates a recursive loop:
    // the resolved dir contains adapters/inbound/mcp which contains the symlink again.
    // The check recurses until it hits a depth/path-length limit and produces a bounded
    // number of errors. The key assertion is termination + bounded output.
    let errors = arch_errors(&results);
    assert!(
        errors.len() <= 5,
        "should terminate with bounded errors, got {}: {errors:#?}",
        errors.len()
    );
}

// -----------------------------------------------------------------------
// Edge case: crates/ symlinked to /dev/null
// -----------------------------------------------------------------------

#[test]
fn crates_symlink_to_dev_null() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        std::os::unix::fs::symlink(
            "/dev/null",
            tmp.path().join(format!("apps/{app}/crates")),
        )
        .expect("symlink");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        3,
        "expected exactly 3 errors (one per app symlinked to /dev/null), got {}: {errors:#?}",
        errors.len()
    );
    for app in RUST_APPS {
        let app_err = errors
            .iter()
            .find(|e| e.title.contains(app))
            .unwrap_or_else(|| panic!("missing error for {app}, got: {errors:#?}"));
        let file = app_err.file.as_deref().unwrap_or("");
        assert!(
            file.contains(&format!("apps/{app}")),
            "error file field should reference apps/{app}, got: '{file}'"
        );
    }
}

// -----------------------------------------------------------------------
// Edge case: app with only Cargo.toml (no structure at all)
// -----------------------------------------------------------------------

#[test]
fn app_with_only_cargo_toml() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/phantom/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error (phantom app missing crates/), got: {errors:#?}");
    assert!(
        errors[0].title.contains("phantom") && errors[0].title.contains("missing crates/"),
        "expected error about phantom app missing crates/, got: {errors:#?}"
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/phantom"),
        "error file field should reference apps/phantom, got: '{file}'"
    );
}

// -----------------------------------------------------------------------
// Edge case: Cargo.toml is empty / malformed
// -----------------------------------------------------------------------

#[test]
fn cargo_toml_is_empty() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/phantom/Cargo.toml", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error for phantom app, got: {errors:#?}");
    assert!(
        errors[0].title.contains("missing crates/") && errors[0].title.contains("phantom"),
        "error title should contain 'missing crates/' and 'phantom', got: '{}'",
        errors[0].title
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/phantom"),
        "error file field should reference apps/phantom, got: '{file}'"
    );
}

#[test]
fn cargo_toml_is_malformed() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/phantom/Cargo.toml", "this is not valid toml {{{{");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error for phantom app, got: {errors:#?}");
    assert!(
        errors[0].title.contains("missing crates/") && errors[0].title.contains("phantom"),
        "error title should contain 'missing crates/' and 'phantom', got: '{}'",
        errors[0].title
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/phantom"),
        "error file field should reference apps/phantom, got: '{file}'"
    );
}

// -----------------------------------------------------------------------
// Edge case: Cargo.toml is a directory (should be skipped)
// -----------------------------------------------------------------------

#[test]
fn cargo_toml_is_a_directory() {
    let tmp = copy_fixture();
    std::fs::create_dir_all(tmp.path().join("apps/broken/Cargo.toml")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(errors.is_empty(), "golden apps clean + broken app skipped should produce zero R-ARCH-01 errors, got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Edge case: Cargo.toml is a broken symlink (should be skipped)
// -----------------------------------------------------------------------

#[test]
fn cargo_toml_is_broken_symlink() {
    let tmp = copy_fixture();
    std::fs::create_dir_all(tmp.path().join("apps/broken")).expect("mkdir");
    std::os::unix::fs::symlink("/nonexistent", tmp.path().join("apps/broken/Cargo.toml"))
        .expect("symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(errors.is_empty(), "golden apps clean + broken symlink app skipped should produce zero R-ARCH-01 errors, got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Edge case: unicode and space in app names
// -----------------------------------------------------------------------

#[test]
fn unicode_app_name() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/\u{00fc}ber-service/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error for unicode app, got: {errors:#?}");
    assert!(
        errors[0].title.contains("\u{00fc}ber-service"),
        "expected error about unicode app name, got: {errors:#?}"
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/\u{00fc}ber-service"),
        "error file field should reference apps/\u{00fc}ber-service, got: '{file}'"
    );
}

#[test]
fn app_name_with_spaces() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/my app/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error for spaced app name, got: {errors:#?}");
    assert!(
        errors[0].title.contains("my app"),
        "expected error about spaced app name, got: {errors:#?}"
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/my app"),
        "error file field should reference apps/my app, got: '{file}'"
    );
}

// -----------------------------------------------------------------------
// Edge case: wrong placement — crates/ inside domain/ or src/
// -----------------------------------------------------------------------

#[test]
fn crates_inside_domain() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/domain/crates/types/Cargo.toml",
        "[package]\nname=\"t\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(errors.is_empty(), "golden apps clean + wrong-place crates/ invisible should produce zero R-ARCH-01 errors, got: {errors:#?}");
}

#[test]
fn crates_inside_src() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/src/crates/domain/Cargo.toml",
        "[package]\nname=\"d\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_single_error(&errors, "src/");
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/devctl"),
        "error file field should reference apps/devctl, got: '{file}'"
    );
}

// -----------------------------------------------------------------------
// Edge case: wrong casing (Crates/) and typo (crate/)
// -----------------------------------------------------------------------

#[test]
fn wrong_casing_crates() {
    let tmp = copy_fixture();
    std::fs::create_dir_all(tmp.path().join("apps/devctl/Crates/domain")).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(errors.is_empty(), "expected zero R-ARCH-01 errors, got: {errors:#?}");
}

#[test]
fn typo_crate_singular() {
    let tmp = copy_fixture();
    std::fs::create_dir_all(tmp.path().join("apps/phantom/crate/domain")).expect("mkdir");
    write_file(
        tmp.path(),
        "apps/phantom/Cargo.toml",
        "[workspace]\nmembers=[]\nresolver=\"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error for phantom with typo crate/, got: {errors:#?}");
    assert!(
        errors[0].title.contains("phantom") && errors[0].title.contains("missing crates/"),
        "expected error about phantom missing crates/, got: {errors:#?}"
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("apps/phantom"),
        "error file field should reference apps/phantom, got: '{file}'"
    );
}

// -----------------------------------------------------------------------
// Edge case: leaf crate with Cargo.toml not mistaken for hex-in-hex
// -----------------------------------------------------------------------

#[test]
fn hex_in_hex_leaf_has_cargo_toml_so_no_recursion() {
    // Scope: R-ARCH-01 only — a crate (has Cargo.toml) should not be treated as hex-in-hex container
    let tmp = copy_fixture();
    // backend/adapters/inbound/rest has Cargo.toml — it's a crate, not hex-in-hex
    // Add a random dir inside — should not be checked by hex arch rules
    std::fs::create_dir_all(
        tmp.path().join("apps/backend/crates/adapters/inbound/rest/internal/stuff"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(errors.is_empty(), "dirs inside a leaf crate should not trigger errors, got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Edge case: hex-in-hex at various containers
// -----------------------------------------------------------------------

#[test]
fn hex_in_hex_at_different_containers() {
    // Scope: R-ARCH-01 only — hex-in-hex should be valid at any container level (domain, app, etc.)
    let tmp = copy_fixture();
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
    let errors = arch_errors(&results);
    assert!(errors.is_empty(), "hex-in-hex in domain/ should be valid, got: {errors:#?}");
}

#[test]
fn multiple_hex_in_hex_in_same_container() {
    // Scope: R-ARCH-01 only — multiple hex-in-hex in same container (adapters/inbound) should all be valid
    let tmp = copy_fixture();
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
    let errors = arch_errors(&results);
    assert!(errors.is_empty(), "two hex-in-hex in same container should be valid, got: {errors:#?}");
}

#[test]
fn hex_in_hex_in_ports() {
    // Scope: R-ARCH-01 only — hex-in-hex should be valid even in ports/ container
    let tmp = copy_fixture();
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
    let errors = arch_errors(&results);
    assert!(errors.is_empty(), "hex-in-hex in ports/ should be valid, got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Edge case: triple-nested hex-in-hex
// -----------------------------------------------------------------------

#[test]
fn triple_nested_hex_in_hex() {
    // Scope: R-ARCH-01 only — three levels of hex nesting should be valid if structure is correct
    let tmp = copy_fixture();
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
    let errors = arch_errors(&results);
    assert!(errors.is_empty(), "triple-nested hex-in-hex should be valid, got: {errors:#?}");
}

#[test]
fn hex_in_hex_missing_crates_at_third_level() {
    let tmp = copy_fixture();
    let base = "apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound/transport";
    remove_dir(tmp.path(), base);
    std::fs::create_dir_all(tmp.path().join(format!("{base}/crates"))).expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error for empty third-level crates/, got: {errors:#?}");
    assert!(
        errors[0].title.contains("transport") || errors[0].title.contains("adapters/inbound"),
        "expected title mentioning third-level hex path, got: '{}'",
        errors[0].title
    );
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("transport"),
        "expected file field referencing transport dir, got: '{file}'"
    );
}

// -----------------------------------------------------------------------
// Edge case: inner hex with wrong dirs (src/ instead of hex containers)
// -----------------------------------------------------------------------

#[test]
fn hex_in_hex_inner_has_wrong_dirs() {
    let tmp = copy_fixture();
    let base = "apps/devctl/crates/app/complex/crates";
    std::fs::create_dir_all(tmp.path().join(format!("{base}/src"))).expect("mkdir");
    write_file(tmp.path(), &format!("{base}/src/lib.rs"), "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // Inner hex has crates/ with only src/ — missing 4 required dirs (domain, app, adapters, ports)
    // plus 1 unexpected entry (src/) = 5 errors
    assert_eq!(errors.len(), 5, "expected 5 errors (4 missing dirs + 1 unexpected src/), got {}: {errors:#?}", errors.len());
    // Tighten title assertions: check each error specifically
    // The 4 missing dirs should mention "domain", "app", "ports", "adapters"
    for required_dir in &["domain", "app", "ports", "adapters"] {
        assert!(
            errors.iter().any(|e| e.title.contains(required_dir)),
            "expected an error mentioning missing '{required_dir}' dir, got: {errors:#?}"
        );
    }
    // The unexpected dir error should mention "src"
    assert!(
        errors.iter().any(|e| e.title.contains("src")),
        "expected an error mentioning unexpected 'src' dir, got: {errors:#?}"
    );
    // File field should reference the inner hex path
    for err in &errors {
        let file = err.file.as_deref().unwrap_or("");
        assert!(
            file.contains("apps/devctl/crates/app/complex"),
            "error file field should reference inner hex path, got: '{file}'"
        );
    }
}

// -----------------------------------------------------------------------
// Edge case: leaf has both Cargo.toml and crates/ (conflict)
// -----------------------------------------------------------------------

#[test]
fn third_level_nesting_at_wrong_place() {
    let tmp = copy_fixture();
    std::fs::create_dir_all(
        tmp.path().join("apps/devctl/crates/domain/types/crates/domain/inner"),
    )
    .expect("mkdir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_single_error(&errors, "has both Cargo.toml and crates/");
    let file = errors[0].file.as_deref().unwrap_or("");
    assert!(file.contains("domain/types"), "file should point to types dir, got: {file}");
}

// -----------------------------------------------------------------------
// Edge case: filesystem permissions
// -----------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn crates_no_read_permission() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = copy_fixture();
    for app in RUST_APPS {
        let crates = tmp.path().join(format!("apps/{app}/crates"));
        std::fs::set_permissions(&crates, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    }
    let results = run_check(tmp.path());
    // Restore permissions so tempdir cleanup works
    for app in RUST_APPS {
        let crates = tmp.path().join(format!("apps/{app}/crates"));
        std::fs::set_permissions(&crates, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    }
    let errors = arch_errors(&results);
    assert_eq!(
        errors.len(),
        3,
        "expected exactly 3 errors (one per app with unreadable crates/), got {}: {errors:#?}",
        errors.len()
    );
    for app in RUST_APPS {
        let app_err = errors
            .iter()
            .find(|e| e.title.contains(app))
            .unwrap_or_else(|| panic!("missing error for {app}, got: {errors:#?}"));
        let file = app_err.file.as_deref().unwrap_or("");
        assert!(
            file.contains(&format!("apps/{app}")),
            "error file field should reference apps/{app}, got: '{file}'"
        );
    }
}

// -----------------------------------------------------------------------
// Edge case: inner hex crates/ with only .gitkeep
// -----------------------------------------------------------------------

#[test]
fn inner_hex_crates_with_only_gitkeep() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    std::fs::create_dir_all(tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates")).expect("mkdir");
    write_file(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    // Inner crates/ has .gitkeep — passes check_01 but check_02 fires for 4 missing required dirs
    assert_eq!(errors.len(), 4, "expected 4 missing dir errors for inner hex, got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Edge case: inner hex crates/ symlinked to /dev/null
// -----------------------------------------------------------------------

#[test]
fn inner_hex_crates_symlink_to_dev_null() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound/mcp/crates");
    std::os::unix::fs::symlink("/dev/null", tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates")).expect("symlink");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error for inner crates/ -> /dev/null, got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Edge case: inner hex crates/ with no read permission
// -----------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn inner_hex_crates_no_read_permission() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = copy_fixture();
    let inner = tmp.path().join("apps/backend/crates/adapters/inbound/mcp/crates");
    std::fs::set_permissions(&inner, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    let results = run_check(tmp.path());
    std::fs::set_permissions(&inner, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 error for unreadable inner crates/, got: {errors:#?}");
}

// -----------------------------------------------------------------------
// Single-app isolation: break ONLY devctl, assert others clean
// -----------------------------------------------------------------------

#[test]
fn missing_crates_dir_devctl_only() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error (devctl only), got: {errors:#?}");
    assert!(
        errors[0].title.contains("devctl"),
        "error should mention devctl, got: '{}'",
        errors[0].title
    );
    // backend and worker must produce 0 errors
    assert!(
        !errors.iter().any(|e| e.title.contains("backend")),
        "backend should produce 0 errors, got: {errors:#?}"
    );
    assert!(
        !errors.iter().any(|e| e.title.contains("worker")),
        "worker should produce 0 errors, got: {errors:#?}"
    );
}

// -----------------------------------------------------------------------
// Single-app isolation: break ONLY worker, assert others clean
// -----------------------------------------------------------------------

#[test]
fn missing_crates_dir_worker_only() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/worker/crates");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), 1, "expected exactly 1 error (worker only), got: {errors:#?}");
    assert!(
        errors[0].title.contains("worker"),
        "error should mention worker, got: '{}'",
        errors[0].title
    );
    // backend and devctl must produce 0 errors
    assert!(
        !errors.iter().any(|e| e.title.contains("backend")),
        "backend should produce 0 errors, got: {errors:#?}"
    );
    assert!(
        !errors.iter().any(|e| e.title.contains("devctl")),
        "devctl should produce 0 errors, got: {errors:#?}"
    );
}
