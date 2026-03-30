use std::collections::BTreeSet;

use super::super::{collect, family_route};
use crate::rs_garde_02_core_method_bans;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn scoped_files_restrict_source_fact_collection() {
    let root = temp_root("rs-garde-facts-scoped-sources");
    let in_scope_rel = "src/in_scope.rs";
    let out_of_scope_rel = "src/out_of_scope.rs";
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
            ),
            ("src", dir_entry(&[], &["in_scope.rs", "out_of_scope.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"[workspace]
members = []
[workspace.dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
            (
                "clippy.toml",
                &rs_garde_02_core_method_bans::canonical_clippy_toml(),
            ),
            (in_scope_rel, source_fixture("InScope")),
            (out_of_scope_rel, source_fixture("OutOfScope")),
        ],
        root.clone(),
    );

    let scoped_files = BTreeSet::from([in_scope_rel.to_owned()]);
    let route = family_route(&tree, Some(&scoped_files));
    let facts = collect(&tree, &route);

    assert!(
        !facts.struct_targets.is_empty()
            && facts
                .struct_targets
                .iter()
                .all(|target| target.rel_path == in_scope_rel),
        "unexpected out-of-scope struct targets: {:#?}",
        facts.struct_targets
    );
    assert_eq!(
        facts
            .manual_deserialize_impls
            .iter()
            .map(|target| target.rel_path.as_str())
            .collect::<Vec<_>>(),
        vec![in_scope_rel]
    );
    assert_eq!(
        facts
            .enum_targets
            .iter()
            .map(|target| target.rel_path.as_str())
            .collect::<Vec<_>>(),
        vec![in_scope_rel]
    );
    assert_eq!(
        facts
            .query_as_macros
            .iter()
            .map(|target| target.rel_path.as_str())
            .collect::<Vec<_>>(),
        vec![in_scope_rel]
    );
    assert!(
        facts
            .boundary_fields
            .iter()
            .all(|field| field.rel_path == in_scope_rel),
        "unexpected out-of-scope boundary field facts: {:#?}",
        facts.boundary_fields
    );
    assert_eq!(
        facts
            .guardrail_config_validation_sites
            .iter()
            .map(|target| target.rel_path.as_str())
            .collect::<Vec<_>>(),
        vec![in_scope_rel]
    );

    std::fs::remove_dir_all(root).expect("remove temporary garde facts root");
}

#[test]
fn scoped_files_skip_out_of_scope_parse_failures() {
    let root = temp_root("rs-garde-facts-scoped-failures");
    let in_scope_rel = "src/in_scope.rs";
    let out_of_scope_rel = "src/out_of_scope.rs";
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
            ),
            ("src", dir_entry(&[], &["in_scope.rs", "out_of_scope.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"[workspace]
members = []
[workspace.dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
            (
                "clippy.toml",
                &rs_garde_02_core_method_bans::canonical_clippy_toml(),
            ),
            (in_scope_rel, "pub struct Healthy;\n"),
            (out_of_scope_rel, "fn broken(\n"),
        ],
        root.clone(),
    );

    let scoped_files = BTreeSet::from([in_scope_rel.to_owned()]);
    let route = family_route(&tree, Some(&scoped_files));
    let facts = collect(&tree, &route);

    assert!(
        facts.input_failures.is_empty(),
        "unexpected out-of-scope input failures: {:#?}",
        facts.input_failures
    );

    std::fs::remove_dir_all(root).expect("remove temporary garde facts root");
}

fn source_fixture(prefix: &str) -> &str {
    match prefix {
        "InScope" => fixture_body("InScope"),
        "OutOfScope" => fixture_body("OutOfScope"),
        _ => panic!("unexpected fixture prefix"),
    }
}

fn fixture_body(prefix: &str) -> &'static str {
    match prefix {
        "InScope" => {
            r#"
use garde::Validate;
use serde::Deserialize;

struct InScopeLeaf {
    value: String,
}

#[derive(Validate)]
struct InScopeNested {
    leaf: InScopeLeaf,
}

#[derive(Deserialize)]
struct InScopeStructBoundary {
    nested: InScopeNested,
}

struct InScopeManualBoundary {
    nested: InScopeNested,
}

impl<'de> Deserialize<'de> for InScopeManualBoundary {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

#[derive(Deserialize)]
enum InScopeEnumBoundary {
    Named { nested: InScopeNested },
}

#[derive(Deserialize, Validate)]
struct InScopeValidatedBoundary {
    notes: Vec<String>,
    nested: InScopeNested,
    #[garde(custom(|value, ctx| validate_name(value, ctx)))]
    name: String,
}

fn validate_name(_value: &str, _ctx: &()) -> garde::Result {
    Ok(())
}

struct InScopeRow;

fn inventory() {
    let _ = sqlx::query_as!(InScopeRow, "select 1");
}

fn load_config(content: &str) -> Option<guardrail3_domain_config::types::GuardrailConfig> {
    toml::from_str(content).ok()
}
"#
        }
        "OutOfScope" => {
            r#"
use garde::Validate;
use serde::Deserialize;

struct OutOfScopeLeaf {
    value: String,
}

#[derive(Validate)]
struct OutOfScopeNested {
    leaf: OutOfScopeLeaf,
}

#[derive(Deserialize)]
struct OutOfScopeStructBoundary {
    nested: OutOfScopeNested,
}

struct OutOfScopeManualBoundary {
    nested: OutOfScopeNested,
}

impl<'de> Deserialize<'de> for OutOfScopeManualBoundary {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

#[derive(Deserialize)]
enum OutOfScopeEnumBoundary {
    Named { nested: OutOfScopeNested },
}

#[derive(Deserialize, Validate)]
struct OutOfScopeValidatedBoundary {
    notes: Vec<String>,
    nested: OutOfScopeNested,
    #[garde(custom(|value, ctx| validate_name(value, ctx)))]
    name: String,
}

fn validate_name(_value: &str, _ctx: &()) -> garde::Result {
    Ok(())
}

struct OutOfScopeRow;

fn inventory() {
    let _ = sqlx::query_as!(OutOfScopeRow, "select 1");
}

fn load_config(content: &str) -> Option<guardrail3_domain_config::types::GuardrailConfig> {
    toml::from_str(content).ok()
}
"#
        }
        _ => panic!("unexpected fixture body"),
    }
}
