use super::helpers::{arch_01_errors, copy_golden, remove_dir, run_check, write_file};
use guardrail3::domain::report::CheckResult;

const RUST_APPS: &[&str] = &["devctl", "backend", "worker"];
const INNER_HEX: &str = "apps/backend/crates/adapters/inbound/mcp/crates";

const ALL_ADAPTERS_DIRS: &[&str] = &[
    "apps/devctl/crates/adapters",
    "apps/backend/crates/adapters",
    "apps/worker/crates/adapters",
    "apps/backend/crates/adapters/inbound/mcp/crates/adapters",
];
const ALL_PORTS_DIRS: &[&str] = &[
    "apps/devctl/crates/ports",
    "apps/backend/crates/ports",
    "apps/worker/crates/ports",
    "apps/backend/crates/adapters/inbound/mcp/crates/ports",
];

fn all_structural_dirs() -> Vec<String> {
    ALL_ADAPTERS_DIRS
        .iter()
        .chain(ALL_PORTS_DIRS.iter())
        .map(|s| s.to_string())
        .collect()
}

fn assert_per_app(errors: &[&CheckResult]) {
    for app in RUST_APPS {
        assert!(
            errors.iter().any(|e| e.title.contains(app)),
            "expected error for {app}"
        );
    }
}

fn assert_inner_hex(errors: &[&CheckResult]) {
    assert!(
        errors
            .iter()
            .any(|e| e.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "expected inner hex error"
    );
}

fn assert_no_ts_apps(errors: &[&CheckResult]) {
    assert!(
        !errors.iter().any(|e| {
            let t = &e.title;
            t.contains("admin") || t.contains("landing")
        }),
        "TS apps should not be flagged"
    );
}

fn assert_no_packages(errors: &[&CheckResult]) {
    assert!(
        !errors.iter().any(|e| {
            let t = &e.title;
            t.contains("shared-types") || t.contains("ui-kit")
        }),
        "packages should not be flagged"
    );
}

fn assert_file_field(errors: &[&CheckResult]) {
    for err in errors {
        let file = err.file.as_deref().unwrap_or("");
        assert!(!file.is_empty(), "error file field should not be empty: {err:?}");
    }
}

fn rule3_errors<'a>(errors: &'a [&'a CheckResult]) -> Vec<&'a CheckResult> {
    errors
        .iter()
        .filter(|e| {
            let t = &e.title;
            (t.contains("missing") && (t.contains("inbound") || t.contains("outbound")))
                || (t.contains("unexpected") && (t.contains("adapters") || t.contains("ports")))
                || (t.contains("loose files") && (t.contains("adapters") || t.contains("ports")))
        })
        .copied()
        .collect()
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
    assert_file_field(&r3);
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
    assert_file_field(&r3);
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
    assert_file_field(&r3);
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
    assert_file_field(&r3);
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
    // Ports should be clean (no ports Rule 3 errors)
    let ports_r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| {
            e.title.contains("ports")
                && (e.title.contains("missing") || e.title.contains("unexpected") || e.title.contains("loose files"))
        })
        .collect();
    assert_eq!(
        ports_r3.len(),
        0,
        "expected 0 ports Rule 3 errors, got {}: {ports_r3:#?}",
        ports_r3.len()
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_both_in_ports_everywhere() {
    let tmp = copy_golden();
    // Removing both inbound/ and outbound/ from ports/ makes the dir empty.
    // check_03 does `if entries.is_empty() { return; }` — early return.
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
    // Adapters should be clean (no adapters Rule 3 errors)
    let adapters_r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| {
            e.title.contains("adapters")
                && (e.title.contains("missing") || e.title.contains("unexpected") || e.title.contains("loose files"))
        })
        .collect();
    assert_eq!(
        adapters_r3.len(),
        0,
        "expected 0 adapters Rule 3 errors, got {}: {adapters_r3:#?}",
        adapters_r3.len()
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_inbound_in_both_adapters_and_ports() {
    let tmp = copy_golden();
    // Removing adapters/inbound from backend outer destroys inner hex.
    // adapters/inbound missing: 3 outer (inner hex destroyed)
    // ports/inbound missing: 3 outer (inner hex ports also destroyed)
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
    assert_file_field(&r3);
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
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/adapters/shared"))
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
    assert_file_field(&r3);
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
    assert_file_field(&r3);
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
        assert_file_field(&r3);
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
        assert_file_field(&r3);
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
    assert_file_field(&r3);
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
    assert_file_field(&r3);
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
    assert_file_field(&r3);
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn multiple_loose_files() {
    let tmp = copy_golden();
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
    assert_file_field(&r3);
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn gitkeep_allowed() {
    let tmp = copy_golden();
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
    for err in &r3 {
        assert!(
            err.message.contains("mod.rs"),
            "expected message mentioning 'mod.rs', got: '{}'",
            err.message
        );
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
    assert_file_field(&r3);
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
    assert_file_field(&r3);
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
    std::fs::create_dir_all(tmp.path().join("apps/worker/crates/ports/shared"))
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
    // inner hex adapters: loose files — must reference mcp/crates in title
    assert!(
        r3.iter().any(|e| e.title.contains("mcp/crates")
            && e.title.contains("loose files")
            && e.title.contains("adapters")),
        "expected inner hex loose files error with 'mcp/crates' in title, got: {r3:#?}"
    );
    // inner hex ports: missing outbound — must reference mcp/crates in title
    assert!(
        r3.iter().any(|e| e.title.contains("mcp/crates")
            && e.title.contains("missing")
            && e.title.contains("outbound")),
        "expected inner hex missing outbound error with 'mcp/crates' in title, got: {r3:#?}"
    );
    assert_file_field(&r3);
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
    assert_per_app(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn inner_hex_structural_broken_outer_clean() {
    let tmp = copy_golden();
    // Only break inner hex adapters/ and ports/ — outer apps stay clean
    remove_dir(tmp.path(), &format!("{INNER_HEX}/adapters/inbound"));
    remove_dir(tmp.path(), &format!("{INNER_HEX}/ports/outbound"));
    std::fs::create_dir_all(tmp.path().join(format!("{INNER_HEX}/adapters/shared")))
        .expect("mkdir");
    write_file(tmp.path(), &format!("{INNER_HEX}/ports/mod.rs"), "// stray");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        4,
        "expected 4 Rule 3 errors (inner hex only), got {}: {r3:#?}",
        r3.len()
    );
    for err in &r3 {
        assert!(
            err.title.contains("backend"),
            "expected error for backend, got: '{}'",
            err.title
        );
    }
    assert!(
        !r3.iter().any(|e| e.title.contains("devctl")),
        "devctl should be clean, got: {r3:#?}"
    );
    assert!(
        !r3.iter().any(|e| e.title.contains("worker")),
        "worker should be clean, got: {r3:#?}"
    );
    assert_file_field(&r3);
    assert_inner_hex(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn inner_hex_label_prefix_correct() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join(format!("{INNER_HEX}/adapters/shared")))
        .expect("mkdir");
    std::fs::create_dir_all(tmp.path().join(format!("{INNER_HEX}/ports/shared")))
        .expect("mkdir");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("unexpected") && e.title.contains("shared"))
        .collect();
    assert_eq!(r3.len(), 2, "expected 2 errors (inner hex only), got: {r3:#?}");
    for err in &r3 {
        assert!(
            err.title.contains("mcp/crates"),
            "expected title containing 'mcp/crates' for inner hex label_prefix, got: '{}'",
            err.title
        );
    }
    for err in &r3 {
        let file = err.file.as_deref().unwrap_or("");
        assert!(
            file.contains("mcp/crates"),
            "expected file field containing 'mcp/crates', got: '{file}'"
        );
    }
    assert_file_field(&r3);
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
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        0,
        "expected 0 Rule 3 errors (adapters/ missing -> early return), got: {r3:#?}"
    );
    // But Rule 2 should have fired
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("missing") && e.title.contains("adapters")),
        "expected Rule 2 error for missing adapters/, got: {errors:#?}"
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn idempotent_results() {
    let tmp = copy_golden();
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
    let mut titles1: Vec<_> = errors1.iter().map(|e| &e.title).collect();
    let mut titles2: Vec<_> = errors2.iter().map(|e| &e.title).collect();
    titles1.sort();
    titles2.sort();
    assert_eq!(titles1, titles2, "idempotent check failed: different error titles");
    assert_no_ts_apps(&errors1);
    assert_no_packages(&errors1);
}

#[test]
fn packages_not_checked() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("packages/shared-types/adapters/wrong"))
        .expect("mkdir");
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
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn ts_apps_not_checked() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/adapters/wrong"))
        .expect("mkdir");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/ports/wrong"))
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
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// ============================================================================
// Group F: New edge-case and boundary tests
// ============================================================================

#[test]
fn required_dir_replaced_with_file() {
    let tmp = copy_golden();
    // Replace inbound/ directories with a file named "inbound".
    // list_dir_names only counts directories, so a file named "inbound" won't satisfy
    // the "inbound" dir check -> missing inbound. The file -> loose file error.
    //
    // Removing adapters/inbound from backend outer destroys inner hex entirely.
    // Inner hex ports also gone (under adapters/inbound/mcp/crates/ports).
    // Adapters: 3 outer * (1 missing + 1 loose) = 6
    // Ports: 3 outer * (1 missing + 1 loose) = 6 (inner hex destroyed)
    // Total: 12
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
        write_file(
            tmp.path(),
            &format!("apps/{app}/crates/adapters/inbound"),
            "// not a dir",
        );
        remove_dir(tmp.path(), &format!("apps/{app}/crates/ports/inbound"));
        write_file(
            tmp.path(),
            &format!("apps/{app}/crates/ports/inbound"),
            "// not a dir",
        );
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        12,
        "expected 12 Rule 3 errors (missing + loose per surviving dir), got {}: {r3:#?}",
        r3.len()
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn required_dir_replaced_with_symlink() {
    let tmp = copy_golden();
    // Replace inbound/ with symlinks pointing to outbound/.
    // DirEntry::file_type() does NOT follow symlinks on Unix — symlinks show as
    // is_symlink(), not is_dir(). So list_dir_names won't include them.
    // Same effect as file: missing inbound + loose "inbound" symlink.
    //
    // Removing adapters/inbound from backend outer destroys inner hex entirely.
    // Inner hex ports also gone (under adapters/inbound/mcp/crates/ports).
    // Adapters: 3 outer * 2 = 6
    // Ports: 3 outer * 2 = 6 (inner hex destroyed)
    // Total: 12
    for app in RUST_APPS {
        let adapters = tmp.path().join(format!("apps/{app}/crates/adapters"));
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
        std::os::unix::fs::symlink(adapters.join("outbound"), adapters.join("inbound"))
            .expect("symlink");
        let ports = tmp.path().join(format!("apps/{app}/crates/ports"));
        remove_dir(tmp.path(), &format!("apps/{app}/crates/ports/inbound"));
        std::os::unix::fs::symlink(ports.join("outbound"), ports.join("inbound"))
            .expect("symlink");
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        12,
        "expected 12 Rule 3 errors (missing + loose per surviving dir), got {}: {r3:#?}",
        r3.len()
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn unicode_lookalike_dir_name() {
    let tmp = copy_golden();
    // "i\u{200B}nbound" (zero-width space) alongside real inbound/ = unexpected dir
    let bad_name = "i\u{200B}nbound";
    for dir in ALL_ADAPTERS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/{bad_name}"))).expect("mkdir");
    }
    for dir in ALL_PORTS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/{bad_name}"))).expect("mkdir");
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("unexpected"))
        .collect();
    assert_eq!(
        r3.len(),
        8,
        "expected 8 unexpected errors (unicode lookalike in 8 dirs), got {}: {r3:#?}",
        r3.len()
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn gitkeep_as_directory() {
    let tmp = copy_golden();
    // mkdir .gitkeep (as a directory, not a file) in all structural dirs.
    // list_dir_names will see ".gitkeep" as a dir -> unexpected directory.
    for dir in ALL_ADAPTERS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/.gitkeep"))).expect("mkdir");
    }
    for dir in ALL_PORTS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/.gitkeep"))).expect("mkdir");
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("unexpected") && e.title.contains(".gitkeep"))
        .collect();
    assert_eq!(
        r3.len(),
        8,
        "expected 8 unexpected '.gitkeep' dir errors, got {}: {r3:#?}",
        r3.len()
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn hidden_dir_unexpected() {
    let tmp = copy_golden();
    for dir in ALL_ADAPTERS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/.hidden"))).expect("mkdir");
    }
    for dir in ALL_PORTS_DIRS {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/.hidden"))).expect("mkdir");
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("unexpected") && e.title.contains(".hidden"))
        .collect();
    assert_eq!(
        r3.len(),
        8,
        "expected 8 unexpected '.hidden' dir errors, got {}: {r3:#?}",
        r3.len()
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn file_coexists_with_same_named_dir() {
    let tmp = copy_golden();
    // Add loose files "inbound.bak" and "outbound.rs" to all 8 structural dirs.
    // These are files, not dirs, so they produce 1 loose-files error per dir
    // (listing both files in the message).
    for dir in all_structural_dirs() {
        write_file(tmp.path(), &format!("{dir}/inbound.bak"), "// backup");
        write_file(tmp.path(), &format!("{dir}/outbound.rs"), "// stray");
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
        "expected 8 loose-files errors (1 per structural dir), got {}: {r3:#?}",
        r3.len()
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn nested_unexpected_dir_tree() {
    let tmp = copy_golden();
    // shared/deep/deeper/ in all 8 dirs. Only "shared" should be unexpected
    // (check_03 only scans immediate children).
    for dir in all_structural_dirs() {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/shared/deep/deeper")))
            .expect("mkdir");
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("unexpected") && e.title.contains("shared"))
        .collect();
    assert_eq!(
        r3.len(),
        8,
        "expected 8 unexpected 'shared' errors (nested dirs not separate errors), got {}: {r3:#?}",
        r3.len()
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn near_miss_names_comprehensive() {
    let tmp = copy_golden();
    // "inbounds/", "outboud/", "input/", "output/" in all 8 dirs = 4 * 8 = 32 unexpected
    let bad_names = &["inbounds", "outboud", "input", "output"];
    for dir in all_structural_dirs() {
        for name in bad_names {
            std::fs::create_dir_all(tmp.path().join(format!("{dir}/{name}"))).expect("mkdir");
        }
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("unexpected directory"))
        .collect();
    assert_eq!(
        r3.len(),
        32,
        "expected 32 unexpected errors (4 bad names * 8 dirs), got {}: {r3:#?}",
        r3.len()
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn permission_denied_structural_dir() {
    let tmp = copy_golden();
    let devctl_adapters = tmp.path().join("apps/devctl/crates/adapters");
    // chmod 000 on devctl adapters/ — list_dir returns empty -> early return
    let original_perms =
        std::fs::metadata(&devctl_adapters).expect("metadata").permissions();
    let mut no_perms = original_perms.clone();
    std::os::unix::fs::PermissionsExt::set_mode(&mut no_perms, 0o000);
    std::fs::set_permissions(&devctl_adapters, no_perms).expect("chmod");

    let results = run_check(tmp.path());

    // Restore permissions before any assertions (so cleanup works even if test fails)
    std::fs::set_permissions(&devctl_adapters, original_perms).expect("restore chmod");

    let errors = arch_01_errors(&results);
    // devctl adapters/ is unreadable -> list_dir returns empty -> check_03 early return.
    // No Rule 3 error for devctl adapters.
    let devctl_adapters_r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| {
            e.title.contains("devctl")
                && e.title.contains("adapters")
                && (e.title.contains("missing") || e.title.contains("unexpected") || e.title.contains("loose"))
        })
        .collect();
    assert_eq!(
        devctl_adapters_r3.len(),
        0,
        "expected 0 Rule 3 errors for unreadable devctl adapters/, got {}: {devctl_adapters_r3:#?}",
        devctl_adapters_r3.len()
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn new_app_gets_checked() {
    let tmp = copy_golden();
    // Create apps/scheduler/ with Cargo.toml + broken crates/adapters/
    let sched = "apps/scheduler";
    write_file(tmp.path(), &format!("{sched}/Cargo.toml"), "[package]\nname = \"scheduler\"");
    std::fs::create_dir_all(tmp.path().join(format!("{sched}/crates/adapters/shared")))
        .expect("mkdir");
    std::fs::create_dir_all(tmp.path().join(format!("{sched}/crates/ports/inbound")))
        .expect("mkdir");
    std::fs::create_dir_all(tmp.path().join(format!("{sched}/crates/ports/outbound")))
        .expect("mkdir");
    std::fs::create_dir_all(tmp.path().join(format!("{sched}/crates/app"))).expect("mkdir");
    std::fs::create_dir_all(tmp.path().join(format!("{sched}/crates/domain"))).expect("mkdir");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let sched_r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("scheduler"))
        .collect();
    // scheduler adapters/ has: shared (unexpected), missing inbound, missing outbound
    assert!(
        sched_r3.iter().any(|e| e.title.contains("missing") && e.title.contains("inbound")),
        "expected scheduler missing inbound error, got: {sched_r3:#?}"
    );
    assert!(
        sched_r3.iter().any(|e| e.title.contains("unexpected") && e.title.contains("shared")),
        "expected scheduler unexpected shared error, got: {sched_r3:#?}"
    );
    assert_file_field(&sched_r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn maximally_complex_single_structural_dir() {
    let tmp = copy_golden();
    // In devctl adapters/: symlink for inbound (replaces dir), keep outbound, add shared/, add mod.rs, .gitkeep
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/inbound");
    let adapters = tmp.path().join("apps/devctl/crates/adapters");
    std::os::unix::fs::symlink(adapters.join("outbound"), adapters.join("inbound"))
        .expect("symlink");
    std::fs::create_dir_all(adapters.join("shared")).expect("mkdir");
    write_file(tmp.path(), "apps/devctl/crates/adapters/mod.rs", "// stray");
    write_file(tmp.path(), "apps/devctl/crates/adapters/.gitkeep", "");

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let devctl_adapters: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("devctl") && e.title.contains("adapters"))
        .collect();
    let r3 = rule3_errors(&devctl_adapters);
    // symlink inbound -> missing inbound (symlink not a dir)
    // symlink inbound -> loose file "inbound" (non-dir entry, not .gitkeep)
    // shared/ -> unexpected
    // mod.rs -> loose (grouped with symlink "inbound" in one loose error)
    // .gitkeep -> allowed
    // So: 1 missing + 1 unexpected + 1 loose = 3
    assert!(
        r3.iter().any(|e| e.title.contains("missing")),
        "expected missing inbound error, got: {r3:#?}"
    );
    assert!(
        r3.iter().any(|e| e.title.contains("unexpected")),
        "expected unexpected shared error, got: {r3:#?}"
    );
    assert!(
        r3.iter().any(|e| e.title.contains("loose files")),
        "expected loose files error, got: {r3:#?}"
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn gitkeep_only_structural_dir() {
    let tmp = copy_golden();
    // Remove inbound + outbound from all 8 dirs, add .gitkeep so dir is non-empty.
    // Non-empty dir -> check_03 runs -> detects missing inbound + outbound = 2 per dir.
    // Removing adapters/inbound from backend outer destroys inner hex.
    // Adapters: 3 outer * 2 = 6 (inner hex destroyed)
    // Ports: 3 outer * 2 = 6 (inner hex ports also destroyed since adapters/inbound gone)
    // Total: 12
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/outbound"));
        write_file(tmp.path(), &format!("apps/{app}/crates/adapters/.gitkeep"), "");
        remove_dir(tmp.path(), &format!("apps/{app}/crates/ports/inbound"));
        remove_dir(tmp.path(), &format!("apps/{app}/crates/ports/outbound"));
        write_file(tmp.path(), &format!("apps/{app}/crates/ports/.gitkeep"), "");
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    // 3 apps * 2 structural types (adapters + ports) * 2 missing (inbound + outbound) = 12
    // Inner hex destroyed so 0 from there.
    assert_eq!(
        r3.len(),
        12,
        "expected 12 missing errors (2 per surviving structural dir), got {}: {r3:#?}",
        r3.len()
    );
    // All should be "missing" errors (no unexpected, no loose)
    for err in &r3 {
        assert!(
            err.title.contains("missing"),
            "expected only 'missing' errors, got: '{}'",
            err.title
        );
    }
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn empty_required_dir_still_present() {
    let tmp = copy_golden();
    // Empty out inbound/ contents in all 8 dirs (remove subdirs but keep inbound/ itself).
    // inbound/ still exists as a dir -> list_dir_names sees it -> no "missing" error.
    // The contents of inbound/ are checked by other rules, not rule 3.
    for dir in all_structural_dirs() {
        let inbound = tmp.path().join(format!("{dir}/inbound"));
        if inbound.exists() {
            // Remove all children
            for entry in std::fs::read_dir(&inbound).expect("readdir") {
                let entry = entry.expect("entry");
                let path = entry.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(&path).expect("rmdir");
                } else {
                    std::fs::remove_file(&path).expect("rm");
                }
            }
        }
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let missing_inbound: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("missing") && e.title.contains("inbound"))
        .collect();
    assert_eq!(
        missing_inbound.len(),
        0,
        "expected 0 missing-inbound errors (empty dir still present), got {}: {missing_inbound:#?}",
        missing_inbound.len()
    );
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn loose_file_inner_hex_only() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        &format!("{INNER_HEX}/adapters/mod.rs"),
        "// stray",
    );

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3: Vec<&CheckResult> = errors
        .iter()
        .copied()
        .filter(|e| e.title.contains("loose files"))
        .collect();
    assert_eq!(
        r3.len(),
        1,
        "expected 1 loose-files error (inner hex adapters only), got {}: {r3:#?}",
        r3.len()
    );
    let file = r3[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("mcp/crates"),
        "expected file containing 'mcp/crates', got: '{file}'"
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn unexpected_dir_inner_hex_only() {
    let tmp = copy_golden();
    std::fs::create_dir_all(tmp.path().join(format!("{INNER_HEX}/ports/shared")))
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
        1,
        "expected 1 unexpected error (inner hex ports only), got {}: {r3:#?}",
        r3.len()
    );
    let file = r3[0].file.as_deref().unwrap_or("");
    assert!(
        file.contains("mcp/crates"),
        "expected file containing 'mcp/crates', got: '{file}'"
    );
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_plus_unexpected_combo() {
    let tmp = copy_golden();
    // Remove inbound/ + add shared/ in all surviving dirs.
    // Removing adapters/inbound from backend outer destroys inner hex.
    // Adapters: 3 outer * (1 missing + 1 unexpected) = 6
    // Ports: 4 * (1 unexpected) = 4, but ports/inbound also removed: 3 outer + 1 inner_hex
    //   -> removing ports inbound doesn't destroy inner hex (inner hex is under adapters path)
    //   Wait: inner hex ports is at apps/backend/crates/adapters/inbound/mcp/crates/ports
    //   We removed adapters/inbound from backend -> inner hex path destroyed entirely.
    //   So inner hex ports is also gone.
    // Adapters: 3 outer * 2 = 6
    // Ports: 3 outer * 2 = 6 (inner hex destroyed)
    // Total: 12
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
        std::fs::create_dir_all(
            tmp.path().join(format!("apps/{app}/crates/adapters/shared")),
        )
        .expect("mkdir");
        remove_dir(tmp.path(), &format!("apps/{app}/crates/ports/inbound"));
        std::fs::create_dir_all(
            tmp.path().join(format!("apps/{app}/crates/ports/shared")),
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
    assert_eq!(missing.len(), 6, "expected 6 missing, got: {missing:#?}");
    assert_eq!(unexpected.len(), 6, "expected 6 unexpected, got: {unexpected:#?}");
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn missing_plus_loose_combo() {
    let tmp = copy_golden();
    // Remove outbound/ + add mod.rs in all 8 dirs.
    // Outbound removal doesn't destroy inner hex.
    // Each dir: 1 missing outbound + 1 loose mod.rs = 2.
    // 8 dirs * 2 = 16
    for dir in ALL_ADAPTERS_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }
    for dir in ALL_PORTS_DIRS {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }

    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(
        r3.len(),
        16,
        "expected 16 Rule 3 errors (missing + loose per dir), got {}: {r3:#?}",
        r3.len()
    );
    let missing: Vec<_> = r3.iter().filter(|e| e.title.contains("missing")).collect();
    let loose: Vec<_> = r3.iter().filter(|e| e.title.contains("loose files")).collect();
    assert_eq!(missing.len(), 8, "expected 8 missing, got: {missing:#?}");
    assert_eq!(loose.len(), 8, "expected 8 loose, got: {loose:#?}");
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

// -----------------------------------------------------------------------
// Round 3: parity gaps + scenario gaps
// -----------------------------------------------------------------------

#[test]
fn missing_inbound_inner_hex_only() {
    // Parity with rule_02's missing_adapters_inner_hex_only
    // Remove only inbound/ from inner hex adapters/, nothing else
    let tmp = copy_golden();
    remove_dir(tmp.path(), &format!("{INNER_HEX}/adapters/inbound"));
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    assert_eq!(r3.len(), 1, "expected 1 error from inner hex only, got: {r3:#?}");
    assert!(r3[0].title.contains("missing") && r3[0].title.contains("inbound"),
        "expected missing inbound error, got: {}", r3[0].title);
    assert!(r3[0].file.as_deref().unwrap_or("").contains("mcp/crates"),
        "expected inner hex file path, got: {:?}", r3[0].file);
    assert!(r3[0].title.contains("mcp/crates"),
        "expected inner hex label_prefix in title, got: {}", r3[0].title);
    // Verify outer apps are clean
    assert!(!r3.iter().any(|e| e.title.contains("devctl")), "devctl should be clean");
    assert!(!r3.iter().any(|e| e.title.contains("worker")), "worker should be clean");
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn loose_cargo_toml_in_structural_dir() {
    // Parity with rule_02's loose_cargo_toml_in_crates
    let tmp = copy_golden();
    for dir in &all_structural_dirs() {
        write_file(tmp.path(), &format!("{dir}/Cargo.toml"), "[package]\nname = \"stray\"");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    let loose: Vec<_> = r3.iter().filter(|e| e.title.contains("loose files")).collect();
    assert_eq!(loose.len(), 8, "expected 8 loose file errors (1 per structural dir), got: {loose:#?}");
    // Verify each error's message mentions Cargo.toml
    for err in &loose {
        assert!(err.message.contains("Cargo.toml"),
            "expected Cargo.toml in message, got: {}", err.message);
    }
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn loose_gitignore_not_gitkeep() {
    // Parity with rule_02's loose_gitignore_not_gitkeep
    let tmp = copy_golden();
    for dir in &all_structural_dirs() {
        write_file(tmp.path(), &format!("{dir}/.gitignore"), "target/");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    let loose: Vec<_> = r3.iter().filter(|e| e.title.contains("loose files")).collect();
    assert_eq!(loose.len(), 8, "expected 8 loose file errors, got: {loose:#?}");
    // .gitignore is NOT .gitkeep — should be flagged
    for err in &loose {
        assert!(err.message.contains(".gitignore"),
            "expected .gitignore in message, got: {}", err.message);
    }
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn dangling_symlink_silently_skipped() {
    // Scenario: dangling symlink in adapters/ — DirEntry::file_type() may return Ok(symlink)
    // or may vary by platform. The symlink should at minimum be caught as a loose file.
    let tmp = copy_golden();
    for dir in &all_structural_dirs() {
        let target = tmp.path().join(format!("{dir}/phantom"));
        std::os::unix::fs::symlink("/nonexistent/target/that/does/not/exist", &target)
            .expect("create dangling symlink");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    // Dangling symlink: file_type() returns Ok(symlink type) on Linux/macOS.
    // is_dir() is false for symlinks, so it should appear in check_loose_files.
    // Expected: 8 loose file errors (1 per dir, mentioning "phantom")
    let loose: Vec<_> = r3.iter().filter(|e| e.title.contains("loose files")).collect();
    assert_eq!(loose.len(), 8, "expected 8 loose file errors for dangling symlinks, got: {loose:#?}");
    for err in &loose {
        assert!(err.message.contains("phantom"),
            "expected 'phantom' in message, got: {}", err.message);
    }
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn outbound_wrong_case_tested() {
    // Scenario hunter gap #6: only Inbound wrong case was tested, not Outbound
    let tmp = copy_golden();
    for dir in &all_structural_dirs() {
        std::fs::create_dir_all(tmp.path().join(format!("{dir}/Outbound"))).expect("mkdir");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    // On case-sensitive FS: Outbound != outbound, flagged as unexpected. 8 errors.
    // On case-insensitive FS: Outbound and outbound merge, 0 unexpected errors.
    if cfg!(target_os = "macos") {
        // Case-insensitive: creating Outbound/ when outbound/ exists is a no-op
        // Check if the FS is case-sensitive by probing
        let probe = tmp.path().join("apps/devctl/crates/adapters/OUTBOUND_PROBE");
        let probe_lower = tmp.path().join("apps/devctl/crates/adapters/outbound_probe");
        std::fs::create_dir_all(&probe).expect("mkdir");
        let is_case_insensitive = probe_lower.exists();
        let _ = std::fs::remove_dir(&probe);
        if is_case_insensitive {
            // Case-insensitive: Outbound/ merges with outbound/, no unexpected error
            // But we should still have 0 rule3 errors
            assert!(r3.is_empty() || r3.iter().all(|e| !e.title.contains("Outbound")),
                "case-insensitive FS should not flag Outbound, got: {r3:#?}");
        } else {
            assert_eq!(r3.len(), 8, "case-sensitive: expected 8 unexpected Outbound errors, got: {r3:#?}");
        }
    } else {
        // Linux: always case-sensitive
        assert_eq!(r3.len(), 8, "expected 8 unexpected Outbound errors, got: {r3:#?}");
        for err in &r3 {
            assert!(err.title.contains("unexpected") && err.title.contains("Outbound"),
                "expected unexpected Outbound, got: {}", err.title);
        }
    }
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}

#[test]
fn gitkeep_plus_partial_required_dirs() {
    // Scenario hunter gap #9: .gitkeep + only inbound/ (no outbound/)
    let tmp = copy_golden();
    for dir in &all_structural_dirs() {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
    }
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    let r3 = rule3_errors(&errors);
    // adapters/ and ports/ still have inbound/ + .gitkeep
    // list_dir returns entries (non-empty), so no early return
    // dir_names = ["inbound"] — outbound missing
    // .gitkeep is a file, not flagged as loose (name == .gitkeep)
    // Expected: 1 missing outbound per dir. But removing outbound/ from backend outer
    // adapters destroys inner hex? No — outbound/ is a sibling of inbound/, removing
    // outbound/ doesn't affect the inbound/mcp path. So all 8 dirs produce errors.
    // Wait: ALL_ADAPTERS_DIRS[3] is inner hex adapters. Removing outbound/ from inner hex
    // adapters is fine. And ALL_PORTS_DIRS[3] is inner hex ports. Removing outbound/ from
    // inner hex ports is fine (ports/outbound/.gitkeep existed there).
    let missing: Vec<_> = r3.iter().filter(|e| e.title.contains("missing") && e.title.contains("outbound")).collect();
    assert_eq!(missing.len(), 8, "expected 8 missing outbound errors, got: {missing:#?}");
    // .gitkeep should NOT be in any loose file error
    let loose: Vec<_> = r3.iter().filter(|e| e.title.contains("loose")).collect();
    assert!(loose.is_empty(), "expected 0 loose file errors (.gitkeep allowed), got: {loose:#?}");
    assert_per_app(&r3);
    assert_inner_hex(&r3);
    assert_file_field(&r3);
    assert_no_ts_apps(&errors);
    assert_no_packages(&errors);
}
