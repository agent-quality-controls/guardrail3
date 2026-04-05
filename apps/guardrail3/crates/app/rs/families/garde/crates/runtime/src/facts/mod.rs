mod cargo_roots;
mod clippy;
mod policy;
mod validation;

use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsGardeRoute;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use self::cargo_roots::collect_cargo_roots;
use self::clippy::{collect_clippy_configs, owning_root_dir, push_root_facts};
use self::policy::read_policy_map;
use self::validation::resolve_validation_state;
use super::discover::{is_test_path, rust_file_rels};
use super::parse::{
    BoundaryKind, GuardrailConfigParseKind, ParsedGardeFile, analyze, parse_rust_file,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyRootKind {
    WorkspaceRoot,
}

impl PolicyRootKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace root",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GardeRootFacts {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) cargo_parsed_typed: Option<cargo_toml_parser::CargoToml>,
    pub(crate) kind: PolicyRootKind,
    pub(crate) garde_dependency_present: bool,
    pub(crate) garde_applicable: bool,
    pub(crate) clippy_rel_path: Option<String>,
    pub(crate) clippy_parsed: Option<toml::Value>,
    pub(crate) clippy_parsed_typed: Option<clippy_toml_parser::ClippyToml>,
    pub(crate) clippy_parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DerivedBoundaryTypeFacts {
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) boundary_kind: BoundaryKind,
    pub(crate) boundary_macros: Vec<String>,
    pub(crate) has_validate: bool,
}

#[derive(Debug, Clone)]
pub struct ManualDeserializeImplFacts {
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) type_name: String,
    pub(crate) needs_validate: bool,
    pub(crate) has_validate: bool,
}

#[derive(Debug, Clone)]
pub struct QueryAsMacroFacts {
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) macro_name: String,
    pub(crate) escape_hatch_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BoundaryFieldFacts {
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) boundary_name: String,
    pub(crate) field_name: String,
    pub(crate) field_type: String,
    pub(crate) requires_field_validation: bool,
    pub(crate) nested_validated: bool,
    pub(crate) has_garde_skip: bool,
    pub(crate) has_garde_dive: bool,
    pub(crate) has_meaningful_garde_rule: bool,
    pub(crate) uses_context: bool,
    pub(crate) boundary_has_context: bool,
}

#[derive(Debug, Clone)]
pub struct GardeInputFailureFacts {
    pub(crate) rel_path: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone)]
pub struct GuardrailConfigValidationFacts {
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) parse_kind: GuardrailConfigParseKind,
}

#[derive(Debug, Clone, Default)]
pub struct GardeFacts {
    pub(crate) roots: Vec<GardeRootFacts>,
    pub(crate) struct_targets: Vec<DerivedBoundaryTypeFacts>,
    pub(crate) enum_targets: Vec<DerivedBoundaryTypeFacts>,
    pub(crate) manual_deserialize_impls: Vec<ManualDeserializeImplFacts>,
    pub(crate) boundary_fields: Vec<BoundaryFieldFacts>,
    pub(crate) query_as_macros: Vec<QueryAsMacroFacts>,
    pub(crate) input_failures: Vec<GardeInputFailureFacts>,
    pub(crate) guardrail_config_validation_sites: Vec<GuardrailConfigValidationFacts>,
}

#[derive(Debug, Clone)]
struct CargoRootFacts {
    rel_dir: String,
    has_workspace: bool,
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
    parsed_typed: Option<clippy_toml_parser::ClippyToml>,
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
    let policy_guardrail_rel = route
        .family_files()
        .iter()
        .find(|file| file.kind() == guardrail3_app_rs_ownership::RustFamilyFileKind::GuardrailToml)
        .map(|file| file.rel_path().to_owned());
    let policy_map = read_policy_map(
        tree,
        &cargo_roots,
        policy_guardrail_rel.as_deref(),
        &mut input_failures,
    );
    let clippy_configs = collect_clippy_configs(tree, route, &workspace_roots, &mut input_failures);

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
    let routed_root_dirs: Vec<_> = roots.iter().map(|root| root.rel_dir.clone()).collect();
    let root_escape_hatches = routed_root_dirs
        .iter()
        .map(|root_rel_dir| {
            let guardrail_rel = route
                .family_files()
                .iter()
                .find(|file| {
                    file.kind() == guardrail3_app_rs_ownership::RustFamilyFileKind::GuardrailToml
                        && file.exact_rust_root_owner()
                        && file.logical_owner_rel() == root_rel_dir
                })
                .map(|file| file.rel_path().to_owned());
            let escape_hatches = tree
                .file_content(guardrail_rel.as_deref().unwrap_or(""))
                .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok())
                .map(|config| config.escape_hatches().to_vec())
                .unwrap_or_default();
            (root_rel_dir.clone(), escape_hatches)
        })
        .collect::<BTreeMap<_, _>>();

    let mut struct_targets = Vec::new();
    let mut enum_targets = Vec::new();
    let mut manual_deserialize_impls = Vec::new();
    let mut boundary_fields = Vec::new();
    let mut query_as_macros = Vec::new();
    let mut guardrail_config_validation_sites = Vec::new();
    let mut global_type_validation_map = BTreeMap::<String, (bool, bool)>::new();
    let mut simple_type_validation_map = BTreeMap::<String, Vec<(bool, bool)>>::new();
    let mut parsed_files = Vec::new();
    let mut root_garde_adoption = BTreeSet::new();

    for rel_path in rust_file_rels(tree) {
        if is_test_path(&rel_path) {
            continue;
        }
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&rel_path))
        {
            continue;
        }
        let Some(root_rel_dir) = owning_root_dir(&rel_path, &routed_root_dirs) else {
            continue;
        };
        let Some(abs_path) = tree.abs_path(&rel_path) else { continue };
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
        if parsed_file_shows_garde_adoption(&parsed) {
            let _ = root_garde_adoption.insert(root_rel_dir.to_owned());
        }
        parsed_files.push((root_rel_dir.to_owned(), rel_path, parsed));
    }

    for root in &mut roots {
        root.garde_applicable =
            root.garde_dependency_present || root_garde_adoption.contains(&root.rel_dir);
    }

    let active_root_dirs: BTreeSet<_> = roots
        .iter()
        .filter(|root| root.garde_applicable)
        .map(|root| root.rel_dir.clone())
        .collect();

    for (root_rel_dir, _rel_path, parsed) in &parsed_files {
        if !active_root_dirs.contains(root_rel_dir) {
            continue;
        }
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
    }

    let global_manual_validate_types: BTreeSet<_> = parsed_files
        .iter()
        .flat_map(|(_, _, parsed)| parsed.manual_validate_impls.iter().cloned())
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

    for (root_rel_dir, rel_path, parsed) in parsed_files {
        if !active_root_dirs.contains(&root_rel_dir) {
            continue;
        }
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
            let selector = format!("{}@L{}", macro_use.macro_name, macro_use.line);
            query_as_macros.push(QueryAsMacroFacts {
                rel_path: rel_path.clone(),
                line: macro_use.line,
                macro_name: macro_use.macro_name,
                escape_hatch_reason: root_escape_hatches
                    .get(&root_rel_dir)
                    .into_iter()
                    .flatten()
                    .find(|entry| {
                        entry.family() == "garde"
                            && entry.file() == rel_path
                            && entry.kind() == "sqlx_query_as"
                            && entry.selector() == selector
                    })
                    .map(|entry| entry.reason().to_owned()),
            });
        }

        for site in parsed.guardrail_config_validation_sites {
            guardrail_config_validation_sites.push(GuardrailConfigValidationFacts {
                rel_path: rel_path.clone(),
                line: site.line,
                parse_kind: site.parse_kind,
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
    guardrail_config_validation_sites
        .sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));

    GardeFacts {
        roots,
        struct_targets,
        enum_targets,
        manual_deserialize_impls,
        boundary_fields,
        query_as_macros,
        input_failures,
        guardrail_config_validation_sites,
    }
}

fn parsed_file_shows_garde_adoption(parsed: &ParsedGardeFile) -> bool {
    !parsed.derived_types.is_empty()
        || !parsed.manual_deserialize_impls.is_empty()
        || !parsed.manual_validate_impls.is_empty()
        || parsed
            .type_validation_map
            .values()
            .any(|(_has_non_primitive_fields, has_validate)| *has_validate)
}

#[cfg(test)]
pub(crate) fn family_route(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    scoped_files: Option<&std::collections::BTreeSet<String>>,
) -> guardrail3_app_rs_family_mapper::RsGardeRoute {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let config = tree.file_content("guardrail3.toml").and_then(|content| {
        toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
    });
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Garde,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(
        &legality,
        config.as_ref(),
        &selected,
        scoped_files,
    )
    .map_rs_garde()
}

#[cfg(test)]
mod tests;
