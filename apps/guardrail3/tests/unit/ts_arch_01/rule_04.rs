use super::helpers::{
    arch_errors, assert_file_field, assert_no_landing, assert_no_packages, assert_no_rust_apps,
    copy_fixture, run_check, write_file,
};

// ============================================================================
// Rule 04: Loose files in structural/container dirs (only .gitkeep allowed)
// ============================================================================

/// Container dirs: domain, application, adapters/{in,out}, ports/{in,out}
const CONTAINER_SUFFIXES: &[&str] = &[
    "application",
    "domain",
    "adapters/inbound",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

fn all_container_paths() -> Vec<String> {
    CONTAINER_SUFFIXES
        .iter()
        .map(|s| format!("apps/admin/src/modules/{s}"))
        .collect()
}

/// Filter to only loose-file errors.
fn loose_file_errors<'a>(errors: &'a [&guardrail3::domain::report::CheckResult]) -> Vec<&'a &'a guardrail3::domain::report::CheckResult> {
    errors
        .iter()
        .filter(|e| e.title.contains("loose files"))
        .collect()
}

// ============================================================================
// GROUP A: Loose files in each container type
// ============================================================================

#[test]
fn loose_file_in_domain() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/domain/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_file_in_application() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/application/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_file_in_adapters_inbound() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/adapters/inbound/stray.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_file_in_all_containers() {
    let tmp = copy_fixture();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 6 loose-file errors (1 per container), got {}: {loose:#?}",
        loose.len()
    );
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
    assert_file_field(&errors);
}

// ============================================================================
// GROUP B: Various file types
// ============================================================================

#[test]
fn loose_tsx_file() {
    let tmp = copy_fixture();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.tsx"), "export default function() {}");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 6 errors for .tsx files, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_json_file() {
    let tmp = copy_fixture();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/config.json"), "{}");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 6 errors for .json files, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_env_file() {
    let tmp = copy_fixture();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.env"), "SECRET=123");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 6 errors for .env files, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_gitignore_not_gitkeep() {
    let tmp = copy_fixture();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitignore"), "node_modules/");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        ".gitignore is NOT .gitkeep — expected 6 errors, got {}: {loose:#?}",
        loose.len()
    );
}

// ============================================================================
// GROUP C: .gitkeep behavior
// ============================================================================

#[test]
fn gitkeep_allowed_in_containers() {
    let tmp = copy_fixture();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitkeep"), "");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert!(
        loose.is_empty(),
        "expected 0 loose-file errors when only .gitkeep is present, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn gitkeep_alongside_loose_file() {
    let tmp = copy_fixture();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        ".gitkeep is allowed but stray.ts is not — expected 6 errors, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn multiple_loose_files_single_error_per_dir() {
    let tmp = copy_fixture();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/index.ts"), "// stray 1");
        write_file(tmp.path(), &format!("{path}/types.ts"), "// stray 2");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        "expected 1 error per dir (6 total), not 1 per file, got {}: {loose:#?}",
        loose.len()
    );
    for err in &loose {
        assert!(
            err.message.contains("index.ts") && err.message.contains("types.ts"),
            "expected both files listed in message, got: '{}'",
            err.message
        );
    }
}

// ============================================================================
// GROUP D: Loose files in modules/ root and structural dirs
// ============================================================================

#[test]
fn loose_file_in_modules_root() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/index.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error in modules/ root, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_file_in_adapters_root() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/adapters/index.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error in adapters/ root, got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn loose_file_in_ports_root() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/src/modules/ports/index.ts", "// stray");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        1,
        "expected 1 loose-file error in ports/ root, got {}: {loose:#?}",
        loose.len()
    );
}

// ============================================================================
// GROUP E: Cross-cutting
// ============================================================================

#[test]
fn loose_files_across_all_dir_types() {
    let tmp = copy_fixture();
    // modules/ root
    write_file(tmp.path(), "apps/admin/src/modules/stray.ts", "// stray");
    // structural dirs (adapters/, ports/)
    write_file(tmp.path(), "apps/admin/src/modules/adapters/stray.ts", "// stray");
    write_file(tmp.path(), "apps/admin/src/modules/ports/stray.ts", "// stray");
    // all 6 containers
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    // 1 (modules root) + 2 (structural) + 6 (containers) = 9
    assert_eq!(
        loose.len(),
        9,
        "expected 9 total loose-file errors (1 + 2 + 6), got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn per_app_attribution() {
    let tmp = copy_fixture();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/stray.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    for err in &loose {
        assert!(
            err.title.contains("admin"),
            "expected 'admin' in error title, got: '{}'",
            err.title
        );
    }
}

#[test]
fn hidden_file_not_gitkeep() {
    let tmp = copy_fixture();
    for path in all_container_paths() {
        write_file(tmp.path(), &format!("{path}/.hidden"), "secret");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose = loose_file_errors(&errors);
    assert_eq!(
        loose.len(),
        6,
        ".hidden is not .gitkeep — expected 6 errors, got {}: {loose:#?}",
        loose.len()
    );
}
