use g3rs_arch_types::types::{G3RsArchFacadeSurface, G3RsArchSourceCrate};
use guardrail3_check_types::G3CheckResult;

pub(super) fn source_crate(rel_dir: &str) -> G3RsArchSourceCrate {
    G3RsArchSourceCrate {
        rel_dir: rel_dir.to_owned(),
        lib_rs_rel: Some(format!("{rel_dir}/src/lib.rs")),
    }
}

pub(super) fn run_rule(
    node: &G3RsArchSourceCrate,
    surface: &G3RsArchFacadeSurface,
) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::feature_gated_exports::check(node, Some(surface), &mut results);
    results
}
