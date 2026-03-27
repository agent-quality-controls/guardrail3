use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_documented_and_cross_rule_near_misses() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let documented_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let nested_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";
    let mixed_rel = "apps/devctl/crates/ports/outbound/traits/src/lib.rs";
    let cfg_attr_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let documented_mod_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";

    let documented_content =
        std::fs::read_to_string(root.join(documented_rel)).expect("read documented file");
    let nested_content = std::fs::read_to_string(root.join(nested_rel)).expect("read nested file");
    let mixed_content = std::fs::read_to_string(root.join(mixed_rel)).expect("read mixed file");
    let cfg_attr_content =
        std::fs::read_to_string(root.join(cfg_attr_rel)).expect("read cfg_attr file");
    let documented_mod_content =
        std::fs::read_to_string(root.join(documented_mod_rel)).expect("read documented mod file");

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
            "{nested_content}\nmod nested_documented {{\n    #[allow(clippy::panic)] // REASON: temporary test seam\n    pub fn helper() {{}}\n}}\n"
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
            "{documented_mod_content}\n#[allow(clippy::unwrap_used, clippy::expect_used)] //reason: adapter boundary shim\npub mod documented_module_probe {{\n    pub fn helper() {{}}\n}}\n"
        ),
    );

    let results = run_family(root);
    let mixed_new = format!(
        "{mixed_content}\n#[allow(clippy::unwrap_used)] // reason: trait adapter boundary\npub fn documented_probe() {{}}\n#[allow(clippy::expect_used)]\npub fn undocumented_probe() {{}}\n"
    );
    let undocumented_line = mixed_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::expect_used)]"))
        .expect("mixed undocumented allow line")
        + 1;
    let rs_code_03_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-03")
        .map(|result| {
            (
                result.file.clone().expect("file"),
                result.line,
                format!("{:?}", result.severity),
                result.title.clone(),
                result.message.clone(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-03"),
        BTreeSet::from([mixed_rel.to_owned()])
    );
    assert_eq!(
        rs_code_03_results,
        vec![(
            mixed_rel.to_owned(),
            Some(undocumented_line),
            format!("{:?}", Severity::Error),
            "item-level allow without reason".to_owned(),
            "`#[allow(clippy::expect_used)]` requires `// reason:` on the same line.".to_owned(),
        )]
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

    let top_level_content =
        std::fs::read_to_string(root.join(top_level_rel)).expect("read top-level documented file");
    let nested_content =
        std::fs::read_to_string(root.join(nested_rel)).expect("read nested documented file");
    let grouped_content =
        std::fs::read_to_string(root.join(grouped_rel)).expect("read grouped documented file");
    let impl_content = std::fs::read_to_string(root.join(impl_rel)).expect("read impl file");
    let trait_content = std::fs::read_to_string(root.join(trait_rel)).expect("read trait file");

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
            "{nested_content}\nmod nested_documented {{\n    #[allow(clippy::panic)] //reason: outbound port seam\n    pub fn helper() {{}}\n}}\n"
        ),
    );
    write_file(
        root,
        grouped_rel,
        &format!(
            "{grouped_content}\n#[allow(clippy::unwrap_used, clippy::expect_used)] // REASON: trait adapter compatibility\npub mod documented_grouped_probe {{\n    pub fn helper() {{}}\n}}\n"
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
    let rs_code_03_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-03")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-03"), BTreeSet::new());
    assert!(rs_code_03_results.is_empty());
}

#[test]
fn skips_non_item_allow_surfaces() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read non-item surface file");
    let new_content = format!(
        "{content}\npub fn non_item_probe(input: Option<&str>) -> usize {{\n    #[allow(clippy::unwrap_used)]\n    let value = input.unwrap_or(\"fallback\");\n    match value.len() {{\n        #[allow(clippy::wildcard_enum_match_arm)]\n        _ => value.len(),\n    }}\n}}\n"
    );
    write_file(root, rel, &new_content);

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-03"), BTreeSet::new());
}

#[test]
fn skips_documented_supported_non_function_item_kinds() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/devctl/crates/ports/outbound/traits/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read supported item kinds file");
    let new_content = format!(
        "{content}\n#[allow(clippy::module_name_repetitions)] // reason: exported config constant\npub const DOCUMENTED_CONST_PROBE: usize = 1;\n#[allow(clippy::module_name_repetitions)] // reason: cached process state\npub static DOCUMENTED_STATIC_PROBE: &str = \"ready\";\n#[allow(clippy::module_name_repetitions)] // reason: compatibility alias\npub type DocumentedAliasProbe = usize;\n#[allow(clippy::module_name_repetitions)] // reason: trait alias bridge\npub trait DocumentedTraitAliasProbe = Send + Sync;\n#[allow(unused_imports)] // reason: adapter import seam\nuse std::fmt as _;\n#[allow(clippy::module_name_repetitions)] // reason: ffi compatibility shape\npub union DocumentedUnionProbe {{\n    bytes: [u8; 4],\n    word: u32,\n}}\n#[allow(clippy::module_name_repetitions)] // reason: DTO naming compatibility\npub struct DocumentedStructProbe;\n#[allow(clippy::module_name_repetitions)] // reason: state machine naming compatibility\npub enum DocumentedEnumProbe {{\n    Ready,\n}}\ntrait AssocBoundary {{\n    #[allow(clippy::module_name_repetitions)] // reason: trait-associated constant contract\n    const LIMIT: usize;\n    #[allow(clippy::module_name_repetitions)] // reason: trait-associated type contract\n    type Output;\n}}\nstruct DocumentedAssocProbe;\nimpl AssocBoundary for DocumentedAssocProbe {{\n    #[allow(clippy::module_name_repetitions)] // reason: trait-associated constant bridge\n    const LIMIT: usize = 1;\n    #[allow(clippy::module_name_repetitions)] // reason: trait-associated type bridge\n    type Output = usize;\n}}\n"
    );
    write_file(root, rel, &new_content);

    let results = run_family(root);
    let rs_code_03_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-03")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-03"), BTreeSet::new());
    assert!(rs_code_03_results.is_empty());
}
