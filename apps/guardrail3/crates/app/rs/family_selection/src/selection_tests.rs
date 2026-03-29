use std::collections::BTreeMap;
use std::path::PathBuf;

use guardrail3_domain_config::types::{GuardrailConfig, RustChecksConfig, RustConfig};
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
use guardrail3_validation_model::RustValidateFamily;

use crate::selection::resolve_for_tests;

#[test]
fn explicit_family_request_bypasses_disabled_config_filter() {
    let tree = minimal_tree();
    let config = GuardrailConfig {
        version: None,
        profile: None,
        rust: Some(RustConfig {
            workspace_root: None,
            workspaces: None,
            apps: None,
            packages: None,
            checks: Some(RustChecksConfig {
                arch: Some(false),
                fmt: None,
                toolchain: None,
                clippy: None,
                deny: None,
                cargo: None,
                code: None,
                hexarch: Some(true),
                libarch: Some(true),
                deps: None,
                garde: None,
                test: None,
                release: None,
                hooks_shared: None,
                hooks_rs: None,
            }),
        }),
        typescript: None,
        hooks: None,
    };

    let selection = resolve_for_tests(&tree, Some(&config), &[RustValidateFamily::Arch]);

    assert!(
        selection.contains(RustValidateFamily::Arch),
        "explicitly requested family should survive disabled config"
    );
}

#[test]
fn empty_request_uses_enabled_family_filtering() {
    let tree = minimal_tree();
    let config = GuardrailConfig {
        version: None,
        profile: None,
        rust: Some(RustConfig {
            workspace_root: None,
            workspaces: None,
            apps: None,
            packages: None,
            checks: Some(RustChecksConfig {
                arch: Some(false),
                fmt: Some(true),
                toolchain: None,
                clippy: None,
                deny: None,
                cargo: None,
                code: None,
                hexarch: Some(false),
                libarch: Some(false),
                deps: None,
                garde: None,
                test: None,
                release: None,
                hooks_shared: None,
                hooks_rs: None,
            }),
        }),
        typescript: None,
        hooks: None,
    };

    let selection = resolve_for_tests(&tree, Some(&config), &[]);

    assert!(
        !selection.contains(RustValidateFamily::Arch),
        "unrequested disabled family should stay filtered"
    );
    assert!(
        selection.contains(RustValidateFamily::Fmt),
        "enabled family should still be selected"
    );
}

fn minimal_tree() -> ProjectTree {
    let mut structure = BTreeMap::new();
    let _ = structure.insert(
        String::new(),
        DirEntry {
            dirs: vec![],
            files: vec![],
            symlink_dirs: vec![],
            symlink_files: vec![],
        },
    );

    ProjectTree {
        root: PathBuf::from("/tmp/guardrail3-family-selection-tests"),
        structure,
        content: BTreeMap::new(),
    }
}
