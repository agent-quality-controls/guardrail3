mod measurements;
mod package_support;

use std::collections::BTreeSet;

use guardrail3_app_rs_family_mapper::RsLibarchRoute;
use guardrail3_domain_project_tree::ProjectTree;

use self::measurements::collect_measurements;
use self::package_support::{
    collect_facade_exports, facade_source_error, fallback_name, library_crate_name,
    library_rel_path, package_name, parse_workspace_members,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LayerName {
    Api,
    Core,
    Infra,
}

impl LayerName {
    #[must_use]
    pub fn from_dir_name(value: &str) -> Option<Self> {
        match value {
            "api" => Some(Self::Api),
            "core" => Some(Self::Core),
            "infra" => Some(Self::Infra),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LibarchFacts {
    pub packages: Vec<LibraryPackageFacts>,
}

#[derive(Debug, Clone)]
pub(crate) struct LibraryPackageFacts {
    pub package_rel_dir: String,
    pub cargo_rel_path: String,
    pub has_package: bool,
    pub is_library: bool,
    pub cargo_parse_error: Option<String>,
    pub lib_rel_path: Option<String>,
    pub facade_source_error: Option<String>,
    pub measurement_error: Option<String>,
    pub escalation_required: bool,
    pub threshold_reasons: Vec<String>,
    pub is_workspace: bool,
    pub workspace_members: Vec<WorkspaceMemberFacts>,
    pub workspace_members_parse_error: Option<String>,
    pub crates_dir_exists: bool,
    pub layer_dirs: Vec<LayerDirFacts>,
    pub uses_layered_mode: bool,
    pub facade_exports: Vec<FacadeExportFacts>,
    pub member_manifests: Vec<LayerMemberFacts>,
}

impl LibraryPackageFacts {
    #[must_use]
    pub fn layered_rules_active(&self) -> bool {
        self.is_library && (self.escalation_required || self.uses_layered_mode)
    }

    #[must_use]
    pub fn layer_dir(&self, layer: LayerName) -> Option<&LayerDirFacts> {
        self.layer_dirs.iter().find(|dir| dir.layer == Some(layer))
    }

    #[must_use]
    pub fn layer_member(&self, layer: LayerName) -> Option<&LayerMemberFacts> {
        self.member_manifests.iter().find(|member| member.layer == Some(layer))
    }

    #[must_use]
    pub fn expected_layer_dir_rels(&self) -> Vec<String> {
        self.layer_dirs
            .iter()
            .filter(|dir| dir.layer.is_some())
            .map(|dir| dir.rel_dir.clone())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WorkspaceMemberFacts {
    pub resolved_dirs: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct LayerDirFacts {
    pub name: String,
    pub rel_dir: String,
    pub layer: Option<LayerName>,
}

#[derive(Debug, Clone)]
pub(crate) struct FacadeExportFacts {
    pub crate_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct LayerMemberFacts {
    pub layer: Option<LayerName>,
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub package_name: String,
    pub lib_crate_name: String,
    pub parse_error: Option<String>,
    pub direct_dependencies: BTreeSet<String>,
}

#[derive(Debug, Clone, Default)]
struct CargoSnapshot {
    value: Option<toml::Value>,
    parse_error: Option<String>,
}

pub fn collect(tree: &ProjectTree, route: &RsLibarchRoute) -> LibarchFacts {
    let mut packages = route
        .roots()
        .iter()
        .map(|root| collect_package(tree, root.rel_dir(), root.cargo_rel_path()))
        .collect::<Vec<_>>();
    packages.sort_by(|left, right| left.package_rel_dir.cmp(&right.package_rel_dir));
    LibarchFacts { packages }
}

fn collect_package(tree: &ProjectTree, package_rel_dir: &str, cargo_rel_path: &str) -> LibraryPackageFacts {
    let cargo = parse_cargo_snapshot(tree, cargo_rel_path);
    let parsed = cargo.value.as_ref();
    let _package_name = parsed
        .and_then(|value| package_name(value))
        .unwrap_or_else(|| fallback_name(package_rel_dir));
    let has_package = parsed.is_some_and(|value| value.get("package").is_some());
    let lib_rel_path = parsed.and_then(|value| library_rel_path(package_rel_dir, value));
    let default_lib_rel_path = ProjectTree::join_rel(package_rel_dir, "src/lib.rs");
    let has_default_lib = tree.file_exists(&default_lib_rel_path);
    let is_library = has_package && (parsed.is_some_and(|value| value.get("lib").is_some()) || has_default_lib);

    let lib_rel_path = if is_library {
        Some(lib_rel_path.unwrap_or(default_lib_rel_path))
    } else {
        None
    };

    let (measurement_error, _dependency_count, _max_module_depth, _max_sibling_dirs, _max_sibling_rs_files, threshold_reasons) =
        collect_measurements(tree, parsed, lib_rel_path.as_deref(), cargo.parse_error.as_deref());
    let escalation_required = !threshold_reasons.is_empty();

    let is_workspace = parsed.is_some_and(|value| value.get("workspace").is_some());
    let (workspace_members, workspace_members_parse_error) =
        parse_workspace_members(tree, package_rel_dir, parsed, cargo.parse_error.as_deref());
    let workspace_dependencies = parsed
        .and_then(|value| value.get("workspace"))
        .and_then(|value| value.get("dependencies"))
        .and_then(toml::Value::as_table)
        .cloned()
        .unwrap_or_default();

    let crates_rel_dir = ProjectTree::join_rel(package_rel_dir, "crates");
    let crates_dir_exists = tree.dir_exists(&crates_rel_dir);
    let layer_dirs = tree
        .dir_contents(&crates_rel_dir)
        .map(|entry| {
            let mut dirs = entry
                .dirs()
                .iter()
                .map(|name| LayerDirFacts {
                    name: name.to_owned(),
                    rel_dir: ProjectTree::join_rel(&crates_rel_dir, name),
                    layer: LayerName::from_dir_name(name),
                })
                .collect::<Vec<_>>();
            dirs.sort_by(|left, right| left.rel_dir.cmp(&right.rel_dir));
            dirs
        })
        .unwrap_or_default();
    let uses_layered_mode =
        is_workspace || crates_dir_exists || !workspace_members.is_empty() || !layer_dirs.is_empty();

    let facade_exports = collect_facade_exports(tree, lib_rel_path.as_deref());
    let facade_source_error = facade_source_error(tree, lib_rel_path.as_deref());
    let member_manifests = collect_member_manifests(tree, &layer_dirs, &workspace_dependencies);

    LibraryPackageFacts {
        package_rel_dir: package_rel_dir.to_owned(),
        cargo_rel_path: cargo_rel_path.to_owned(),
        has_package,
        is_library,
        cargo_parse_error: cargo.parse_error,
        lib_rel_path,
        facade_source_error,
        measurement_error,
        escalation_required,
        threshold_reasons,
        is_workspace,
        workspace_members,
        workspace_members_parse_error,
        crates_dir_exists,
        layer_dirs,
        uses_layered_mode,
        facade_exports,
        member_manifests,
    }
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

fn collect_member_manifests(
    tree: &ProjectTree,
    layer_dirs: &[LayerDirFacts],
    workspace_dependencies: &toml::map::Map<String, toml::Value>,
) -> Vec<LayerMemberFacts> {
    let mut members = Vec::new();
    for layer_dir in layer_dirs.iter().filter(|dir| dir.layer.is_some()) {
        let cargo_rel_path = ProjectTree::join_rel(&layer_dir.rel_dir, "Cargo.toml");
        let cargo = parse_cargo_snapshot(tree, &cargo_rel_path);
        let parsed = cargo.value.as_ref();
        let package_name = parsed
            .and_then(|value| package_name(value))
            .unwrap_or_else(|| fallback_name(&layer_dir.rel_dir));
        let lib_crate_name = parsed
            .and_then(|value| library_crate_name(value, &package_name))
            .unwrap_or_else(|| package_name.replace('-', "_"));
        let direct_dependencies = parsed
            .map(|value| {
                let mut deps = dependency_names(value.get("dependencies"), Some(workspace_dependencies));
                deps.extend(dependency_names(
                    value.get("build-dependencies"),
                    Some(workspace_dependencies),
                ));
                deps.extend(target_dependency_names(value, workspace_dependencies));
                deps
            })
            .unwrap_or_default();

        members.push(LayerMemberFacts {
            layer: layer_dir.layer,
            rel_dir: layer_dir.rel_dir.clone(),
            cargo_rel_path,
            package_name,
            lib_crate_name,
            parse_error: cargo.parse_error,
            direct_dependencies,
        });
    }
    members.sort_by(|left, right| left.rel_dir.cmp(&right.rel_dir));
    members
}

fn dependency_names(
    section: Option<&toml::Value>,
    workspace_dependencies: Option<&toml::map::Map<String, toml::Value>>,
) -> BTreeSet<String> {
    let Some(table) = section.and_then(toml::Value::as_table) else {
        return BTreeSet::new();
    };
    table.iter()
        .map(|(alias, value)| dependency_package_name(alias, value, workspace_dependencies))
        .collect()
}

fn target_dependency_names(
    parsed: &toml::Value,
    workspace_dependencies: &toml::map::Map<String, toml::Value>,
) -> BTreeSet<String> {
    let mut deps = BTreeSet::new();
    if let Some(target_table) = parsed.get("target").and_then(toml::Value::as_table) {
        for table in target_table.values().filter_map(toml::Value::as_table) {
            deps.extend(dependency_names(
                table.get("dependencies"),
                Some(workspace_dependencies),
            ));
            deps.extend(dependency_names(
                table.get("build-dependencies"),
                Some(workspace_dependencies),
            ));
            deps.extend(dependency_names(
                table.get("dev-dependencies"),
                Some(workspace_dependencies),
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
