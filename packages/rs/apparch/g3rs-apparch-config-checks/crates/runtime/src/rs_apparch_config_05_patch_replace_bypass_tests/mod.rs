use g3rs_apparch_types::{
    G3RsApparchPatchBypass, G3RsApparchPatchKind, G3RsApparchRustPolicyState,
};
use guardrail3_check_types::G3Severity;
use guardrail3_rs_toml_parser::parse;

fn patch(kind: G3RsApparchPatchKind, key: &str) -> G3RsApparchPatchBypass {
    G3RsApparchPatchBypass {
        cargo_rel_path: "Cargo.toml".to_owned(),
        key: key.to_owned(),
        kind,
        target_cargo_rel_path: "io/outbound/db/Cargo.toml".to_owned(),
        target_rel_dir: "io/outbound/db".to_owned(),
        target_layer: None,
    }
}

#[test]
fn missing_waiver_fires() {
    let mut results = Vec::new();
    crate::rs_apparch_config_05_patch_replace_bypass::check(
        &patch(G3RsApparchPatchKind::Patch, "patch.crates-io.serde"),
        &G3RsApparchRustPolicyState::Missing,
        &mut results,
    );

    let result = results.first().expect("missing waiver result");
    assert_eq!(result.id(), "RS-APPARCH-CONFIG-05");
    assert_eq!(result.severity(), G3Severity::Error);
}

#[test]
fn documented_patch_warns() {
    let mut results = Vec::new();
    crate::rs_apparch_config_05_patch_replace_bypass::check(
        &patch(G3RsApparchPatchKind::Patch, "patch.crates-io.serde"),
        &G3RsApparchRustPolicyState::Parsed {
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
            .expect("parse waiver")
            .waivers,
        },
        &mut results,
    );

    let result = results.first().expect("documented patch result");
    assert_eq!(result.severity(), G3Severity::Warn);
}

#[test]
fn weak_replace_reason_fires() {
    let mut results = Vec::new();
    crate::rs_apparch_config_05_patch_replace_bypass::check(
        &patch(G3RsApparchPatchKind::Replace, "replace.serde:1.0.0"),
        &G3RsApparchRustPolicyState::Parsed {
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
            .expect("parse waiver")
            .waivers,
        },
        &mut results,
    );

    let result = results.first().expect("weak reason result");
    assert_eq!(result.severity(), G3Severity::Error);
    assert!(result.title().contains("reason too weak"));
}

#[test]
fn unreadable_policy_fires() {
    let mut results = Vec::new();
    crate::rs_apparch_config_05_patch_replace_bypass::check(
        &patch(G3RsApparchPatchKind::Patch, "patch.crates-io.serde"),
        &G3RsApparchRustPolicyState::Unreadable {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "permission denied".to_owned(),
        },
        &mut results,
    );

    let result = results.first().expect("unreadable policy result");
    assert_eq!(result.severity(), G3Severity::Error);
    assert!(result.message().contains("permission denied"));
}
