use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_core::discover::resolve_app_paths_from_member_dirs;
use guardrail3_app_rs_family_mapper::RsGardeRoute;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;

use super::discover::{is_test_path, rust_file_rels};
use super::parse::{BoundaryKind, analyze, parse_rust_file};

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

fn resolve_validation_state(
    candidate_names: &[String],
    global_type_validation_map: &BTreeMap<String, (bool, bool)>,
    simple_type_validation_map: &BTreeMap<String, Vec<(bool, bool)>>,
    global_manual_validate_types: &BTreeSet<String>,
    simple_manual_validate_counts: &BTreeMap<String, usize>,
) -> Option<(bool, bool)> {
    for candidate_name in candidate_names {
        if let Some((has_non_primitive, has_validate_derive)) = resolve_exact_validation_state(
            candidate_name,
            global_type_validation_map,
            global_manual_validate_types,
        ) {
            return Some((has_non_primitive, has_validate_derive));
        }

        let stripped_candidate = strip_local_path_prefixes(candidate_name);
        if let Some((has_non_primitive, has_validate_derive)) =
            stripped_candidate.and_then(|local_candidate| {
                resolve_exact_validation_state(
                    local_candidate,
                    global_type_validation_map,
                    global_manual_validate_types,
                )
            })
        {
            return Some((has_non_primitive, has_validate_derive));
        }

        if candidate_name.contains("::") && stripped_candidate.is_none() {
            continue;
        }

        let simple_name = stripped_candidate
            .unwrap_or(candidate_name.as_str())
            .rsplit("::")
            .next()
            .unwrap_or(candidate_name.as_str());
        if let Some(states) = simple_type_validation_map.get(simple_name) {
            if states.len() == 1 {
                return Some((
                    states[0].0,
                    states[0].1
                        || simple_manual_validate_counts.get(simple_name).copied() == Some(1),
                ));
            }
        }
    }
    None
}

fn resolve_exact_validation_state(
    candidate_name: &str,
    global_type_validation_map: &BTreeMap<String, (bool, bool)>,
    global_manual_validate_types: &BTreeSet<String>,
) -> Option<(bool, bool)> {
    if let Some((has_non_primitive, has_validate_derive)) =
        global_type_validation_map.get(candidate_name)
    {
        return Some((
            *has_non_primitive,
            *has_validate_derive || global_manual_validate_types.contains(candidate_name),
        ));
    }
    None
}

fn strip_local_path_prefixes(candidate_name: &str) -> Option<&str> {
    let mut stripped = candidate_name;
    let mut changed = false;
    loop {
        if let Some(rest) = stripped.strip_prefix("crate::") {
            stripped = rest;
            changed = true;
            continue;
        }
        if let Some(rest) = stripped.strip_prefix("self::") {
            stripped = rest;
            changed = true;
            continue;
        }
        if let Some(rest) = stripped.strip_prefix("super::") {
            stripped = rest;
            changed = true;
            continue;
        }
        break;
    }
    changed.then_some(stripped)
}

fn collect_cargo_roots(
    tree: &ProjectTree,
    route: &RsGardeRoute,
    input_failures: &mut Vec<GardeInputFailureFacts>,
) -> BTreeMap<String, CargoRootFacts> {
    route
        .roots
        .iter()
        .map(|root| {
            let rel_dir = root.root.rel_dir.clone();
            let rel_path = root.root.cargo_rel_path.clone();
            let parsed = tree
                .file_content(&rel_path)
                .map(|content| toml::from_str::<toml::Value>(content));
            let facts = match parsed {
                Some(Ok(parsed)) => CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    has_workspace: parsed.get("workspace").is_some(),
                    has_package: parsed.get("package").is_some(),
                    workspace_members: parse_workspace_members(tree, &rel_dir, &parsed),
                },
                Some(Err(parse_error)) => {
                    input_failures.push(GardeInputFailureFacts {
                        rel_path: rel_path.clone(),
                        message: format!(
                            "Failed to parse Cargo.toml for garde root discovery: {parse_error}"
                        ),
                    });
                    CargoRootFacts {
                        rel_dir: rel_dir.clone(),
                        has_workspace: false,
                        has_package: false,
                        workspace_members: Vec::new(),
                    }
                }
                None => CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    has_workspace: false,
                    has_package: false,
                    workspace_members: Vec::new(),
                },
            };
            (rel_dir, facts)
        })
        .collect()
}

fn parse_workspace_members(
    tree: &ProjectTree,
    workspace_rel: &str,
    parsed: &toml::Value,
) -> Vec<String> {
    parsed
        .get("workspace")
        .and_then(|value| value.get("members"))
        .and_then(toml::Value::as_array)
        .map(|members| {
            members
                .iter()
                .filter_map(toml::Value::as_str)
                .flat_map(|member| expand_member_pattern(tree, workspace_rel, member))
                .collect()
        })
        .unwrap_or_default()
}

fn expand_member_pattern(tree: &ProjectTree, workspace_rel: &str, member: &str) -> Vec<String> {
    let trimmed = member.trim_matches('/');
    let pattern = if workspace_rel.is_empty() {
        trimmed.to_owned()
    } else {
        ProjectTree::join_rel(workspace_rel, trimmed)
    };

    if trimmed.contains('*') || trimmed.contains('?') || trimmed.contains('[') {
        tree.matching_dir_rels(&pattern)
    } else {
        vec![pattern]
    }
}

fn read_policy_map(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    standalone_package_roots: &BTreeSet<String>,
    input_failures: &mut Vec<GardeInputFailureFacts>,
) -> BTreeMap<String, PolicySettings> {
    let parsed = match tree.file_content("guardrail3.toml") {
        Some(content) => match toml::from_str::<GuardrailConfig>(content) {
            Ok(parsed) => Some(parsed),
            Err(parse_error) => {
                input_failures.push(GardeInputFailureFacts {
                    rel_path: "guardrail3.toml".to_owned(),
                    message: format!(
                        "Failed to parse guardrail3.toml for garde policy resolution: {parse_error}"
                    ),
                });
                None
            }
        },
        None => None,
    };

    let default_garde = parsed
        .as_ref()
        .and_then(|config| config.rust.as_ref())
        .and_then(|rust| rust.checks.as_ref())
        .and_then(|checks| checks.garde)
        .unwrap_or(true);

    let app_paths = resolve_app_paths_from_member_dirs(
        cargo_roots
            .values()
            .flat_map(|root| root.workspace_members.iter().cloned()),
    );
    let app_paths_include_root = app_paths.values().any(|rel_dir| rel_dir.is_empty());

    let mut map = BTreeMap::from([(
        String::new(),
        PolicySettings {
            garde_enabled: default_garde,
        },
    )]);

    for (app_name, app_dir) in &app_paths {
        let app_cfg = parsed
            .as_ref()
            .and_then(|config| config.rust.as_ref())
            .and_then(|rust| rust.apps.as_ref())
            .and_then(|apps| apps.get(app_name));
        let garde_enabled = app_cfg
            .and_then(crate_checks)
            .and_then(|checks| checks.garde)
            .unwrap_or(default_garde);
        let _ = map.insert(app_dir.clone(), PolicySettings { garde_enabled });
    }

    if let Some(packages_cfg) = parsed
        .as_ref()
        .and_then(|config| config.rust.as_ref())
        .and_then(|rust| rust.packages.as_ref())
    {
        let garde_enabled = crate_checks(packages_cfg)
            .and_then(|checks| checks.garde)
            .unwrap_or(default_garde);
        if !app_paths_include_root {
            let _ = map.insert(String::new(), PolicySettings { garde_enabled });
        }
        for package_dir in standalone_package_roots {
            let _ = map.insert(package_dir.clone(), PolicySettings { garde_enabled });
        }
    }

    map
}

fn crate_checks(
    config: &guardrail3_domain_config::types::CrateConfig,
) -> Option<&guardrail3_domain_config::types::RustChecksConfig> {
    config.checks.as_ref()
}

fn collect_clippy_configs(
    tree: &ProjectTree,
    workspace_roots: &BTreeSet<String>,
    standalone_package_roots: &BTreeSet<String>,
    input_failures: &mut Vec<GardeInputFailureFacts>,
) -> Vec<ClippyConfigCandidate> {
    let mut allowed_policy_roots = BTreeSet::from([String::new()]);
    allowed_policy_roots.extend(workspace_roots.iter().cloned());
    allowed_policy_roots.extend(standalone_package_roots.iter().cloned());

    let mut candidates = Vec::new();
    for file_name in ["clippy.toml", ".clippy.toml"] {
        if tree.file_exists(file_name) && allowed_policy_roots.contains("") {
            candidates.push(parse_clippy_candidate(tree, "", file_name, input_failures));
        }
        for rel_dir in tree.dirs_with_file(file_name) {
            if allowed_policy_roots.contains(&rel_dir) {
                candidates.push(parse_clippy_candidate(
                    tree,
                    &rel_dir,
                    file_name,
                    input_failures,
                ));
            }
        }
    }

    let mut by_dir = BTreeMap::<String, Vec<ClippyConfigCandidate>>::new();
    for candidate in candidates {
        by_dir
            .entry(candidate.rel_dir.clone())
            .or_default()
            .push(candidate);
    }

    let mut deduped = Vec::new();
    for (_rel_dir, mut same_root) in by_dir {
        same_root.sort_by_key(|candidate| config_precedence(&candidate.rel_path));
        if let Some(preferred) = same_root.into_iter().next() {
            deduped.push(preferred);
        }
    }
    deduped
}

fn parse_clippy_candidate(
    tree: &ProjectTree,
    rel_dir: &str,
    file_name: &str,
    input_failures: &mut Vec<GardeInputFailureFacts>,
) -> ClippyConfigCandidate {
    let rel_path = ProjectTree::join_rel(rel_dir, file_name);
    let (parsed, parse_error) = match tree.file_content(&rel_path) {
        Some(content) => match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => (Some(parsed), None),
            Err(parse_error) => {
                let message = format!(
                    "Failed to parse `{rel_path}` for garde clippy-ban validation: {parse_error}"
                );
                input_failures.push(GardeInputFailureFacts {
                    rel_path: rel_path.clone(),
                    message: message.clone(),
                });
                (None, Some(message))
            }
        },
        None => (None, None),
    };

    ClippyConfigCandidate {
        rel_dir: rel_dir.to_owned(),
        rel_path,
        parsed,
        parse_error,
    }
}

fn config_precedence(rel_path: &str) -> usize {
    if rel_path.ends_with("clippy.toml") && !rel_path.ends_with(".clippy.toml") {
        return 0;
    }
    if rel_path.ends_with(".clippy.toml") {
        return 1;
    }
    2
}

fn push_root_facts(
    tree: &ProjectTree,
    rel_dir: &str,
    kind: PolicyRootKind,
    policy_map: &BTreeMap<String, PolicySettings>,
    clippy_configs: &[ClippyConfigCandidate],
    out: &mut Vec<GardeRootFacts>,
) {
    let settings = policy_settings_for(rel_dir, policy_map);
    if !settings.garde_enabled {
        return;
    }
    let cargo_rel_path = if rel_dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        ProjectTree::join_rel(rel_dir, "Cargo.toml")
    };
    let cargo_parsed = tree
        .file_content(&cargo_rel_path)
        .and_then(|content| toml::from_str::<toml::Value>(content).ok());
    let garde_dependency_present = cargo_parsed
        .as_ref()
        .is_some_and(content_has_garde_dependency);

    let covering_config = nearest_covering_clippy(rel_dir, clippy_configs);
    out.push(GardeRootFacts {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path,
        kind,
        garde_dependency_present,
        clippy_rel_path: covering_config.map(|config| config.rel_path.clone()),
        clippy_parsed: covering_config.and_then(|config| config.parsed.clone()),
        clippy_parse_error: covering_config.and_then(|config| config.parse_error.clone()),
    });
}

fn policy_settings_for(
    rel_dir: &str,
    policy_map: &BTreeMap<String, PolicySettings>,
) -> PolicySettings {
    let mut best = policy_map.get("").cloned().unwrap_or(PolicySettings {
        garde_enabled: true,
    });
    let mut best_len = 0usize;

    for (candidate_dir, settings) in policy_map {
        if candidate_dir.is_empty() {
            continue;
        }
        if rel_dir == candidate_dir
            || rel_dir
                .strip_prefix(candidate_dir)
                .is_some_and(|rest| rest.starts_with('/'))
        {
            let len = candidate_dir.len();
            if len > best_len {
                best = settings.clone();
                best_len = len;
            }
        }
    }

    best
}

fn nearest_covering_clippy<'a>(
    rel_dir: &str,
    configs: &'a [ClippyConfigCandidate],
) -> Option<&'a ClippyConfigCandidate> {
    configs
        .iter()
        .filter(|config| {
            config.rel_dir.is_empty()
                || rel_dir == config.rel_dir
                || rel_dir
                    .strip_prefix(&config.rel_dir)
                    .is_some_and(|rest| rest.starts_with('/'))
        })
        .max_by_key(|config| config.rel_dir.len())
}

fn owning_root_dir<'a>(rel_path: &str, root_dirs: &'a [String]) -> Option<&'a str> {
    let parent = file_parent_rel(rel_path);
    root_dirs
        .iter()
        .filter(|root| {
            root.is_empty()
                || parent == root.as_str()
                || parent
                    .strip_prefix(root.as_str())
                    .is_some_and(|rest| rest.starts_with('/'))
        })
        .max_by_key(|root| root.len())
        .map(String::as_str)
}

fn file_parent_rel(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}

fn content_has_garde_dependency(parsed: &toml::Value) -> bool {
    parsed
        .get("workspace")
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(toml::Value::as_table)
        .is_some_and(|deps| deps.contains_key("garde"))
        || parsed
            .get("dependencies")
            .and_then(toml::Value::as_table)
            .is_some_and(|deps| deps.contains_key("garde"))
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
