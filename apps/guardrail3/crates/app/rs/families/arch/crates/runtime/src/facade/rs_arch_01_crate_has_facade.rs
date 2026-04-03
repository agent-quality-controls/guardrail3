use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::CrateNode;

const ID: &str = "RS-ARCH-01";

pub(crate) fn check(node: &CrateNode, results: &mut Vec<CheckResult>) {
    if node.cargo_parse_error.is_some() {
        return;
    }
    if !node.has_package {
        return;
    }

    if node.has_lib_rs || node.has_main_rs {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "crate has facade entry point".to_owned(),
                format!(
                    "Crate `{}` has a facade entry point ({}).",
                    node.rel_dir,
                    if node.has_lib_rs { "lib.rs" } else { "main.rs" }
                ),
                Some(node.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "crate missing facade entry point".to_owned(),
        format!(
            "Crate `{}` has no lib.rs or main.rs. Every crate must have a facade entry point. Create `src/lib.rs` or `src/main.rs` and re-export only the crate's intended public API.",
            node.rel_dir
        ),
        Some(node.cargo_rel_path.clone()),
        None,
        false,
    ));
}
