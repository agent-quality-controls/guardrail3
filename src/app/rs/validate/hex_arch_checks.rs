use std::collections::BTreeMap;
use std::path::Path;

use crate::app::discover::ProjectInfo;
use crate::domain::config::types::CrateConfig;
use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layer {
    Domain,
    Ports,
    App,
    Adapters,
}

impl Layer {
    const fn forbidden(self) -> &'static [Self] {
        match self {
            Self::Domain => &[Self::Ports, Self::App, Self::Adapters],
            Self::Ports => &[Self::App, Self::Adapters],
            Self::App => &[Self::Adapters],
            Self::Adapters => &[],
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Ports => "ports",
            Self::App => "app",
            Self::Adapters => "adapters",
        }
    }
}

fn layer_from_config(value: &str) -> Option<Layer> {
    match value {
        "domain" | "pure" => Some(Layer::Domain),
        "ports" => Some(Layer::Ports),
        "app" => Some(Layer::App),
        "adapters" | "composition-root" => Some(Layer::Adapters),
        _ => None,
    }
}

fn layer_from_path(dir: &str) -> Option<Layer> {
    if contains_segment(dir, "domain") {
        Some(Layer::Domain)
    } else if contains_segment(dir, "ports") {
        Some(Layer::Ports)
    } else if contains_segment(dir, "app") {
        Some(Layer::App)
    } else if contains_segment(dir, "adapters") {
        Some(Layer::Adapters)
    } else {
        None
    }
}

fn contains_segment(path: &str, segment: &str) -> bool {
    path.split('/').any(|s| s == segment)
}

fn resolve_layer(
    name: &str,
    dir: &str,
    cfgs: &BTreeMap<String, CrateConfig>,
) -> Option<Layer> {
    if let Some(cfg) = cfgs.get(name) {
        if let Some(ref l) = cfg.layer {
            let resolved = layer_from_config(l);
            if resolved.is_some() {
                return resolved;
            }
        }
    }
    layer_from_path(dir)
}

/// Maps crate name to (`member_dir`, layer).
type CrateLayerMap = BTreeMap<String, (String, Layer)>;

// R-ARCH-01: Service must have hex arch structure
pub fn check_hex_arch_structure(
    fs: &dyn FileSystem,
    root: &Path,
    cfgs: &BTreeMap<String, CrateConfig>,
    results: &mut Vec<CheckResult>,
) {
    for (name, cfg) in cfgs {
        if cfg.profile.as_deref().is_none_or(|p| p != "service") {
            continue;
        }
        let dir = root.join(name);
        for sub in &["domain", "adapters"] {
            let ws = dir.join("crates").join(sub).join("Cargo.toml");
            let sc = dir.join("src").join(sub).join("mod.rs");
            if fs.read_file(&ws).is_none() && fs.read_file(&sc).is_none() {
                results.push(CheckResult {
                    id: "R-ARCH-01".to_owned(),
                    severity: Severity::Warn,
                    title: format!("Service `{name}` missing {sub} layer"),
                    message: format!(
                        "Service `{name}` has profile=\"service\" but no \
                         crates/{sub}/Cargo.toml or src/{sub}/mod.rs found."
                    ),
                    file: Some(dir.display().to_string()),
                    line: None,
                });
            }
        }
    }
}

// R-ARCH-02: Dependency flow violation
pub fn check_dependency_flow(
    fs: &dyn FileSystem,
    root: &Path,
    project: &ProjectInfo,
    cfgs: &BTreeMap<String, CrateConfig>,
    results: &mut Vec<CheckResult>,
) {
    let mut layers: CrateLayerMap = BTreeMap::new();
    for (idx, dir) in project.workspace_member_dirs.iter().enumerate() {
        let name = project.workspace_members.get(idx)
            .map_or(dir.as_str(), std::string::String::as_str);
        if let Some(layer) = resolve_layer(name, dir, cfgs) {
            let _ = layers.insert(name.to_owned(), (dir.clone(), layer));
        }
    }

    let mut dir_to_layer: BTreeMap<String, Layer> = BTreeMap::new();
    for (name, (dir, layer)) in &layers {
        let _ = dir_to_layer.insert(dir.clone(), *layer);
        let _ = dir_to_layer.insert(name.clone(), *layer);
    }

    for (crate_name, (member_dir, src_layer)) in &layers {
        let cargo = root.join(member_dir).join("Cargo.toml");
        let Some(content) = fs.read_file(&cargo) else { continue };
        let Ok(table) = content.parse::<toml::Value>() else { continue };
        let Some(deps) = table.get("dependencies").and_then(|d| d.as_table()) else {
            continue;
        };
        let forbidden = src_layer.forbidden();
        for (dep_name, dep_val) in deps {
            let dep_path = extract_path_dep(dep_val);
            if dep_path.is_none() { continue; }
            let tgt = resolve_dep_layer(dep_name, dep_path, member_dir, &layers, &dir_to_layer);
            if let Some(tgt_layer) = tgt {
                if forbidden.contains(&tgt_layer) {
                    results.push(CheckResult {
                        id: "R-ARCH-02".to_owned(),
                        severity: Severity::Error,
                        title: "Dependency flow violation".to_owned(),
                        message: format!(
                            "{} crate `{crate_name}` ({member_dir}) depends on \
                             {} crate `{dep_name}`",
                            src_layer.label(), tgt_layer.label(),
                        ),
                        file: Some(cargo.display().to_string()),
                        line: None,
                    });
                }
            }
        }
    }
}

fn extract_path_dep(value: &toml::Value) -> Option<&str> {
    value.as_table().and_then(|t| t.get("path")).and_then(|p| p.as_str())
}

fn resolve_dep_layer(
    name: &str,
    dep_path: Option<&str>,
    source_dir: &str,
    layers: &CrateLayerMap,
    dir_map: &BTreeMap<String, Layer>,
) -> Option<Layer> {
    if let Some((_, layer)) = layers.get(name) {
        return Some(*layer);
    }
    if let Some(rel) = dep_path {
        let resolved = normalize_path(source_dir, rel);
        if let Some(layer) = dir_map.get(&resolved) {
            return Some(*layer);
        }
        return layer_from_path(&resolved);
    }
    None
}

fn normalize_path(base: &str, rel: &str) -> String {
    let mut parts: Vec<&str> = base.split('/').collect();
    for seg in rel.split('/') {
        match seg {
            ".." => { let _ = parts.pop(); }
            "." | "" => {}
            s => parts.push(s),
        }
    }
    parts.join("/")
}

// R-ARCH-03: Library depends on service internals
pub fn check_library_service_boundary(
    fs: &dyn FileSystem,
    root: &Path,
    project: &ProjectInfo,
    cfgs: &BTreeMap<String, CrateConfig>,
    results: &mut Vec<CheckResult>,
) {
    for (idx, dir) in project.workspace_member_dirs.iter().enumerate() {
        let name = project.workspace_members.get(idx)
            .map_or(dir.as_str(), std::string::String::as_str);
        let lib_by_cfg = cfgs.get(name)
            .and_then(|c| c.profile.as_deref())
            .is_some_and(|p| p == "library");
        if !lib_by_cfg && !dir.starts_with("packages/") { continue; }

        let cargo = root.join(dir).join("Cargo.toml");
        let Some(content) = fs.read_file(&cargo) else { continue };
        let Ok(table) = content.parse::<toml::Value>() else { continue };
        let Some(deps) = table.get("dependencies").and_then(|d| d.as_table()) else {
            continue;
        };
        for (dep_name, dep_val) in deps {
            let Some(rel) = extract_path_dep(dep_val) else { continue };
            let resolved = normalize_path(dir, rel);
            if is_service_internal(&resolved) {
                results.push(CheckResult {
                    id: "R-ARCH-03".to_owned(),
                    severity: Severity::Error,
                    title: "Library depends on service internals".to_owned(),
                    message: format!(
                        "Library `{name}` ({dir}) depends on `{dep_name}` \
                         which resolves to {resolved} inside a service's crates."
                    ),
                    file: Some(cargo.display().to_string()),
                    line: None,
                });
            }
        }
    }
}

// R-ARCH-04: Workspace members must be configured + single-crate service must be in apps/
pub fn check_unconfigured_members(
    fs: &dyn FileSystem,
    root: &Path,
    project: &ProjectInfo,
    cfgs: &BTreeMap<String, CrateConfig>,
    profile: &str,
    results: &mut Vec<CheckResult>,
) {
    // Single crate (no workspace): if profile is service, check for apps/ structure
    if project.workspace_member_dirs.is_empty() {
        if profile == "service" {
            // Check if this is already inside an apps/ directory
            let root_str = root.display().to_string();
            let in_apps = root_str.contains("/apps/");

            // Check if apps/ exists at the root (workspace that should have apps/)
            let has_apps_dir = fs.read_file(&root.join("apps")).is_some()
                || root.join("apps").exists();

            if !in_apps && !has_apps_dir {
                results.push(CheckResult {
                    id: "R-ARCH-04".to_owned(),
                    severity: Severity::Error,
                    title: "Service not in apps/ directory".to_owned(),
                    message: "Profile is \"service\" but project is not inside an apps/ \
                             directory. Services must live in apps/<name>/ with hex arch \
                             structure (crates/domain, crates/ports, crates/app, \
                             crates/adapters). Shared libraries go in packages/."
                        .to_owned(),
                    file: Some("guardrail3.toml".to_owned()),
                    line: None,
                });
            }
        }
        return;
    }

    // Workspace: if profile is service and no per-crate configs, error
    if cfgs.is_empty() && profile == "service" {
        results.push(CheckResult {
            id: "R-ARCH-04".to_owned(),
            severity: Severity::Error,
            title: "No per-crate configuration".to_owned(),
            message: format!(
                "Profile is \"service\" but no [rust.crates.*] sections in guardrail3.toml. \
                 Configure each workspace member with profile and layer. \
                 Members: {}",
                project.workspace_member_dirs.join(", ")
            ),
            file: Some("guardrail3.toml".to_owned()),
            line: None,
        });
        return;
    }

    // Check each workspace member has a config entry
    for member_dir in &project.workspace_member_dirs {
        let crate_name = member_dir.rsplit('/').next().unwrap_or(member_dir);
        if !cfgs.contains_key(crate_name) && !cfgs.contains_key(member_dir.as_str()) {
            results.push(CheckResult {
                id: "R-ARCH-04".to_owned(),
                severity: Severity::Warn,
                title: format!("Workspace member `{crate_name}` not configured"),
                message: format!(
                    "No [rust.crates.{crate_name}] in guardrail3.toml. \
                     Add profile, layer, and allowed_deps."
                ),
                file: Some("guardrail3.toml".to_owned()),
                line: None,
            });
        }
    }
}

fn is_service_internal(path: &str) -> bool {
    let parts: Vec<&str> = path.split('/').collect();
    parts.len() >= 4
        && parts.first().is_some_and(|s| *s == "apps")
        && parts.get(2).is_some_and(|s| *s == "crates")
}

#[cfg(test)]
#[allow(clippy::indexing_slicing, clippy::expect_used)] // reason: test assertions
#[path = "hex_arch_checks_tests.rs"]
mod tests;
