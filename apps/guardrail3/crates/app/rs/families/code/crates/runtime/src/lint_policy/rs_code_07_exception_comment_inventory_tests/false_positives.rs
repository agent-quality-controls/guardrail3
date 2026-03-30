use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_app_rs_family_code_assertions::rs_code_07_exception_comment_inventory::{
    RuleFinding, Severity, assert_findings, assert_no_hits,
};
use test_support::{create_temp_dir, write_file};

#[test]
fn ignores_exception_like_text_outside_supported_config_comment_forms() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let root_package_rel = "package.json";
    let backend_cargo_rel = "apps/backend/Cargo.toml";
    let root_guardrail_rel = "guardrail3.toml";

    let root_package = test_support::read_file(root, root_package_rel);
    let backend_cargo = test_support::read_file(root, backend_cargo_rel);
    let root_guardrail = test_support::read_file(root, root_guardrail_rel);

    write_file(
        root,
        root_package_rel,
        &format!("{root_package}\n// EXCEPTION: package metadata note\n"),
    );
    write_file(
        root,
        backend_cargo_rel,
        &format!("{backend_cargo}\n# exception backend note without required marker\n"),
    );
    write_file(
        root,
        root_guardrail_rel,
        &format!("{root_guardrail}\nnote = \"# EXCEPTION: literal text only\"\n"),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}

#[test]
fn does_not_inventory_repo_root_exception_comments_in_backend_scoped_run() {
    let fixture = create_temp_dir("rs-code-07-scoped-root-config");
    let root = fixture.path();

    write_file(
        root,
        "guardrail3.toml",
        "[profile]\nname = \"service\"\n# EXCEPTION: repo root only\n",
    );
    write_file(
        root,
        "apps/backend/Cargo.toml",
        "[package]\nname = \"backend\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "apps/backend/src/lib.rs", "pub fn ready() {}\n");
    write_file(
        root,
        "apps/backend/rustfmt.toml",
        "max_width = 100 # EXCEPTION: backend scoped only\n",
    );

    let tree = walk_project(&RealFileSystem, root);
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let config = tree.file_content("guardrail3.toml").and_then(|content| {
        toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
    });
    let selected = guardrail3_validation_model::RustFamilySelection::new(BTreeSet::from([
        guardrail3_validation_model::RustValidateFamily::Code,
    ]));
    let route = guardrail3_app_rs_family_mapper::FamilyMapper::new(
        &tree,
        &scope,
        config.as_ref(),
        &selected,
        None,
    )
    .with_validation_scope(Some("apps/backend/src"))
    .map_rs_code();

    let results = crate::check(&tree, &route);
    assert_findings(
        &results,
        &[RuleFinding::new(
            Severity::Info,
            "EXCEPTION comment inventory",
            "Config exception comment: # EXCEPTION: backend scoped only",
            Some("apps/backend/rustfmt.toml"),
            Some(1),
            true,
        )],
    );
}
