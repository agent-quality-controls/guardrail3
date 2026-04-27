use g3rs_arch_types::types::{G3RsArchConfigCrate, G3RsArchRustPolicyState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-ARCH-CONFIG-07";
const MAX_DEPENDENCIES: usize = 12;
const SELECTOR: &str = "dependency-count";

pub(crate) fn check(
    node: &G3RsArchConfigCrate,
    rust_policy: &G3RsArchRustPolicyState,
    results: &mut Vec<G3CheckResult>,
) {
    if node.production_dependency_count <= MAX_DEPENDENCIES {
        return;
    }
    if has_waiver(node, rust_policy) {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "crate has too many production dependencies, must split".to_owned(),
        format!(
            "Crate `{}` has {} production dependencies in `[dependencies]` and `[build-dependencies]` (max {}). Move related code behind smaller crate boundaries so the runtime crate carries fewer direct production links.",
            node.rel_dir, node.production_dependency_count, MAX_DEPENDENCIES
        ),
        Some(node.cargo_rel_path.clone()),
        None,
    ));
}

fn has_waiver(node: &G3RsArchConfigCrate, rust_policy: &G3RsArchRustPolicyState) -> bool {
    let G3RsArchRustPolicyState::Parsed { waivers, .. } = rust_policy else {
        return false;
    };
    waivers.iter().any(|waiver| {
        waiver.rule == ID && waiver.file == node.cargo_rel_path && waiver.selector == SELECTOR
    })
}

#[cfg(test)]
#[path = "rs_arch_07b_dependency_count_split_tests/mod.rs"]
// reason: keep rule tests in the owned x_tests sidecar directory.
mod rs_arch_07b_dependency_count_split_tests;
