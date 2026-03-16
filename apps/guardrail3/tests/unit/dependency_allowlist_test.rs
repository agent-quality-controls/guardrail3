use std::path::{Path, PathBuf};

use guardrail3::app::rs::validate::dependency_allowlist::*;
use guardrail3::domain::config::types::CrateConfig;
use guardrail3::domain::report::Severity;
use guardrail3::ports::outbound::FileSystem;

/// Stub filesystem that returns a predefined Cargo.toml content.
struct StubFs {
    content: String,
}

impl FileSystem for StubFs {
    fn read_file(&self, _path: &Path) -> Option<String> {
        Some(self.content.clone())
    }

    #[allow(clippy::unnecessary_wraps)] // reason: trait requires Result return type
    fn read_file_err(&self, _path: &Path) -> Result<String, std::io::Error> {
        Ok(self.content.clone())
    }

    fn list_dir(&self, _path: &Path) -> Vec<std::fs::DirEntry> {
        Vec::new()
    }

    fn metadata(&self, _path: &Path) -> Option<std::fs::Metadata> {
        None
    }
}

#[test]
#[allow(clippy::indexing_slicing, clippy::expect_used)] // reason: test assertions
fn r_deps_01_unauthorized_dep_flagged() {
    let cargo_toml = r#"
[package]
name = "my-domain"

[dependencies]
serde = "1"
tokio = "1"
thiserror = "1"
"#;
    let fs = StubFs {
        content: cargo_toml.to_owned(),
    };
    let mut results = Vec::new();
    let allowed = vec!["serde".to_owned(), "thiserror".to_owned()];

    check_dependency_allowlist(
        &PathBuf::from("crates/my-domain/Cargo.toml"),
        "my-domain",
        &allowed,
        &fs,
        &mut results,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-DEPS-01");
    assert_eq!(results[0].severity, Severity::Error);
    assert!(results[0].message.contains("tokio"));
}

#[test]
#[allow(clippy::indexing_slicing, clippy::expect_used)] // reason: test assertions
fn r_deps_01_allowed_dep_passes() {
    let cargo_toml = r#"
[package]
name = "my-domain"

[dependencies]
serde = "1"
thiserror = "1"
"#;
    let fs = StubFs {
        content: cargo_toml.to_owned(),
    };
    let mut results = Vec::new();
    let allowed = vec!["serde".to_owned(), "thiserror".to_owned()];

    check_dependency_allowlist(
        &PathBuf::from("crates/my-domain/Cargo.toml"),
        "my-domain",
        &allowed,
        &fs,
        &mut results,
    );

    assert!(results.is_empty());
}

#[test]
#[allow(clippy::indexing_slicing, clippy::expect_used)] // reason: test assertions
fn r_deps_01_dev_deps_not_checked() {
    let cargo_toml = r#"
[package]
name = "my-domain"

[dependencies]
serde = "1"

[dev-dependencies]
tokio = { version = "1", features = ["test-util"] }
"#;
    let fs = StubFs {
        content: cargo_toml.to_owned(),
    };
    let mut results = Vec::new();
    let allowed = vec!["serde".to_owned()];

    check_dependency_allowlist(
        &PathBuf::from("crates/my-domain/Cargo.toml"),
        "my-domain",
        &allowed,
        &fs,
        &mut results,
    );

    assert!(results.is_empty());
}

#[test]
#[allow(clippy::indexing_slicing, clippy::expect_used)] // reason: test assertions
fn r_deps_02_library_without_allowlist_warns() {
    let cfg = CrateConfig {
        layer: None,
        profile: Some("library".to_owned()),
        allowed_deps: None,
    };
    let mut results = Vec::new();

    check_library_has_allowlist("my-domain", &cfg, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-DEPS-02");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
#[allow(clippy::indexing_slicing, clippy::expect_used)] // reason: test assertions
fn r_deps_02_service_without_allowlist_ok() {
    let cfg = CrateConfig {
        layer: None,
        profile: Some("service".to_owned()),
        allowed_deps: None,
    };
    let mut results = Vec::new();

    check_library_has_allowlist("my-api", &cfg, &mut results);

    assert!(results.is_empty());
}

#[test]
#[allow(clippy::indexing_slicing, clippy::expect_used)] // reason: test assertions
fn r_deps_01_skips_workspace_path_deps() {
    let cargo_toml = r#"
[package]
name = "my-api"

[dependencies]
serde = "1"
my-domain = { path = "../my-domain" }
my-types = { workspace = true }
"#;
    let fs = StubFs {
        content: cargo_toml.to_owned(),
    };
    let mut results = Vec::new();
    let allowed = vec!["serde".to_owned()];

    check_dependency_allowlist(
        &PathBuf::from("crates/my-api/Cargo.toml"),
        "my-api",
        &allowed,
        &fs,
        &mut results,
    );

    // my-domain (path dep) and my-types (workspace dep) should be skipped
    assert!(results.is_empty());
}
