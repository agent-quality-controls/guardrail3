use g3rs_apparch_types::{
    G3RsApparchPatchBypass, G3RsApparchPatchBypassChecksInput, G3RsApparchPatchKind,
    G3RsApparchRustPolicyState,
};
use guardrail3_check_types::G3CheckResult;

pub(super) fn patch(kind: G3RsApparchPatchKind, key: &str) -> G3RsApparchPatchBypass {
    G3RsApparchPatchBypass {
        cargo_rel_path: "Cargo.toml".to_owned(),
        key: key.to_owned(),
        kind,
        target_cargo_rel_path: "io/outbound/db/Cargo.toml".to_owned(),
        target_rel_dir: "io/outbound/db".to_owned(),
        target_layer: None,
    }
}

pub(super) fn run_rule(
    patch: &G3RsApparchPatchBypass,
    rust_policy: &G3RsApparchRustPolicyState,
) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_apparch_config_05_patch_replace_bypass::check(
        &G3RsApparchPatchBypassChecksInput {
            patch: patch.clone(),
            rust_policy: rust_policy.clone(),
        },
        &mut results,
    );
    results
}
