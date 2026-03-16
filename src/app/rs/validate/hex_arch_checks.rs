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

fn is_service_internal(path: &str) -> bool {
    let parts: Vec<&str> = path.split('/').collect();
    parts.len() >= 4
        && parts.first().is_some_and(|s| *s == "apps")
        && parts.get(2).is_some_and(|s| *s == "crates")
}

#[cfg(test)]
#[allow(clippy::indexing_slicing, clippy::expect_used)] // reason: test assertions
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct StubFs { files: BTreeMap<PathBuf, String> }

    impl StubFs {
        fn new() -> Self { Self { files: BTreeMap::new() } }
        fn add(&mut self, p: &str, c: &str) -> &mut Self {
            let _ = self.files.insert(PathBuf::from(p), c.to_owned());
            self
        }
    }

    impl FileSystem for StubFs {
        fn read_file(&self, path: &Path) -> Option<String> {
            self.files.get(path).cloned()
        }
        #[allow(clippy::unnecessary_wraps)] // reason: trait requires Result
        fn read_file_err(&self, path: &Path) -> Result<String, std::io::Error> {
            self.files.get(path).cloned().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "stub")
            })
        }
        fn list_dir(&self, _: &Path) -> Vec<std::fs::DirEntry> { Vec::new() }
        fn metadata(&self, _: &Path) -> Option<std::fs::Metadata> { None }
    }

    #[allow(clippy::type_complexity)] // reason: test helper tuple pairs
    fn project(members: &[(&str, &str)]) -> ProjectInfo {
        ProjectInfo {
            has_rust: true,
            has_typescript: false,
            cargo_workspace_root: Some(PathBuf::from("/ws")),
            workspace_members: members.iter().map(|(n, _)| n.to_string()).collect(),
            workspace_member_dirs: members.iter().map(|(_, d)| d.to_string()).collect(),
            package_json_path: None,
        }
    }

    fn service_cfg() -> CrateConfig {
        CrateConfig {
            layer: Some("composition-root".to_owned()),
            profile: Some("service".to_owned()),
            allowed_deps: None,
        }
    }

    #[test]
    fn r_arch_01_service_missing_domain_dir() {
        let mut fs = StubFs::new();
        let _ = fs.add("/ws/apps/api/Cargo.toml", "[package]\nname = \"api\"");
        let _ = fs.add("/ws/apps/api/crates/adapters/Cargo.toml", "[package]\nname = \"a\"");
        let mut cfgs = BTreeMap::new();
        let _ = cfgs.insert("apps/api".to_owned(), service_cfg());
        let mut r = Vec::new();
        check_hex_arch_structure(&fs, Path::new("/ws"), &cfgs, &mut r);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "R-ARCH-01");
        assert_eq!(r[0].severity, Severity::Warn);
        assert!(r[0].title.contains("domain"));
    }

    #[test]
    fn r_arch_01_service_with_full_structure_ok() {
        let mut fs = StubFs::new();
        let _ = fs.add("/ws/apps/api/crates/domain/Cargo.toml", "[package]\nname=\"d\"");
        let _ = fs.add("/ws/apps/api/crates/adapters/Cargo.toml", "[package]\nname=\"a\"");
        let mut cfgs = BTreeMap::new();
        let _ = cfgs.insert("apps/api".to_owned(), service_cfg());
        let mut r = Vec::new();
        check_hex_arch_structure(&fs, Path::new("/ws"), &cfgs, &mut r);
        assert!(r.is_empty(), "expected no warnings, got: {r:?}");
    }

    #[test]
    fn r_arch_02_domain_depends_on_adapters() {
        let mut fs = StubFs::new();
        let _ = fs.add("/ws/crates/domain/Cargo.toml",
            "[package]\nname=\"domain\"\n[dependencies]\nadapters = { path = \"../adapters\" }\n");
        let _ = fs.add("/ws/crates/adapters/Cargo.toml", "[package]\nname=\"adapters\"");
        let p = project(&[("domain","crates/domain"),("adapters","crates/adapters")]);
        let mut r = Vec::new();
        check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "R-ARCH-02");
        assert_eq!(r[0].severity, Severity::Error);
        assert!(r[0].message.contains("domain"));
        assert!(r[0].message.contains("adapters"));
    }

    #[test]
    fn r_arch_02_app_depends_on_domain_ok() {
        let mut fs = StubFs::new();
        let _ = fs.add("/ws/crates/app/Cargo.toml",
            "[package]\nname=\"app\"\n[dependencies]\ndomain = { path = \"../domain\" }\n");
        let _ = fs.add("/ws/crates/domain/Cargo.toml", "[package]\nname=\"domain\"");
        let p = project(&[("app","crates/app"),("domain","crates/domain")]);
        let mut r = Vec::new();
        check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
        assert!(r.is_empty(), "expected no errors, got: {r:?}");
    }

    #[test]
    fn r_arch_02_ports_depends_on_app() {
        let mut fs = StubFs::new();
        let _ = fs.add("/ws/crates/ports/Cargo.toml",
            "[package]\nname=\"ports\"\n[dependencies]\napp = { path = \"../app\" }\n");
        let _ = fs.add("/ws/crates/app/Cargo.toml", "[package]\nname=\"app\"");
        let p = project(&[("ports","crates/ports"),("app","crates/app")]);
        let mut r = Vec::new();
        check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "R-ARCH-02");
        assert_eq!(r[0].severity, Severity::Error);
        assert!(r[0].message.contains("ports"));
        assert!(r[0].message.contains("app"));
    }

    #[test]
    fn r_arch_03_library_depends_on_service_internal() {
        let mut fs = StubFs::new();
        let _ = fs.add("/ws/packages/lib/Cargo.toml",
            "[package]\nname=\"lib\"\n[dependencies]\ndomain={path=\"../../apps/api/crates/domain\"}\n");
        let p = project(&[("lib","packages/lib"),("domain","apps/api/crates/domain")]);
        let mut r = Vec::new();
        check_library_service_boundary(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "R-ARCH-03");
        assert_eq!(r[0].severity, Severity::Error);
        assert!(r[0].message.contains("lib"));
    }

    #[test]
    fn r_arch_03_library_depends_on_other_package_ok() {
        let mut fs = StubFs::new();
        let _ = fs.add("/ws/packages/lib/Cargo.toml",
            "[package]\nname=\"lib\"\n[dependencies]\ntypes={path=\"../types\"}\n");
        let p = project(&[("lib","packages/lib"),("types","packages/types")]);
        let mut r = Vec::new();
        check_library_service_boundary(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
        assert!(r.is_empty(), "expected no errors, got: {r:?}");
    }

    #[test]
    fn normalize_path_resolves_parent() {
        assert_eq!(normalize_path("a/b/c/domain", "../ports"), "a/b/c/ports");
    }

    #[test]
    fn normalize_path_resolves_deep_parent() {
        assert_eq!(normalize_path("packages/lib", "../../apps/api/crates/d"), "apps/api/crates/d");
    }

    #[test]
    fn contains_segment_exact() {
        assert!(contains_segment("domain", "domain"));
        assert!(contains_segment("a/domain", "domain"));
        assert!(contains_segment("a/app", "app"));
        assert!(!contains_segment("a/app-extra", "app"));
    }

    #[test]
    fn is_service_internal_matches() {
        assert!(is_service_internal("apps/api/crates/domain"));
        assert!(is_service_internal("apps/be/crates/adapters"));
        assert!(!is_service_internal("packages/lib"));
        assert!(!is_service_internal("apps/api/src/main.rs"));
    }

    #[test]
    fn layer_from_config_maps_correctly() {
        assert_eq!(layer_from_config("domain"), Some(Layer::Domain));
        assert_eq!(layer_from_config("pure"), Some(Layer::Domain));
        assert_eq!(layer_from_config("ports"), Some(Layer::Ports));
        assert_eq!(layer_from_config("app"), Some(Layer::App));
        assert_eq!(layer_from_config("adapters"), Some(Layer::Adapters));
        assert_eq!(layer_from_config("composition-root"), Some(Layer::Adapters));
        assert_eq!(layer_from_config("unknown"), None);
    }
}
