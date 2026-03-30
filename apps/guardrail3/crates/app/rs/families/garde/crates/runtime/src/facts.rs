mod cargo_roots;
mod clippy;
mod policy;
mod validation;

use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsGardeRoute;
use guardrail3_domain_project_tree::ProjectTree;

use super::discover::{is_test_path, rust_file_rels};
use super::parse::{BoundaryKind, analyze, parse_rust_file};
use self::cargo_roots::collect_cargo_roots;
use self::clippy::{collect_clippy_configs, owning_root_dir, push_root_facts};
use self::policy::read_policy_map;
use self::validation::resolve_validation_state;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyRootKind {
    WorkspaceRoot,
    StandalonePackageRoot,
}

impl PolicyRootKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace root",
            Self::StandalonePackageRoot => "standalone package root",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GardeRootFacts {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub kind: PolicyRootKind,
    pub garde_dependency_present: bool,
    pub clippy_rel_path: Option<String>,
    pub clippy_parsed: Option<toml::Value>,
    pub clippy_parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DerivedBoundaryTypeFacts {
    pub rel_path: String,
    pub line: usize,
    pub name: String,
    pub boundary_kind: BoundaryKind,
    pub boundary_macros: Vec<String>,
    pub has_validate: bool,
}

#[derive(Debug, Clone)]
pub struct ManualDeserializeImplFacts {
    pub rel_path: String,
    pub line: usize,
    pub type_name: String,
    pub needs_validate: bool,
    pub has_validate: bool,
}

#[derive(Debug, Clone)]
pub struct QueryAsMacroFacts {
    pub rel_path: String,
    pub line: usize,
    pub macro_name: String,
}

#[derive(Debug, Clone)]
pub struct BoundaryFieldFacts {
    pub rel_path: String,
    pub line: usize,
    pub boundary_name: String,
    pub field_name: String,
    pub field_type: String,
    pub requires_field_validation: bool,
    pub nested_validated: bool,
    pub has_garde_skip: bool,
    pub has_garde_dive: bool,
    pub has_meaningful_garde_rule: bool,
    pub uses_context: bool,
    pub boundary_has_context: bool,
}

#[derive(Debug, Clone)]
pub struct GardeInputFailureFacts {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct GardeFacts {
    pub roots: Vec<GardeRootFacts>,
    pub struct_targets: Vec<DerivedBoundaryTypeFacts>,
    pub enum_targets: Vec<DerivedBoundaryTypeFacts>,
    pub manual_deserialize_impls: Vec<ManualDeserializeImplFacts>,
    pub boundary_fields: Vec<BoundaryFieldFacts>,
    pub query_as_macros: Vec<QueryAsMacroFacts>,
    pub input_failures: Vec<GardeInputFailureFacts>,
}

#[derive(Debug, Clone)]
struct CargoRootFacts {
    rel_dir: String,
    has_workspace: bool,
    has_package: bool,
    workspace_members: Vec<String>,
}

#[derive(Debug, Clone)]
struct PolicySettings {
    garde_enabled: bool,
}

#[derive(Debug, Clone)]
struct ClippyConfigCandidate {
    rel_dir: String,
    rel_path: String,
    parsed: Option<toml::Value>,
    parse_error: Option<String>,
}

pub fn collect(tree: &ProjectTree, route: &RsGardeRoute) -> GardeFacts {
    let mut input_failures = Vec::new();
    let cargo_roots = collect_cargo_roots(tree, route, &mut input_failures);
    let workspace_roots: BTreeSet<_> = cargo_roots
        .values()
        .filter(|facts| facts.has_workspace)
        .map(|facts| facts.rel_dir.clone())
        .collect();
    let workspace_members: BTreeSet<_> = cargo_roots
        .values()
        .flat_map(|facts| facts.workspace_members.iter().cloned())
        .collect();
    let standalone_package_roots: BTreeSet<_> = cargo_roots
        .values()
        .filter(|facts| facts.has_package && !workspace_members.contains(&facts.rel_dir))
        .map(|facts| facts.rel_dir.clone())
        .collect();

    let policy_map = read_policy_map(
        tree,
        &cargo_roots,
        &standalone_package_roots,
        &mut input_failures,
    );
    let clippy_configs = collect_clippy_configs(
        tree,
        &workspace_roots,
        &standalone_package_roots,
        &mut input_failures,
    );

    let mut roots = Vec::new();
    for rel_dir in workspace_roots {
        push_root_facts(
            tree,
            &rel_dir,
            PolicyRootKind::WorkspaceRoot,
            &policy_map,
            &clippy_configs,
            &mut roots,
        );
    }
    for rel_dir in standalone_package_roots {
        push_root_facts(
            tree,
            &rel_dir,
            PolicyRootKind::StandalonePackageRoot,
            &policy_map,
            &clippy_configs,
            &mut roots,
        );
    }

    let active_root_dirs: Vec<_> = roots
        .iter()
        .filter(|root| root.garde_dependency_present)
        .map(|root| root.rel_dir.clone())
        .collect();

    let mut struct_targets = Vec::new();
    let mut enum_targets = Vec::new();
    let mut manual_deserialize_impls = Vec::new();
    let mut boundary_fields = Vec::new();
    let mut query_as_macros = Vec::new();
    let mut global_type_validation_map = BTreeMap::<String, (bool, bool)>::new();
    let mut simple_type_validation_map = BTreeMap::<String, Vec<(bool, bool)>>::new();
    let mut parsed_files = Vec::new();

    for rel_path in rust_file_rels(tree) {
        if is_test_path(&rel_path) {
            continue;
        }
        let Some(_root_rel_dir) = owning_root_dir(&rel_path, &active_root_dirs) else {
            continue;
        };
        let abs_path = tree.abs_path(&rel_path);
        let content = match guardrail3_shared_fs::read_file_err(&abs_path) {
            Ok(content) => content,
            Err(read_error) => {
                input_failures.push(GardeInputFailureFacts {
                    rel_path: rel_path.clone(),
                    message: format!(
                        "Failed to read Rust source file for garde checks: {read_error}"
                    ),
                });
                continue;
            }
        };
        let ast = match parse_rust_file(&content) {
            Ok(ast) => ast,
            Err(parse_error) => {
                input_failures.push(GardeInputFailureFacts {
                    rel_path: rel_path.clone(),
                    message: format!(
                        "Failed to parse Rust source file for garde checks: {parse_error}"
                    ),
                });
                continue;
            }
        };
        let parsed = analyze(&ast);
        for (type_name, state) in &parsed.type_validation_map {
            let _ = global_type_validation_map.insert(type_name.clone(), *state);
            let simple_name = type_name
                .rsplit("::")
                .next()
                .unwrap_or(type_name.as_str())
                .to_owned();
            simple_type_validation_map
                .entry(simple_name)
                .or_default()
                .push(*state);
        }
        parsed_files.push((rel_path, parsed));
    }

    let global_manual_validate_types: BTreeSet<_> = parsed_files
        .iter()
        .flat_map(|(_, parsed)| parsed.manual_validate_impls.iter().cloned())
        .collect();
    let mut simple_manual_validate_counts = BTreeMap::<String, usize>::new();
    for type_name in &global_manual_validate_types {
        let simple_name = type_name
            .rsplit("::")
            .next()
            .unwrap_or(type_name.as_str())
            .to_owned();
        *simple_manual_validate_counts
            .entry(simple_name)
            .or_insert(0) += 1;
    }

    for (rel_path, parsed) in parsed_files {
        for target in parsed.derived_types {
            let fact = DerivedBoundaryTypeFacts {
                rel_path: rel_path.clone(),
                line: target.line,
                name: target.name,
                boundary_kind: target.boundary_kind,
                boundary_macros: target.boundary_macros,
                has_validate: target.has_validate_derive,
            };
            match target.boundary_kind {
                BoundaryKind::Struct if target.has_non_primitive_fields => {
                    struct_targets.push(fact)
                }
                BoundaryKind::Enum if target.has_non_primitive_fields => enum_targets.push(fact),
                _ => {}
            }
        }

        for manual_impl in parsed.manual_deserialize_impls {
            let resolved = resolve_validation_state(
                std::slice::from_ref(&manual_impl.type_name),
                &global_type_validation_map,
                &simple_type_validation_map,
                &global_manual_validate_types,
                &simple_manual_validate_counts,
            );
            let has_manual_validate = parsed
                .manual_validate_impls
                .contains(&manual_impl.type_name);
            let needs_validate = resolved.map_or(true, |(has_non_primitive, _)| has_non_primitive);
            let has_validate =
                resolved.is_some_and(|(_, has_validate)| has_validate) || has_manual_validate;
            manual_deserialize_impls.push(ManualDeserializeImplFacts {
                rel_path: rel_path.clone(),
                line: manual_impl.line,
                type_name: manual_impl.type_name,
                needs_validate,
                has_validate,
            });
        }

        for field in parsed.boundary_fields {
            let boundary_has_validate = field.boundary_has_validate_derive
                || resolve_validation_state(
                    std::slice::from_ref(&field.boundary_name),
                    &global_type_validation_map,
                    &simple_type_validation_map,
                    &global_manual_validate_types,
                    &simple_manual_validate_counts,
                )
                .is_some_and(|(_, has_validate)| has_validate);
            if !boundary_has_validate {
                continue;
            }

            let nested_validated = resolve_validation_state(
                &field.candidate_type_names,
                &global_type_validation_map,
                &simple_type_validation_map,
                &global_manual_validate_types,
                &simple_manual_validate_counts,
            )
            .is_some_and(|(needs_validate, has_validate)| needs_validate && has_validate);

            boundary_fields.push(BoundaryFieldFacts {
                rel_path: rel_path.clone(),
                line: field.line,
                boundary_name: field.boundary_name,
                field_name: field.field_name,
                field_type: field.field_type,
                requires_field_validation: field.requires_field_validation,
                nested_validated,
                has_garde_skip: field.has_garde_skip,
                has_garde_dive: field.has_garde_dive,
                has_meaningful_garde_rule: field.has_meaningful_garde_rule,
                uses_context: field.uses_context,
                boundary_has_context: field.boundary_has_context,
            });
        }

        for macro_use in parsed.query_as_macros {
            query_as_macros.push(QueryAsMacroFacts {
                rel_path: rel_path.clone(),
                line: macro_use.line,
                macro_name: macro_use.macro_name,
            });
        }
    }

    roots.sort_by(|a, b| a.rel_dir.cmp(&b.rel_dir));
    struct_targets.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    enum_targets.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    manual_deserialize_impls.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    boundary_fields.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    query_as_macros.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    input_failures.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.message.cmp(&b.message)));

    GardeFacts {
        roots,
        struct_targets,
        enum_targets,
        manual_deserialize_impls,
        boundary_fields,
        query_as_macros,
        input_failures,
    }
}

#[cfg(test)]
pub(super) fn family_route(
    tree: &guardrail3_domain_project_tree::ProjectTree,
    scoped_files: Option<&std::collections::BTreeSet<String>>,
) -> guardrail3_app_rs_family_mapper::RsGardeRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree.file_content("guardrail3.toml").and_then(|content| {
        toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
    });
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Garde,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::new(
        tree,
        &scope,
        config.as_ref(),
        &selected,
        scoped_files,
    )
    .map_rs_garde()
}

#[cfg(test)]
#[path = "facts_tests/mod.rs"]
mod facts_tests;
