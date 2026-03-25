use std::fs;

use super::{generate_rust_hook_artifact, generate_rust_owned_artifacts};

#[test]
#[allow(clippy::expect_used)] // reason: test setup assertions
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

    let cfg = toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(
        r#"
version = "0.1"

[profile]
name = "service"

[rust.apps.backend]
type = "library"
"#,
    )
    .expect("parse guardrail config");

    let files = generate_rust_owned_artifacts(tmp.path(), &cfg);
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

#[test]
#[allow(clippy::expect_used)] // reason: test assertions
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
    .expect("parse guardrail config");

    let hook = generate_rust_hook_artifact(Some(&cfg));

    assert!(
        hook.content.contains("guardrail3 rs validate --staged ."),
        "rust hook should validate Rust changes"
    );
    assert!(
        !hook.content.contains("guardrail3 ts validate --staged ."),
        "rust hook should not emit TypeScript validation steps"
    );
}
