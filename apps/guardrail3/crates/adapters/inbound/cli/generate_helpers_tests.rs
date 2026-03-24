use std::fs;

use super::{LocalOverrides, generate_rust_files};

fn empty_overrides() -> LocalOverrides {
    LocalOverrides {
        clippy_methods: String::new(),
        clippy_types: String::new(),
        deny_bans: String::new(),
        deny_skip: String::new(),
        deny_feature_bans: String::new(),
    }
}

#[test]
fn per_app_deny_uses_app_effective_profile() {
    let tmp = tempfile::tempdir().expect("create tempdir");
    fs::create_dir_all(tmp.path().join("apps/backend")).expect("create app dir");
    fs::write(
        tmp.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"apps/backend\"]\n",
    )
    .expect("write root Cargo.toml");
    fs::write(
        tmp.path().join("apps/backend/Cargo.toml"),
        "[package]\nname = \"backend\"\n",
    )
    .expect("write backend Cargo.toml");

    let cfg = toml::from_str::<crate::domain::config::types::GuardrailConfig>(
        r#"
version = "0.1"

[profile]
name = "service"

[rust.apps.backend]
type = "library"
"#,
    )
    .expect("parse guardrail config");

    let files = generate_rust_files(tmp.path(), &cfg, "service", &empty_overrides());
    let backend_deny = files
        .iter()
        .find(|file| file.path == "apps/backend/deny.toml")
        .expect("expected backend deny.toml");

    assert!(
        backend_deny.content.contains("Library IO bans")
            && backend_deny.content.contains("{ name = \"tokio\""),
        "library deny config should include the library-only tokio crate ban: {}",
        backend_deny.content
    );
    assert!(
        !backend_deny.content.contains("[[bans.features]]"),
        "library deny config should not emit tokio feature-ban section: {}",
        backend_deny.content
    );
}
