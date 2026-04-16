use g3rs_apparch_config_checks_assertions::rs_apparch_config_05_patch_replace_bypass as assertions;
use g3rs_apparch_types::{G3RsApparchPatchKind, G3RsApparchRustPolicyState};
use guardrail3_rs_toml_parser::parse;

use super::helpers::{patch, run_rule};

#[test]
fn missing_waiver_fires() {
    let results = run_rule(
        &patch(G3RsApparchPatchKind::Patch, "patch.crates-io.serde"),
        &G3RsApparchRustPolicyState::Missing,
    );

    assertions::assert_missing_waiver(&results);
}

#[test]
fn documented_patch_warns() {
    let policy = G3RsApparchRustPolicyState::Parsed {
        rel_path: "guardrail3-rs.toml".to_owned(),
        profile: None,
        allowed_deps: Vec::new(),
        waivers: parse(
            r#"
            [[waivers]]
            rule = "RS-APPARCH-CONFIG-05"
            file = "Cargo.toml"
            selector = "patch.crates-io.serde"
            reason = "Temporary internal patch while splitting postgres adapter from shared serde glue."
            "#,
        )
        .expect("waiver fixture should parse")
        .waivers,
    };
    let results = run_rule(
        &patch(G3RsApparchPatchKind::Patch, "patch.crates-io.serde"),
        &policy,
    );

    assertions::assert_documented_patch(&results);
}

#[test]
fn weak_replace_reason_fires() {
    let policy = G3RsApparchRustPolicyState::Parsed {
        rel_path: "guardrail3-rs.toml".to_owned(),
        profile: None,
        allowed_deps: Vec::new(),
        waivers: parse(
            r#"
            [[waivers]]
            rule = "RS-APPARCH-CONFIG-05"
            file = "Cargo.toml"
            selector = "replace.serde:1.0.0"
            reason = "needed"
            "#,
        )
        .expect("weak waiver fixture should parse")
        .waivers,
    };
    let results = run_rule(
        &patch(G3RsApparchPatchKind::Replace, "replace.serde:1.0.0"),
        &policy,
    );

    assertions::assert_weak_reason(&results);
}

#[test]
fn unreadable_policy_fires() {
    let results = run_rule(
        &patch(G3RsApparchPatchKind::Patch, "patch.crates-io.serde"),
        &G3RsApparchRustPolicyState::Unreadable {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "permission denied".to_owned(),
        },
    );

    assertions::assert_policy_error_contains(&results, "permission denied");
}
