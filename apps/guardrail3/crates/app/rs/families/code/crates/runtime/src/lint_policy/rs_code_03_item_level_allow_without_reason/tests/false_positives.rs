use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_03_item_level_allow_without_reason::{
    RuleFinding, Severity, assert_files, assert_findings, assert_no_hits,
};
use test_support::write_file;

#[test]
fn skips_documented_and_cross_rule_near_misses() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let documented_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let nested_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";
    let mixed_rel = "apps/devctl/crates/ports/outbound/traits/src/lib.rs";
    let cfg_attr_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let documented_mod_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";

    let documented_content = test_support::read_file(root, documented_rel);
    let nested_content = test_support::read_file(root, nested_rel);
    let mixed_content = test_support::read_file(root, mixed_rel);
    let cfg_attr_content = test_support::read_file(root, cfg_attr_rel);
    let documented_mod_content = test_support::read_file(root, documented_mod_rel);

    write_file(
        root,
        documented_rel,
        &format!(
            "{documented_content}\n#[allow(clippy::unwrap_used)] // reason: compatibility shim\npub fn documented_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        nested_rel,
        &format!(
            "{nested_content}\nmod nested_documented {{\n    #[allow(clippy::panic)] // reason: temporary test seam\n    pub fn helper() {{}}\n}}\n"
        ),
    );
    write_file(
        root,
        mixed_rel,
        &format!(
            "{mixed_content}\n#[allow(clippy::unwrap_used)] // reason: trait adapter boundary\npub fn documented_probe() {{}}\n#[allow(clippy::expect_used)]\npub fn undocumented_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        cfg_attr_rel,
        &format!(
            "{cfg_attr_content}\n#[cfg_attr(feature = \"probe\", allow(clippy::unwrap_used))]\npub fn conditional_cfg_attr_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        documented_mod_rel,
        &format!(
            "{documented_mod_content}\n#[allow(clippy::unwrap_used, clippy::expect_used)] // reason: adapter boundary shim\npub mod documented_module_probe {{\n    pub fn helper() {{}}\n}}\n"
        ),
    );

    let results = run_family(root);
    let mixed_new = format!(
        "{mixed_content}\n#[allow(clippy::unwrap_used)] // reason: trait adapter boundary\npub fn documented_probe() {{}}\n#[allow(clippy::expect_used)]\npub fn undocumented_probe() {{}}\n"
    );
    let undocumented_line = mixed_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::expect_used)]"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_files(&results, BTreeSet::from([mixed_rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding::new(
            Severity::Error,
            "item-level allow without reason",
            "`#[allow(clippy::expect_used)]` requires `// reason:` on the same line.",
            Some(mixed_rel),
            Some(undocumented_line),
            false,
        )],
    );
}

#[test]
fn skips_broad_documented_item_level_allows_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let top_level_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let nested_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";
    let grouped_rel = "apps/devctl/crates/ports/outbound/traits/src/lib.rs";
    let impl_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let trait_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";

    let top_level_content = test_support::read_file(root, top_level_rel);
    let nested_content = test_support::read_file(root, nested_rel);
    let grouped_content = test_support::read_file(root, grouped_rel);
    let impl_content = test_support::read_file(root, impl_rel);
    let trait_content = test_support::read_file(root, trait_rel);

    write_file(
        root,
        top_level_rel,
        &format!(
            "{top_level_content}\n#[allow(clippy::unwrap_used)] // reason: compatibility shim\npub fn documented_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        nested_rel,
        &format!(
            "{nested_content}\nmod nested_documented {{\n    #[allow(clippy::panic)] // reason: outbound port seam\n    pub fn helper() {{}}\n}}\n"
        ),
    );
    write_file(
        root,
        grouped_rel,
        &format!(
            "{grouped_content}\n#[allow(clippy::unwrap_used, clippy::expect_used)] // reason: trait adapter compatibility\npub mod documented_grouped_probe {{\n    pub fn helper() {{}}\n}}\n"
        ),
    );
    write_file(
        root,
        impl_rel,
        &format!(
            "{impl_content}\nstruct ImplBoundary;\nimpl ImplBoundary {{\n    #[allow(clippy::panic)] // reason: outbound adapter glue\n    fn documented_impl_probe(&self) {{}}\n}}\n"
        ),
    );
    write_file(
        root,
        trait_rel,
        &format!(
            "{trait_content}\npub trait DocumentedTraitBoundary {{\n    #[allow(clippy::unwrap_used)] // reason: trait shim contract\n    fn documented_trait_probe(&self);\n}}\n"
        ),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}

#[test]
fn skips_non_item_allow_surfaces() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let new_content = format!(
        "{content}\npub fn non_item_probe(input: Option<&str>) -> usize {{\n    #[allow(clippy::unwrap_used)]\n    let value = input.unwrap_or(\"fallback\");\n    match value.len() {{\n        #[allow(clippy::wildcard_enum_match_arm)]\n        _ => value.len(),\n    }}\n}}\n"
    );
    write_file(root, rel, &new_content);

    let results = run_family(root);

    assert_no_hits(&results);
}

#[test]
fn skips_documented_supported_non_function_item_kinds() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/devctl/crates/ports/outbound/traits/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let new_content = format!(
        "{content}\n#[allow(clippy::module_name_repetitions)] // reason: exported config constant\npub const DOCUMENTED_CONST_PROBE: usize = 1;\n#[allow(clippy::module_name_repetitions)] // reason: cached process state\npub static DOCUMENTED_STATIC_PROBE: &str = \"ready\";\n#[allow(clippy::module_name_repetitions)] // reason: compatibility alias\npub type DocumentedAliasProbe = usize;\n#[allow(clippy::module_name_repetitions)] // reason: trait alias bridge\npub trait DocumentedTraitAliasProbe = Send + Sync;\n#[allow(unused_imports)] // reason: adapter import seam\nuse std::fmt as _;\n#[allow(clippy::module_name_repetitions)] // reason: ffi compatibility shape\npub union DocumentedUnionProbe {{\n    bytes: [u8; 4],\n    word: u32,\n}}\n#[allow(clippy::module_name_repetitions)] // reason: DTO naming compatibility\npub struct DocumentedStructProbe;\n#[allow(clippy::module_name_repetitions)] // reason: state machine naming compatibility\npub enum DocumentedEnumProbe {{\n    Ready,\n}}\ntrait AssocBoundary {{\n    #[allow(clippy::module_name_repetitions)] // reason: trait-associated constant contract\n    const LIMIT: usize;\n    #[allow(clippy::module_name_repetitions)] // reason: trait-associated type contract\n    type Output;\n}}\nstruct DocumentedAssocProbe;\nimpl AssocBoundary for DocumentedAssocProbe {{\n    #[allow(clippy::module_name_repetitions)] // reason: trait-associated constant bridge\n    const LIMIT: usize = 1;\n    #[allow(clippy::module_name_repetitions)] // reason: trait-associated type bridge\n    type Output = usize;\n}}\n"
    );
    write_file(root, rel, &new_content);

    let results = run_family(root);
    assert_no_hits(&results);
}

#[test]
fn skips_multiline_documented_allow_with_reason_on_closing_line() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let new_content = format!(
        "{content}\n#[allow(\n    clippy::unwrap_used,\n    clippy::expect_used\n)] // reason: multiline adapter seam\npub fn documented_probe() {{}}\n"
    );
    write_file(root, rel, &new_content);

    let results = run_family(root);
    assert_no_hits(&results);
}

#[test]
fn rejects_weak_same_line_reasons() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let new_content = format!(
        "{content}\n#[allow(clippy::unwrap_used)] // reason: temp\npub fn weak_reason_probe() {{}}\n"
    );
    write_file(root, rel, &new_content);

    let line = new_content
        .lines()
        .position(|source| source.contains("#[allow(clippy::unwrap_used)] // reason: temp"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_findings(
        &run_family(root),
        &[RuleFinding::new(
            Severity::Error,
            "item-level allow reason too weak",
            "`#[allow(clippy::unwrap_used)]` reason must be specific and at least two words. Weak reason `temp` found.",
            Some(rel),
            Some(line),
            false,
        )],
    );
}
