use g3rs_arch_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results};
use guardrail3_check_types::G3Severity;

use crate::test_support::{input, module_dir};

#[test]
fn sibling_file_without_mod_rs_uses_forbidden_convention() {
    let mut module = module_dir("crate_a/src/nested");
    module.mod_decl_file = "crate_a/src/lib.rs".to_owned();
    module.mod_decl_line = 3;
    module.has_sibling_file = true;

    let results = crate::check(&input(Vec::new(), vec![module]));

    assert_rule_results(
        &results,
        "RS-ARCH-FILETREE-03",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("module directory uses foo.rs convention"),
            file: Some("crate_a/src/lib.rs"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn declared_module_directory_without_mod_rs_fires() {
    let mut module = module_dir("crate_a/src/nested");
    module.mod_decl_file = "crate_a/src/lib.rs".to_owned();
    module.mod_decl_line = 7;

    let results = crate::check(&input(Vec::new(), vec![module]));

    assert_rule_results(
        &results,
        "RS-ARCH-FILETREE-03",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("module directory missing mod.rs"),
            file: Some("crate_a/src/lib.rs"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn path_wired_directory_without_mod_declaration_fires_on_directory() {
    let module = module_dir("crate_a/src/generated");

    let results = crate::check(&input(Vec::new(), vec![module]));

    assert_rule_results(
        &results,
        "RS-ARCH-FILETREE-03",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("module directory missing mod.rs"),
            file: Some("crate_a/src/generated"),
            inventory: Some(false),
            message: Some(
                "Directory `crate_a/src/generated` contains 1 .rs files but has no mod.rs. Create `crate_a/src/generated/mod.rs` with `mod` declarations for each .rs file in the directory.",
            ),
        }],
    );
}

#[test]
fn module_directory_with_mod_rs_inventories() {
    let mut module = module_dir("crate_a/src/nested");
    module.has_mod_rs = true;

    let results = crate::check(&input(Vec::new(), vec![module]));

    assert_rule_results(
        &results,
        "RS-ARCH-FILETREE-03",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Info),
            title: Some("module directory has mod.rs"),
            file: Some("crate_a/src/nested/mod.rs"),
            inventory: Some(true),
            message: None,
        }],
    );
}
