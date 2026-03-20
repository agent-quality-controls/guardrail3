use super::helpers::{
    arch_01_errors, copy_golden, remove_dir, run_check, write_file,
};
use guardrail3::domain::report::CheckResult;

const RUST_APPS: &[&str] = &["devctl", "backend", "worker"];
const INNER_HEX: &str = "apps/backend/crates/adapters/inbound/mcp/crates";

/// All adapters/ dirs that get checked (4 locations).
const ALL_ADAPTERS_DIRS: &[&str] = &[
    "apps/devctl/crates/adapters",
    "apps/backend/crates/adapters",
    "apps/worker/crates/adapters",
    "apps/backend/crates/adapters/inbound/mcp/crates/adapters",
];

/// All ports/ dirs that get checked (4 locations).
const ALL_PORTS_DIRS: &[&str] = &[
    "apps/devctl/crates/ports",
    "apps/backend/crates/ports",
    "apps/worker/crates/ports",
    "apps/backend/crates/adapters/inbound/mcp/crates/ports",
];

// ============================================================================
// Shared assertion helpers
// ============================================================================

/// Filter to only Rule 3 errors by title pattern.
/// Rule 3 errors match: "missing .../inbound|outbound/ directory",
/// "unexpected directory .../", "loose files in .../"
/// where the label contains "adapters" or "ports".
fn rule3_errors<'a>(errors: &[&'a CheckResult]) -> Vec<&'a CheckResult> {
    errors
        .iter()
        .copied()
        .filter(|e| {
            let t = &e.title;
            let is_structural = t.contains("adapters") || t.contains("ports");
            let is_missing_io =
                t.contains("missing") && (t.contains("inbound") || t.contains("outbound"));
            let is_unexpected = t.contains("unexpected directory");
            let is_loose = t.contains("loose files in");
            is_structural && (is_missing_io || is_unexpected || is_loose)
        })
        .collect()
}

/// Assert that each Rust app (devctl, backend, worker) produced at least one error.
fn assert_per_app(errors: &[&CheckResult]) {
    for app in RUST_APPS {
        assert!(
            errors.iter().any(|e| e.title.contains(app)),
            "expected error for app `{app}`, got: {errors:#?}"
        );
    }
}

/// Assert that the inner hex (mcp/crates) produced at least one error.
fn assert_inner_hex(errors: &[&CheckResult]) {
    assert!(
        errors
            .iter()
            .any(|e| e.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "expected at least one error from inner hex (mcp/crates), got: {errors:#?}"
    );
}

/// Assert no errors mention packages/ items (shared-types, ui-kit).
fn assert_no_packages(errors: &[&CheckResult]) {
    assert!(
        !errors.iter().any(|e| {
            let t = &e.title;
            t.contains("shared-types") || t.contains("ui-kit")
        }),
        "packages should not be flagged, got: {errors:#?}"
    );
}

/// Assert no errors mention TS apps (admin, landing).
fn assert_no_ts_apps(errors: &[&CheckResult]) {
    assert!(
        !errors
            .iter()
            .any(|e| e.title.contains("admin") || e.title.contains("landing")),
        "TS apps should not be flagged, got: {errors:#?}"
    );
}

// ============================================================================
// Group A: Missing required dirs
// ============================================================================

#[test]
fn missing_inbound_in_adapters_everywhere() {
    let tmp = copy_golden();
    // Removing adapters/inbound from backend outer DESTROYS the inner hex path
    // (mcp is under adapters/inbound/). So backend's inner hex becomes unreachable.
    // Result: 3 errors (one per outer app), NOT 4.
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| {
            e.title.contains("missing")
                && e.title.contains("inbound")
                && e.title.contains("adapters")
        })
        .collect();
    // 3 outer apps * 1 missing inbound = 3 errors
    // Inner hex is destroyed (unreachable) so no inner hex error
    assert_eq!(
        r3.len(),
        3,
        "expected 3 errors (outer only; inner hex destroyed), got {}: {r3:#?}",
        r3.len()
    );
    for err in &r3 {
        assert!(
            err.title.contains("missing") && err.title.contains("inbound"),
            "expected title mentioning 'missing' and 'inbound', got: '{}'",
            err.title
        );
    }
    assert_per_app(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_outbound_in_adapters_everywhere() {
    let tmp = copy_golden();
    // Removing outbound/ is safe — doesn't destroy inner hex path
    for dir in ALL_ADAPTERS_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| {
            e.title.contains("missing")
                && e.title.contains("outbound")
                && e.title.contains("adapters")
        })
        .collect();
    assert_eq!(
        r3.len(),
        4,
        "expected 4 errors (3 outer + 1 inner hex), got {}: {r3:#?}",
        r3.len()
    );
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_inbound_in_ports_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_PORTS_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/inbound"));
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| {
            e.title.contains("missing")
                && e.title.contains("inbound")
                && e.title.contains("ports")
        })
        .collect();
    assert_eq!(
        r3.len(),
        4,
        "expected 4 errors (3 outer + 1 inner hex), got {}: {r3:#?}",
        r3.len()
    );
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_outbound_in_ports_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_PORTS_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| {
            e.title.contains("missing")
                && e.title.contains("outbound")
                && e.title.contains("ports")
        })
        .collect();
    assert_eq!(
        r3.len(),
        4,
        "expected 4 errors (3 outer + 1 inner hex), got {}: {r3:#?}",
        r3.len()
    );
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_both_in_adapters_everywhere() {
    let tmp = copy_golden();
    // Removing both inbound/ and outbound/ from adapters/ makes the dir empty.
    // check_03 does `if entries.is_empty() { return; }` — so it early-returns.
    // Rule 3 does NOT fire when the structural dir is completely empty.
    // (The missing inbound/outbound would be caught, but only if the dir has
    // OTHER entries. Empty dir -> early return.)
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/outbound"));
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| {
            e.title.contains("missing")
                && (e.title.contains("inbound") || e.title.contains("outbound"))
                && e.title.contains("adapters")
        })
        .collect();
    // Empty adapters/ -> check_03 early-returns -> 0 Rule 3 errors
    assert_eq!(
        r3.len(),
        0,
        "expected 0 Rule 3 errors (empty adapters/ -> early return), got {}: {r3:#?}",
        r3.len()
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_both_in_ports_everywhere() {
    let tmp = copy_golden();
    // Removing both inbound/ and outbound/ from ports/ makes the dir empty.
    // check_03 does `if entries.is_empty() { return; }` — early return.
    // Same behavior as adapters/: no Rule 3 errors for empty structural dirs.
    for dir in ALL_PORTS_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/inbound"));
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| {
            e.title.contains("missing")
                && (e.title.contains("inbound") || e.title.contains("outbound"))
                && e.title.contains("ports")
        })
        .collect();
    assert_eq!(
        r3.len(),
        0,
        "expected 0 Rule 3 errors (empty ports/ -> early return), got {}: {r3:#?}",
        r3.len()
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_inbound_in_both_adapters_and_ports() {
    let tmp = copy_golden();
    // Removing adapters/inbound from backend outer destroys inner hex.
    // adapters/inbound missing: 3 outer (inner hex destroyed)
    // ports/inbound missing: 3 outer (inner hex destroyed for backend)
    // Total: 6
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
        remove_dir(tmp.path(), &format!("apps/{app}/crates/ports/inbound"));
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("missing") && e.title.contains("inbound"))
        .collect();
    assert_eq!(
        r3.len(),
        6,
        "expected 6 errors (3 adapters + 3 ports, inner hex destroyed), got {}: {r3:#?}",
        r3.len()
    );
    assert_per_app(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// Group B: Unexpected dirs
// ============================================================================

#[test]
fn unexpected_dir_in_adapters_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_ADAPTERS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/shared"))).expect("mkdir");
    }
    // Also add to TS app admin's adapters/ — should NOT be flagged
    std::fs::create_dir_all(
        tmp.path().join("apps/admin/src/modules/adapters/shared"),
    )
    .expect("mkdir");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("unexpected") && e.title.contains("shared"))
        .collect();
    assert_eq!(
        r3.len(),
        4,
        "expected 4 errors (1 per adapters/ location), got {}: {r3:#?}",
        r3.len()
    );
    for err in &r3 {
        assert!(
            err.title.contains("adapters"),
            "expected title mentioning 'adapters', got: '{}'",
            err.title
        );
    }
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn unexpected_dir_in_ports_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_PORTS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/common"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("unexpected") && e.title.contains("common"))
        .collect();
    assert_eq!(
        r3.len(),
        4,
        "expected 4 errors (1 per ports/ location), got {}: {r3:#?}",
        r3.len()
    );
    for err in &r3 {
        assert!(
            err.title.contains("ports"),
            "expected title mentioning 'ports', got: '{}'",
            err.title
        );
    }
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn unexpected_dir_wrong_case() {
    let tmp = copy_golden();
    // On case-insensitive FS (macOS), Inbound/ == inbound/. Detect and adapt.
    let test_path = tmp.path().join(format!("{}/Inbound", ALL_ADAPTERS_DIRS[0]));
    std::fs::create_dir_all(&test_path).expect("mkdir");
    let is_case_sensitive = std::fs::read_dir(tmp.path().join(ALL_ADAPTERS_DIRS[0]))
        .expect("readdir")
        .filter_map(|e| e.ok())
        .filter(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            n == "Inbound" || n == "inbound"
        })
        .count()
        == 2;

    if !is_case_sensitive {
        // Case-insensitive: use a distinct name that won't collide
        let tmp = copy_golden();
        for dir in ALL_ADAPTERS_DIRS {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/Incoming"))).expect("mkdir");
        }
        for dir in ALL_PORTS_DIRS {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/Incoming"))).expect("mkdir");
        }
        let results = run_check(tmp.path());
        let errors = arch_01_errors(&results);
        let r3: Vec<&CheckResult> = errors
            .iter()
            .copied()
            .filter(|e| e.title.contains("unexpected") && e.title.contains("Incoming"))
            .collect();
        assert_eq!(
            r3.len(),
            8,
            "expected 8 errors (Incoming/ in 4 adapters + 4 ports), got {}: {r3:#?}",
            r3.len()
        );
        assert_per_app(&r3);
        assert_inner_hex(&r3);
        assert_no_ts_apps(&errors);
        assert_no_packages(&errors);
    } else {
        // Case-sensitive: Inbound/ alongside inbound/ — Inbound is unexpected
        for dir in ALL_ADAPTERS_DIRS.iter().skip(1) {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/Inbound"))).expect("mkdir");
        }
        for dir in ALL_PORTS_DIRS {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/Inbound"))).expect("mkdir");
        }
        let results = run_check(tmp.path());
        let errors = arch_01_errors(&results);
        let r3: Vec<&CheckResult> = errors
            .iter()
            .copied()
            .filter(|e| e.title.contains("unexpected") && e.title.contains("Inbound"))
            .collect();
        assert_eq!(
            r3.len(),
            8,
            "expected 8 errors (Inbound/ in 4 adapters + 4 ports), got {}: {r3:#?}",
            r3.len()
        );
        assert_per_app(&r3);
        assert_inner_hex(&r3);
        assert_no_ts_apps(&errors);
        assert_no_packages(&errors);
    }
}

#[test]
fn multiple_unexpected_dirs() {
    let tmp = copy_golden();
    for dir in ALL_ADAPTERS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/shared"))).expect("mkdir");
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/common"))).expect("mkdir");
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/utils"))).expect("mkdir");
    }
    for dir in ALL_PORTS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/shared"))).expect("mkdir");
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/common"))).expect("mkdir");
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/utils"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("unexpected directory"))
        .collect();
    // 3 unexpected dirs * 8 structural dirs = 24 errors
    assert_eq!(
        r3.len(),
        24,
        "expected 24 errors (3 unexpected * 8 structural dirs), got {}: {r3:#?}",
        r3.len()
    );
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// Group C: Loose files
// ============================================================================

#[test]
fn loose_file_in_adapters_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_ADAPTERS_DIRS {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("loose files") && e.title.contains("adapters"))
        .collect();
    assert_eq!(
        r3.len(),
        4,
        "expected 4 errors (1 per adapters/ location), got {}: {r3:#?}",
        r3.len()
    );
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn loose_file_in_ports_everywhere() {
    let tmp = copy_golden();
    for dir in ALL_PORTS_DIRS {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("loose files") && e.title.contains("ports"))
        .collect();
    assert_eq!(
        r3.len(),
        4,
        "expected 4 errors (1 per ports/ location), got {}: {r3:#?}",
        r3.len()
    );
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn multiple_loose_files() {
    let tmp = copy_golden();
    // Add mod.rs + lib.rs to all 8 structural dirs
    for dir in ALL_ADAPTERS_DIRS {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
        write_file(tmp.path(), &format!("{dir}/lib.rs"), "// stray");
    }
    for dir in ALL_PORTS_DIRS {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
        write_file(tmp.path(), &format!("{dir}/lib.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("loose files"))
        .collect();
    // Loose files produce 1 error PER DIRECTORY (listing all bad files in message),
    // not 1 per file. So: 8 structural dirs = 8 errors.
    assert_eq!(
        r3.len(),
        8,
        "expected 8 errors (1 per structural dir, not per file), got {}: {r3:#?}",
        r3.len()
    );
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn gitkeep_allowed() {
    let tmp = copy_golden();
    // Add .gitkeep to all structural dirs — should produce 0 Rule 3 errors
    for dir in ALL_ADAPTERS_DIRS {
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
    }
    for dir in ALL_PORTS_DIRS {
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        0,
        "expected 0 errors (.gitkeep is allowed), got: {errors:#?}"
    );
}

#[test]
fn gitkeep_alongside_loose_files() {
    let tmp = copy_golden();
    // .gitkeep + mod.rs — only mod.rs should be flagged
    for dir in ALL_ADAPTERS_DIRS {
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    for dir in ALL_PORTS_DIRS {
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("loose files"))
        .collect();
    assert_eq!(
        r3.len(),
        8,
        "expected 8 errors (mod.rs flagged, .gitkeep not), got {}: {r3:#?}",
        r3.len()
    );
    // Verify the message mentions mod.rs in the bad-files listing.
    // The message format is: "...files in ... that don't belong: {files}. Only `.gitkeep`..."
    // So .gitkeep appears in the instructional suffix, but NOT in the listed files.
    for err in &r3 {
        assert!(
            err.message.contains("mod.rs"),
            "expected message mentioning 'mod.rs', got: '{}'",
            err.message
        );
        // Extract the "don't belong:" portion and verify .gitkeep is not listed as a bad file
        if let Some(listing_start) = err.message.find("don't belong: ") {
            let after = &err.message[listing_start + 14..];
            let listing_end = after.find('.').unwrap_or(after.len());
            let listing = &after[..listing_end];
            assert!(
                !listing.contains(".gitkeep"),
                ".gitkeep should NOT be in the bad-files listing '{listing}', full message: '{}'",
                err.message
            );
        }
    }
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// Group D: Combinations
// ============================================================================

#[test]
fn missing_plus_unexpected_plus_loose() {
    let tmp = copy_golden();
    // All 3 violation types in all structural dirs:
    // - Remove inbound/ (missing)
    // - Add shared/ (unexpected)
    // - Add mod.rs (loose)
    //
    // Removing adapters/inbound from backend outer destroys inner hex.
    // adapters/: 3 outer * 3 violations = 9
    // ports/: 3 outer * 3 violations = 9 (inner hex destroyed)
    // Total Rule 3: 18
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
        std::fs::create_dir_all(
            tmp.path().join(format!("apps/{app}/crates/adapters/shared")),
        )
        .expect("mkdir");
        write_file(
            tmp.path(),
            &format!("apps/{app}/crates/adapters/mod.rs"),
            "// stray",
        );

        remove_dir(tmp.path(), &format!("apps/{app}/crates/ports/inbound"));
        std::fs::create_dir_all(
            tmp.path().join(format!("apps/{app}/crates/ports/shared")),
        )
        .expect("mkdir");
        write_file(
            tmp.path(),
            &format!("apps/{app}/crates/ports/mod.rs"),
            "// stray",
        );
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        18,
        "expected 18 Rule 3 errors (3 violation types * 6 surviving structural dirs), got {}: {r3:#?}",
        r3.len()
    );
    // Verify all 3 violation types present
    assert!(
        r3.iter().any(|e| e.title.contains("missing")),
        "expected at least one 'missing' error, got: {r3:#?}"
    );
    assert!(
        r3.iter().any(|e| e.title.contains("unexpected")),
        "expected at least one 'unexpected' error, got: {r3:#?}"
    );
    assert!(
        r3.iter().any(|e| e.title.contains("loose files")),
        "expected at least one 'loose files' error, got: {r3:#?}"
    );
    assert_per_app(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn different_breakage_per_structural_dir() {
    let tmp = copy_golden();
    // adapters/: missing inbound (devctl only)
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/inbound");
    // ports/: unexpected shared/ (worker only)
    std::fs::create_dir_all(
        tmp.path().join("apps/worker/crates/ports/shared"),
    )
    .expect("mkdir");
    // inner hex adapters/: loose file
    write_file(
        tmp.path(),
        &format!("{INNER_HEX}/adapters/mod.rs"),
        "// stray",
    );
    // inner hex ports/: missing outbound
    remove_dir(tmp.path(), &format!("{INNER_HEX}/ports/outbound"));

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        4,
        "expected 4 Rule 3 errors (one per breakage), got {}: {r3:#?}",
        r3.len()
    );
    // devctl: missing adapters/inbound
    assert!(
        r3.iter().any(|e| e.title.contains("devctl")
            && e.title.contains("missing")
            && e.title.contains("inbound")),
        "expected devctl missing inbound error, got: {r3:#?}"
    );
    // worker: unexpected shared in ports
    assert!(
        r3.iter().any(|e| e.title.contains("worker")
            && e.title.contains("unexpected")
            && e.title.contains("shared")),
        "expected worker unexpected shared error, got: {r3:#?}"
    );
    // inner hex adapters: loose files
    assert!(
        r3.iter().any(|e| e.title.contains("backend")
            && e.title.contains("loose files")
            && e.title.contains("adapters")),
        "expected backend inner hex loose files error, got: {r3:#?}"
    );
    // inner hex ports: missing outbound
    assert!(
        r3.iter().any(|e| e.title.contains("backend")
            && e.title.contains("missing")
            && e.title.contains("outbound")),
        "expected backend inner hex missing outbound error, got: {r3:#?}"
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// Group E: Edge cases
// ============================================================================

#[test]
fn renamed_inbound_to_incoming() {
    let tmp = copy_golden();
    // Rename inbound/ to incoming/ in all adapters/ and ports/
    // This produces: missing inbound + unexpected incoming per dir
    //
    // Removing adapters/inbound from backend outer destroys inner hex.
    // adapters: 3 outer * 2 (missing + unexpected) = 6
    // ports: 3 outer * 2 = 6 (inner hex destroyed)
    // Total: 12
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
        std::fs::create_dir_all(
            tmp.path().join(format!("apps/{app}/crates/adapters/incoming")),
        )
        .expect("mkdir");
        remove_dir(tmp.path(), &format!("apps/{app}/crates/ports/inbound"));
        std::fs::create_dir_all(
            tmp.path().join(format!("apps/{app}/crates/ports/incoming")),
        )
        .expect("mkdir");
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        12,
        "expected 12 Rule 3 errors (missing + unexpected per surviving dir), got {}: {r3:#?}",
        r3.len()
    );
    let missing: Vec<_> = r3.iter().filter(|e| e.title.contains("missing")).collect();
    let unexpected: Vec<_> = r3.iter().filter(|e| e.title.contains("unexpected")).collect();
    assert_eq!(missing.len(), 6, "expected 6 missing errors, got: {missing:#?}");
    assert_eq!(unexpected.len(), 6, "expected 6 unexpected errors, got: {unexpected:#?}");
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn inner_hex_structural_broken_outer_clean() {
    let tmp = copy_golden();
    // Only break inner hex adapters/ and ports/ — outer apps stay clean
    remove_dir(tmp.path(), &format!("{INNER_HEX}/adapters/inbound"));
    remove_dir(tmp.path(), &format!("{INNER_HEX}/ports/outbound"));
    std::fs::create_dir_all(
        tmp.path().join(format!("{INNER_HEX}/adapters/shared")),
    )
    .expect("mkdir");
    write_file(tmp.path(), &format!("{INNER_HEX}/ports/mod.rs"), "// stray");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    // Inner hex adapters: missing inbound + unexpected shared = 2
    // Inner hex ports: missing outbound + loose mod.rs = 2
    assert_eq!(
        r3.len(),
        4,
        "expected 4 Rule 3 errors (inner hex only), got {}: {r3:#?}",
        r3.len()
    );
    // All errors should be for backend
    for err in &r3 {
        assert!(
            err.title.contains("backend"),
            "expected error for backend, got: '{}'",
            err.title
        );
    }
    // No devctl or worker errors
    assert!(
        !r3.iter().any(|e| e.title.contains("devctl")),
        "devctl should be clean, got: {r3:#?}"
    );
    assert!(
        !r3.iter().any(|e| e.title.contains("worker")),
        "worker should be clean, got: {r3:#?}"
    );
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn inner_hex_label_prefix_correct() {
    let tmp = copy_golden();
    // Add unexpected dir to inner hex adapters/ and ports/
    std::fs::create_dir_all(
        tmp.path().join(format!("{INNER_HEX}/adapters/shared")),
    )
    .expect("mkdir");
    std::fs::create_dir_all(
        tmp.path().join(format!("{INNER_HEX}/ports/shared")),
    )
    .expect("mkdir");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("unexpected") && e.title.contains("shared"))
        .collect();
    assert_eq!(r3.len(), 2, "expected 2 errors (inner hex only), got: {r3:#?}");
    // Verify title contains the full nested label path (mcp/crates)
    for err in &r3 {
        assert!(
            err.title.contains("mcp/crates"),
            "expected title containing 'mcp/crates' for inner hex label_prefix, got: '{}'",
            err.title
        );
    }
    // File field should point to inner hex path
    for err in &r3 {
        let file = err.file.as_deref().unwrap_or("");
        assert!(
            file.contains("mcp/crates"),
            "expected file field containing 'mcp/crates', got: '{file}'"
        );
    }
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn empty_structural_dir_early_return() {
    let tmp = copy_golden();
    // Remove entire adapters/ dir from devctl. Rule 2 catches the missing dir.
    // Rule 3 should early-return because list_dir on missing dir returns empty.
    remove_dir(tmp.path(), "apps/devctl/crates/adapters");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    // Rule 2 fires for missing adapters/ = 1 error
    // Rule 3 should NOT fire because the dir doesn't exist
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        0,
        "expected 0 Rule 3 errors (adapters/ missing -> early return), got: {r3:#?}"
    );
    // But Rule 2 should have fired
    assert!(
        errors.iter().any(|e| e.title.contains("missing") && e.title.contains("adapters")),
        "expected Rule 2 error for missing adapters/, got: {errors:#?}"
    );
}

#[test]
fn idempotent_results() {
    let tmp = copy_golden();
    // Break something — add unexpected dir to all adapters/ dirs
    for dir in ALL_ADAPTERS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/shared"))).expect("mkdir");
    }
    let results1 = run_check(tmp.path());
    let errors1 = arch_01_errors(&results1);
    let results2 = run_check(tmp.path());
    let errors2 = arch_01_errors(&results2);
    assert_eq!(
        errors1.len(),
        errors2.len(),
        "idempotent check failed: run 1 produced {} errors, run 2 produced {}",
        errors1.len(),
        errors2.len()
    );
    // Verify same titles (sorted for determinism)
    let mut titles1: Vec<_> = errors1.iter().map(|e| &e.title).collect();
    let mut titles2: Vec<_> = errors2.iter().map(|e| &e.title).collect();
    titles1.sort();
    titles2.sort();
    assert_eq!(titles1, titles2, "idempotent check failed: different error titles");
}

#[test]
fn packages_not_checked() {
    let tmp = copy_golden();
    // Break packages/ structure — add adapters/ and ports/ with wrong contents
    std::fs::create_dir_all(tmp.path().join("packages/shared-types/adapters/wrong")).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("packages/ui-kit/ports/wrong")).expect("mkdir");
    write_file(
        tmp.path(),
        "packages/shared-types/adapters/mod.rs",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        0,
        "expected 0 errors (packages should not be checked), got: {errors:#?}"
    );
}

#[test]
fn ts_apps_not_checked() {
    let tmp = copy_golden();
    // Break TS apps' module structure with adapters/ports violations
    std::fs::create_dir_all(
        tmp.path().join("apps/admin/src/modules/adapters/wrong"),
    )
    .expect("mkdir");
    std::fs::create_dir_all(
        tmp.path().join("apps/admin/src/modules/ports/wrong"),
    )
    .expect("mkdir");
    write_file(
        tmp.path(),
        "apps/admin/src/modules/adapters/mod.rs",
        "// stray",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert_eq!(
        errors.len(),
        0,
        "expected 0 RS errors for TS app breakage, got: {errors:#?}"
    );
}
