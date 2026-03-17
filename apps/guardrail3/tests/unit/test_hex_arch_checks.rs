#![allow(clippy::indexing_slicing, clippy::expect_used)] // reason: test assertions

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use guardrail3::app::discover::{ProjectInfo, RustWorkspace, WorkspaceMember};
use guardrail3::app::rs::validate::hex_arch_checks::{
    Layer, check_dependency_flow, check_hex_arch_structure, check_library_service_boundary,
    contains_segment, is_service_internal, layer_from_config, normalize_path,
};
use guardrail3::domain::config::types::CrateConfig;
use guardrail3::domain::report::Severity;
use guardrail3::ports::outbound::FileSystem;

struct StubFs {
    files: BTreeMap<PathBuf, String>,
}

impl StubFs {
    const fn new() -> Self {
        Self {
            files: BTreeMap::new(),
        }
    }
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
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "stub"))
    }
    fn list_dir(&self, _: &Path) -> Vec<std::fs::DirEntry> {
        Vec::new()
    }
    fn metadata(&self, _: &Path) -> Option<std::fs::Metadata> {
        None
    }
}

#[allow(clippy::type_complexity)] // reason: test helper tuple pairs
fn project(members: &[(&str, &str)]) -> ProjectInfo {
    ProjectInfo {
        has_rust: true,
        has_typescript: false,
        workspaces: vec![RustWorkspace {
            root: PathBuf::from("/ws"),
            members: members
                .iter()
                .map(|(n, d)| WorkspaceMember {
                    name: n.to_string(),
                    dir: d.to_string(),
                })
                .collect(),
        }],
        package_json_path: None,
    }
}

fn service_cfg() -> CrateConfig {
    CrateConfig {
        layer: Some("composition-root".to_owned()),
        profile: Some("service".to_owned()),
        type_: None,
        allowed_deps: None,
        checks: None,
    }
}

#[test]
fn r_arch_01_service_missing_domain_dir() {
    let mut fs = StubFs::new();
    let _ = fs.add("/ws/apps/api/Cargo.toml", "[package]\nname = \"api\"");
    let _ = fs.add(
        "/ws/apps/api/crates/adapters/Cargo.toml",
        "[package]\nname = \"a\"",
    );
    let mut cfgs = BTreeMap::new();
    let _ = cfgs.insert("api".to_owned(), service_cfg());
    let p = project(&[("api", "apps/api")]);
    let mut r = Vec::new();
    check_hex_arch_structure(&fs, Path::new("/ws"), &p, &cfgs, &mut r);
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].id, "R-ARCH-01");
    assert_eq!(r[0].severity, Severity::Warn);
    assert!(r[0].title.contains("domain"));
}

#[test]
fn r_arch_01_service_with_full_structure_ok() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/apps/api/crates/domain/Cargo.toml",
        "[package]\nname=\"d\"",
    );
    let _ = fs.add(
        "/ws/apps/api/crates/adapters/Cargo.toml",
        "[package]\nname=\"a\"",
    );
    let mut cfgs = BTreeMap::new();
    let _ = cfgs.insert("api".to_owned(), service_cfg());
    let p = project(&[("api", "apps/api")]);
    let mut r = Vec::new();
    check_hex_arch_structure(&fs, Path::new("/ws"), &p, &cfgs, &mut r);
    assert!(r.is_empty(), "expected no warnings, got: {r:?}");
}

#[test]
fn r_arch_02_domain_depends_on_adapters() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/domain/Cargo.toml",
        "[package]\nname=\"domain\"\n[dependencies]\nadapters = { path = \"../adapters\" }\n",
    );
    let _ = fs.add(
        "/ws/crates/adapters/Cargo.toml",
        "[package]\nname=\"adapters\"",
    );
    let p = project(&[("domain", "crates/domain"), ("adapters", "crates/adapters")]);
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
    let _ = fs.add(
        "/ws/crates/app/Cargo.toml",
        "[package]\nname=\"app\"\n[dependencies]\ndomain = { path = \"../domain\" }\n",
    );
    let _ = fs.add("/ws/crates/domain/Cargo.toml", "[package]\nname=\"domain\"");
    let p = project(&[("app", "crates/app"), ("domain", "crates/domain")]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert!(r.is_empty(), "expected no errors, got: {r:?}");
}

#[test]
fn r_arch_02_ports_depends_on_app() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/ports/Cargo.toml",
        "[package]\nname=\"ports\"\n[dependencies]\napp = { path = \"../app\" }\n",
    );
    let _ = fs.add("/ws/crates/app/Cargo.toml", "[package]\nname=\"app\"");
    let p = project(&[("ports", "crates/ports"), ("app", "crates/app")]);
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
    let _ = fs.add(
        "/ws/packages/lib/Cargo.toml",
        "[package]\nname=\"lib\"\n[dependencies]\ndomain={path=\"../../apps/api/crates/domain\"}\n",
    );
    let p = project(&[
        ("lib", "packages/lib"),
        ("domain", "apps/api/crates/domain"),
    ]);
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
    let _ = fs.add(
        "/ws/packages/lib/Cargo.toml",
        "[package]\nname=\"lib\"\n[dependencies]\ntypes={path=\"../types\"}\n",
    );
    let p = project(&[("lib", "packages/lib"), ("types", "packages/types")]);
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
    assert_eq!(
        normalize_path("packages/lib", "../../apps/api/crates/d"),
        "apps/api/crates/d"
    );
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

// -----------------------------------------------------------------------
// R-ARCH-02 exhaustive dependency flow tests
// -----------------------------------------------------------------------

#[test]
fn r_arch_02_domain_depends_on_ports_fails() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/domain/Cargo.toml",
        "[package]\nname=\"domain\"\n[dependencies]\nports = { path = \"../ports\" }\n",
    );
    let _ = fs.add("/ws/crates/ports/Cargo.toml", "[package]\nname=\"ports\"");
    let p = project(&[("domain", "crates/domain"), ("ports", "crates/ports")]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].id, "R-ARCH-02");
    assert!(
        r[0].message.contains("domain"),
        "should mention domain: {}",
        r[0].message
    );
    assert!(
        r[0].message.contains("ports"),
        "should mention ports: {}",
        r[0].message
    );
}

#[test]
fn r_arch_02_domain_depends_on_app_fails() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/domain/Cargo.toml",
        "[package]\nname=\"domain\"\n[dependencies]\napp = { path = \"../app\" }\n",
    );
    let _ = fs.add("/ws/crates/app/Cargo.toml", "[package]\nname=\"app\"");
    let p = project(&[("domain", "crates/domain"), ("app", "crates/app")]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].id, "R-ARCH-02");
    assert!(
        r[0].message.contains("domain"),
        "should mention domain: {}",
        r[0].message
    );
    assert!(
        r[0].message.contains("app"),
        "should mention app: {}",
        r[0].message
    );
}

#[test]
fn r_arch_02_ports_depends_on_domain_ok() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/ports/Cargo.toml",
        "[package]\nname=\"ports\"\n[dependencies]\ndomain = { path = \"../domain\" }\n",
    );
    let _ = fs.add("/ws/crates/domain/Cargo.toml", "[package]\nname=\"domain\"");
    let p = project(&[("ports", "crates/ports"), ("domain", "crates/domain")]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert!(
        r.is_empty(),
        "ports -> domain should be allowed, got: {r:?}"
    );
}

#[test]
fn r_arch_02_ports_depends_on_adapters_fails() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/ports/Cargo.toml",
        "[package]\nname=\"ports\"\n[dependencies]\nadapters = { path = \"../adapters\" }\n",
    );
    let _ = fs.add(
        "/ws/crates/adapters/Cargo.toml",
        "[package]\nname=\"adapters\"",
    );
    let p = project(&[("ports", "crates/ports"), ("adapters", "crates/adapters")]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].id, "R-ARCH-02");
    assert!(
        r[0].message.contains("ports"),
        "should mention ports: {}",
        r[0].message
    );
    assert!(
        r[0].message.contains("adapters"),
        "should mention adapters: {}",
        r[0].message
    );
}

#[test]
fn r_arch_02_app_depends_on_ports_ok() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/app/Cargo.toml",
        "[package]\nname=\"app\"\n[dependencies]\nports = { path = \"../ports\" }\n",
    );
    let _ = fs.add("/ws/crates/ports/Cargo.toml", "[package]\nname=\"ports\"");
    let p = project(&[("app", "crates/app"), ("ports", "crates/ports")]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert!(r.is_empty(), "app -> ports should be allowed, got: {r:?}");
}

#[test]
fn r_arch_02_app_depends_on_adapters_fails() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/app/Cargo.toml",
        "[package]\nname=\"app\"\n[dependencies]\nadapters = { path = \"../adapters\" }\n",
    );
    let _ = fs.add(
        "/ws/crates/adapters/Cargo.toml",
        "[package]\nname=\"adapters\"",
    );
    let p = project(&[("app", "crates/app"), ("adapters", "crates/adapters")]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].id, "R-ARCH-02");
    assert!(
        r[0].message.contains("app"),
        "should mention app: {}",
        r[0].message
    );
    assert!(
        r[0].message.contains("adapters"),
        "should mention adapters: {}",
        r[0].message
    );
}

#[test]
fn r_arch_02_adapters_depends_on_everything_ok() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/adapters/Cargo.toml",
        "[package]\nname=\"adapters\"\n[dependencies]\n\
         domain = { path = \"../domain\" }\n\
         ports = { path = \"../ports\" }\n\
         app = { path = \"../app\" }\n",
    );
    let _ = fs.add("/ws/crates/domain/Cargo.toml", "[package]\nname=\"domain\"");
    let _ = fs.add("/ws/crates/ports/Cargo.toml", "[package]\nname=\"ports\"");
    let _ = fs.add("/ws/crates/app/Cargo.toml", "[package]\nname=\"app\"");
    let p = project(&[
        ("adapters", "crates/adapters"),
        ("domain", "crates/domain"),
        ("ports", "crates/ports"),
        ("app", "crates/app"),
    ]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert!(
        r.is_empty(),
        "adapters -> domain+ports+app should be allowed, got: {r:?}"
    );
}

#[test]
fn r_arch_02_multiple_violations_all_reported() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/domain/Cargo.toml",
        "[package]\nname=\"domain\"\n[dependencies]\n\
         ports = { path = \"../ports\" }\n\
         adapters = { path = \"../adapters\" }\n",
    );
    let _ = fs.add("/ws/crates/ports/Cargo.toml", "[package]\nname=\"ports\"");
    let _ = fs.add(
        "/ws/crates/adapters/Cargo.toml",
        "[package]\nname=\"adapters\"",
    );
    let p = project(&[
        ("domain", "crates/domain"),
        ("ports", "crates/ports"),
        ("adapters", "crates/adapters"),
    ]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert_eq!(r.len(), 2, "expected 2 violations, got: {r:?}");
    assert!(r.iter().all(|c| c.id == "R-ARCH-02"));
}

// -----------------------------------------------------------------------
// R-ARCH-03 adversarial tests
// -----------------------------------------------------------------------

#[test]
fn r_arch_03_library_depends_on_other_library_ok() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/packages/a/Cargo.toml",
        "[package]\nname=\"a\"\n[dependencies]\nb = { path = \"../b\" }\n",
    );
    let _ = fs.add("/ws/packages/b/Cargo.toml", "[package]\nname=\"b\"");
    let p = project(&[("a", "packages/a"), ("b", "packages/b")]);
    let mut r = Vec::new();
    check_library_service_boundary(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert!(
        r.is_empty(),
        "packages/a -> packages/b should be allowed, got: {r:?}"
    );
}

#[test]
fn r_arch_03_service_internal_depends_on_package_ok() {
    let mut fs = StubFs::new();
    let _ = fs.add("/ws/apps/x/crates/adapters/Cargo.toml",
        "[package]\nname=\"x-adapters\"\n[dependencies]\ny = { path = \"../../../../packages/y\" }\n");
    let _ = fs.add("/ws/packages/y/Cargo.toml", "[package]\nname=\"y\"");
    let p = project(&[
        ("x-adapters", "apps/x/crates/adapters"),
        ("y", "packages/y"),
    ]);
    let mut r = Vec::new();
    check_library_service_boundary(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert!(
        r.is_empty(),
        "service internal -> package should not trigger R-ARCH-03, got: {r:?}"
    );
}

// -----------------------------------------------------------------------
// Edge cases
// -----------------------------------------------------------------------

#[test]
fn r_arch_02_crate_with_no_layer_skipped() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/utils/Cargo.toml",
        "[package]\nname=\"utils\"\n[dependencies]\ndomain = { path = \"../domain\" }\n",
    );
    let _ = fs.add("/ws/crates/domain/Cargo.toml", "[package]\nname=\"domain\"");
    let p = project(&[("utils", "crates/utils"), ("domain", "crates/domain")]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert!(
        r.is_empty(),
        "crate with no layer should be skipped, got: {r:?}"
    );
}

#[test]
fn r_arch_02_external_dep_not_checked() {
    let mut fs = StubFs::new();
    let _ = fs.add(
        "/ws/crates/domain/Cargo.toml",
        "[package]\nname=\"domain\"\n[dependencies]\nserde = \"1\"\n",
    );
    let p = project(&[("domain", "crates/domain")]);
    let mut r = Vec::new();
    check_dependency_flow(&fs, Path::new("/ws"), &p, &BTreeMap::new(), &mut r);
    assert!(
        r.is_empty(),
        "external (non-path) deps should not trigger R-ARCH-02, got: {r:?}"
    );
}
