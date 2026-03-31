mod measurements;
mod package_support;

use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::{RsArchRoute, RsProjectSurface as ProjectTree};

use self::measurements::collect_measurements;
use self::package_support::{fallback_name, library_rel_path, package_name};

#[derive(Debug, Clone, Default)]
pub(crate) struct ArchFacts {
    pub packages: Vec<LibraryPackageFacts>,
}

#[derive(Debug, Clone)]
pub(crate) struct LibraryPackageFacts {
    pub package_rel_dir: String,
    pub cargo_rel_path: String,
    pub package_name: String,
    pub has_package: bool,
    pub is_library: bool,
    pub cargo_parse_error: Option<String>,
    pub measurement_error: Option<String>,
    pub escalation_required: bool,
    pub threshold_reasons: Vec<String>,
    pub is_workspace: bool,
    pub internal_members: Vec<InternalMemberFacts>,
    pub uses_split_mode: bool,
    pub external_dependency_hits: Vec<ExternalDependencyHitFacts>,
}

impl LibraryPackageFacts {
    #[must_use]
    pub fn split_rules_active(&self) -> bool {
        self.escalation_required || self.uses_split_mode
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InternalMemberFacts {
    pub rel_dir: String,
    pub package_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ExternalDependencyHitFacts {
    pub consumer_rel_dir: String,
    pub consumer_cargo_rel_path: String,
    pub consumer_package_name: String,
    pub internal_member_package_name: String,
}

#[derive(Debug, Clone, Default)]
struct CargoSnapshot {
    value: Option<toml::Value>,
    parse_error: Option<String>,
}

#[derive(Debug, Clone)]
struct RootSnapshot {
    rel_dir: String,
    cargo_rel_path: String,
    parsed: Option<toml::Value>,
    parse_error: Option<String>,
    package_name: String,
    has_package: bool,
    is_library: bool,
    lib_rel_path: Option<String>,
    direct_dependencies: BTreeSet<String>,
}

pub fn collect(tree: &ProjectTree, route: &RsArchRoute) -> ArchFacts {
    let snapshots = collect_root_snapshots(tree, route);

    let mut packages = snapshots
        .values()
        .filter(|root| is_arch_candidate_root(root, &snapshots))
        .filter(|root| !has_arch_candidate_ancestor(root, &snapshots))
        .map(|root| collect_package(tree, root, &snapshots))
        .collect::<Vec<_>>();
    packages.sort_by(|left, right| left.package_rel_dir.cmp(&right.package_rel_dir));
    ArchFacts { packages }
}

fn collect_root_snapshots(
    tree: &ProjectTree,
    route: &RsArchRoute,
) -> BTreeMap<String, RootSnapshot> {
    route
        .roots()
        .iter()
        .map(|root| {
            let snapshot = collect_root_snapshot(tree, root.rel_dir(), root.cargo_rel_path());
            (snapshot.rel_dir.clone(), snapshot)
        })
        .collect()
}

fn collect_root_snapshot(tree: &ProjectTree, rel_dir: &str, cargo_rel_path: &str) -> RootSnapshot {
    let cargo = parse_cargo_snapshot(tree, cargo_rel_path);
    let parsed = cargo.value.as_ref();
    let package_name = parsed
        .and_then(package_name)
        .unwrap_or_else(|| fallback_name(rel_dir));
    let has_package = parsed.is_some_and(|value| value.get("package").is_some());
    let lib_rel_path = parsed.and_then(|value| library_rel_path(rel_dir, value));
    let default_lib_rel_path = ProjectTree::join_rel(rel_dir, "src/lib.rs");
    let has_default_lib = tree.file_exists(&default_lib_rel_path);
    let is_library =
        has_package && (parsed.is_some_and(|value| value.get("lib").is_some()) || has_default_lib);
    let lib_rel_path = if is_library {
        Some(lib_rel_path.unwrap_or(default_lib_rel_path))
    } else {
        None
    };
    let direct_dependencies = parsed
        .map(|value| collect_direct_dependencies(value))
        .unwrap_or_default();

    RootSnapshot {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: cargo_rel_path.to_owned(),
        parsed: cargo.value,
        parse_error: cargo.parse_error,
        package_name,
        has_package,
        is_library,
        lib_rel_path,
        direct_dependencies,
    }
}

fn collect_package(
    tree: &ProjectTree,
    root: &RootSnapshot,
    snapshots: &BTreeMap<String, RootSnapshot>,
) -> LibraryPackageFacts {
    let (
        measurement_error,
        _dependency_count,
        _max_module_depth,
        _max_sibling_dirs,
        _max_sibling_rs_files,
        threshold_reasons,
    ) = collect_measurements(
        tree,
        root.parsed.as_ref(),
        root.lib_rel_path.as_deref(),
        root.parse_error.as_deref(),
    );
    let escalation_required = !threshold_reasons.is_empty();
    let is_workspace = root
        .parsed
        .as_ref()
        .is_some_and(|value| value.get("workspace").is_some());

    let mut internal_members = snapshots
        .values()
        .filter(|candidate| candidate.rel_dir != root.rel_dir)
        .filter(|candidate| is_descendant_rel(&root.rel_dir, &candidate.rel_dir))
        .map(|candidate| InternalMemberFacts {
            rel_dir: candidate.rel_dir.clone(),
            package_name: candidate.package_name.clone(),
        })
        .collect::<Vec<_>>();
    internal_members.sort_by(|left, right| left.rel_dir.cmp(&right.rel_dir));

    let uses_split_mode = is_workspace || !internal_members.is_empty();
    let external_dependency_hits =
        collect_external_dependency_hits(root, &internal_members, snapshots);

    LibraryPackageFacts {
        package_rel_dir: root.rel_dir.clone(),
        cargo_rel_path: root.cargo_rel_path.clone(),
        package_name: root.package_name.clone(),
        has_package: root.has_package,
        is_library: root.is_library,
        cargo_parse_error: root.parse_error.clone(),
        measurement_error,
        escalation_required,
        threshold_reasons,
        is_workspace,
        internal_members,
        uses_split_mode,
        external_dependency_hits,
    }
}

fn collect_external_dependency_hits(
    root: &RootSnapshot,
    internal_members: &[InternalMemberFacts],
    snapshots: &BTreeMap<String, RootSnapshot>,
) -> Vec<ExternalDependencyHitFacts> {
    let internal_names = internal_members
        .iter()
        .map(|member| member.package_name.as_str())
        .collect::<BTreeSet<_>>();

    let mut hits = Vec::new();
    for consumer in snapshots.values() {
        if consumer.rel_dir == root.rel_dir || is_descendant_rel(&root.rel_dir, &consumer.rel_dir) {
            continue;
        }
        for dep in &consumer.direct_dependencies {
            if internal_names.contains(dep.as_str()) {
                hits.push(ExternalDependencyHitFacts {
                    consumer_rel_dir: consumer.rel_dir.clone(),
                    consumer_cargo_rel_path: consumer.cargo_rel_path.clone(),
                    consumer_package_name: consumer.package_name.clone(),
                    internal_member_package_name: dep.clone(),
                });
            }
        }
    }
    hits.sort_by(|left, right| {
        left.consumer_rel_dir
            .cmp(&right.consumer_rel_dir)
            .then(left.internal_member_package_name.cmp(&right.internal_member_package_name))
    });
    hits
}

fn parse_cargo_snapshot(tree: &ProjectTree, cargo_rel_path: &str) -> CargoSnapshot {
    let Some(content) = tree.file_content(cargo_rel_path) else {
        return CargoSnapshot {
            value: None,
            parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
        };
    };
    match toml::from_str::<toml::Value>(content) {
        Ok(value) => CargoSnapshot {
            value: Some(value),
            parse_error: None,
        },
        Err(error) => CargoSnapshot {
            value: None,
            parse_error: Some(error.to_string()),
        },
    }
}

fn collect_direct_dependencies(parsed: &toml::Value) -> BTreeSet<String> {
    let workspace_dependencies = parsed
        .get("workspace")
        .and_then(|value| value.get("dependencies"))
        .and_then(toml::Value::as_table);

    let mut deps = dependency_names(parsed.get("dependencies"), workspace_dependencies);
    deps.extend(dependency_names(
        parsed.get("build-dependencies"),
        workspace_dependencies,
    ));
    deps.extend(dependency_names(
        parsed.get("dev-dependencies"),
        workspace_dependencies,
    ));
    deps.extend(target_dependency_names(parsed, workspace_dependencies));
    deps
}

fn dependency_names(
    section: Option<&toml::Value>,
    workspace_dependencies: Option<&toml::map::Map<String, toml::Value>>,
) -> BTreeSet<String> {
    let Some(table) = section.and_then(toml::Value::as_table) else {
        return BTreeSet::new();
    };
    table
        .iter()
        .map(|(alias, value)| dependency_package_name(alias, value, workspace_dependencies))
        .collect()
}

fn target_dependency_names(
    parsed: &toml::Value,
    workspace_dependencies: Option<&toml::map::Map<String, toml::Value>>,
) -> BTreeSet<String> {
    let mut deps = BTreeSet::new();
    if let Some(target_table) = parsed.get("target").and_then(toml::Value::as_table) {
        for table in target_table.values().filter_map(toml::Value::as_table) {
            deps.extend(dependency_names(table.get("dependencies"), workspace_dependencies));
            deps.extend(dependency_names(
                table.get("build-dependencies"),
                workspace_dependencies,
            ));
            deps.extend(dependency_names(
                table.get("dev-dependencies"),
                workspace_dependencies,
            ));
        }
    }
    deps
}

fn dependency_package_name(
    alias: &str,
    value: &toml::Value,
    workspace_dependencies: Option<&toml::map::Map<String, toml::Value>>,
) -> String {
    let dep_table = value.as_table();
    let workspace_value = dep_table
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .filter(|enabled| *enabled)
        .and_then(|_| workspace_dependencies.and_then(|deps| deps.get(alias)));

    dep_table
        .and_then(|table| table.get("package"))
        .and_then(toml::Value::as_str)
        .or_else(|| {
            workspace_value
                .and_then(toml::Value::as_table)
                .and_then(|table| table.get("package"))
                .and_then(toml::Value::as_str)
        })
        .unwrap_or(alias)
        .to_owned()
}

fn is_arch_candidate_root(
    root: &RootSnapshot,
    snapshots: &BTreeMap<String, RootSnapshot>,
) -> bool {
    if !is_packages_root(&root.rel_dir) {
        return false;
    }

    root.is_library
        || root.parsed
            .as_ref()
            .is_some_and(|value| value.get("workspace").is_some())
        || snapshots
            .values()
            .any(|candidate| candidate.rel_dir != root.rel_dir && is_descendant_rel(&root.rel_dir, &candidate.rel_dir))
}

fn has_arch_candidate_ancestor(
    root: &RootSnapshot,
    snapshots: &BTreeMap<String, RootSnapshot>,
) -> bool {
    snapshots.values().any(|candidate| {
        candidate.rel_dir != root.rel_dir
            && is_arch_candidate_root(candidate, snapshots)
            && is_descendant_rel(&candidate.rel_dir, &root.rel_dir)
    })
}

fn is_packages_root(rel_dir: &str) -> bool {
    rel_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .windows(2)
        .any(|window| matches!(window, ["packages", _]))
}

fn is_descendant_rel(parent: &str, child: &str) -> bool {
    child
        .strip_prefix(parent)
        .is_some_and(|rest| rest.starts_with('/'))
}
