use std::fs;

use super::{generate_rust_hook_artifact, generate_rust_owned_artifacts};
use guardrail3_app_rs_generate_assertions::owned_artifacts::{
    assert_contains, assert_not_contains,
};

#[test]
fn per_app_deny_uses_app_effective_profile() {
    let tmp = tempfile::tempdir()
        .unwrap_or_else(|error| panic!("create temporary owned-artifacts test root: {error}"));
    fs::create_dir_all(tmp.path().join("apps/backend"))
        .unwrap_or_else(|error| panic!("create app dir: {error}"));
    fs::write(
        tmp.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"apps/backend\"]\n",
    )
    .unwrap_or_else(|error| panic!("write root Cargo.toml: {error}"));
    fs::write(
        tmp.path().join("apps/backend/Cargo.toml"),
        "[package]\nname = \"backend\"\n",
    )
    .unwrap_or_else(|error| panic!("write backend Cargo.toml: {error}"));

    let cfg = toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(
        r#"
version = "0.1"

[profile]
name = "service"

[rust.apps.backend]
type = "library"
"#,
    )
    .unwrap_or_else(|error| panic!("parse guardrail config: {error}"));

    let files = generate_rust_owned_artifacts(tmp.path(), &cfg);
    let backend_deny = files
        .iter()
        .find(|file| file.path() == "apps/backend/deny.toml")
        .unwrap_or_else(|| panic!("expected backend deny.toml"));

    assert_contains(
        backend_deny.content(),
        "Library IO bans",
        "library deny config should include the library-only section",
    );
    assert_contains(
        backend_deny.content(),
        "{ name = \"tokio\"",
        "library deny config should include the library-only tokio crate ban",
    );
    assert_not_contains(
        backend_deny.content(),
        "[[bans.features]]",
        "library deny config should not emit the tokio feature-ban section",
    );
}

#[test]
fn rust_hook_artifact_stays_rust_only_even_in_mixed_config() {
    let cfg = toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(
        r#"
version = "0.1"

[profile]
name = "service"

[rust]

[typescript]
"#,
    )
    .unwrap_or_else(|error| panic!("parse guardrail config: {error}"));

    let hook = generate_rust_hook_artifact(Some(&cfg));

    assert_contains(
        hook.content(),
        "guardrail3 rs validate --staged .",
        "rust hook should validate Rust changes",
    );
    assert_not_contains(
        hook.content(),
        "guardrail3 ts validate --staged .",
        "rust hook should not emit TypeScript validation steps",
    );
}
