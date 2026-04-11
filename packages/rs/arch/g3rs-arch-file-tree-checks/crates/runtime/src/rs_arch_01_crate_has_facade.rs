use g3rs_arch_types::G3RsArchCrateNode;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-ARCH-01";

pub(crate) fn check(node: &G3RsArchCrateNode, results: &mut Vec<G3CheckResult>) {
    if node.cargo_parse_error.is_some() || !node.has_package {
        return;
    }

    if node.has_lib_rs || node.has_main_rs {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "crate has facade entry point".to_owned(),
                format!(
                    "Crate `{}` has a facade entry point ({}).",
                    node.rel_dir,
                    if node.has_lib_rs { "lib.rs" } else { "main.rs" }
                ),
                Some(node.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "crate missing facade entry point".to_owned(),
        format!(
            "Crate `{}` has no lib.rs or main.rs. Every crate must have a facade entry point. Create `src/lib.rs` or `src/main.rs` and re-export only the crate's intended public API.",
            node.rel_dir
        ),
        Some(node.cargo_rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rs_arch_01_crate_has_facade_tests/mod.rs"]
mod tests;
