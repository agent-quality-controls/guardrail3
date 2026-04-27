use g3rs_arch_file_tree_checks_assertions::mod_rs_required as assertions;

use super::helpers::{module_dir, run_rule};

#[test]
fn sibling_file_without_mod_rs_uses_forbidden_convention() {
    let mut module = module_dir("crate_a/src/nested");
    module.mod_decl_file = "crate_a/src/lib.rs".to_owned();
    module.mod_decl_line = 3;
    module.has_sibling_file = true;

    let results = run_rule(&module);

    assertions::assert_foo_rs_convention_error(&results, "crate_a/src/lib.rs");
}

#[test]
fn declared_module_directory_without_mod_rs_fires() {
    let mut module = module_dir("crate_a/src/nested");
    module.mod_decl_file = "crate_a/src/lib.rs".to_owned();
    module.mod_decl_line = 7;

    let results = run_rule(&module);

    assertions::assert_missing_mod_rs_error(&results, "crate_a/src/lib.rs");
}

#[test]
fn path_wired_directory_without_mod_declaration_fires_on_directory() {
    let results = run_rule(&module_dir("crate_a/src/generated"));

    assertions::assert_missing_mod_rs_error(&results, "crate_a/src/generated");
}

#[test]
fn module_directory_with_mod_rs_inventories() {
    let mut module = module_dir("crate_a/src/nested");
    module.has_mod_rs = true;

    let results = run_rule(&module);

    assertions::assert_mod_rs_inventory(&results, "crate_a/src/nested/mod.rs");
}
