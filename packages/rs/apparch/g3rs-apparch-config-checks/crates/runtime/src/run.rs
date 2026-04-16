use std::collections::BTreeMap;

use g3rs_apparch_types::G3RsApparchConfigChecksInput;
use g3rs_apparch_types::{G3RsApparchCrate, G3RsApparchLayer};
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsApparchConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let crates_by_path = input
        .crates
        .iter()
        .map(|krate| (krate.cargo_rel_path.clone(), krate))
        .collect::<BTreeMap<_, _>>();

    for krate in &input.crates {
        crate::rs_apparch_config_01_types_dependency_direction::check(
            krate,
            &crates_by_path,
            &input.dependency_edges,
            &mut results,
        );
        crate::rs_apparch_config_02_logic_dependency_direction::check(
            krate,
            &crates_by_path,
            &input.dependency_edges,
            &mut results,
        );
        crate::rs_apparch_config_03_io_outbound_dependency_direction::check(
            krate,
            &crates_by_path,
            &input.dependency_edges,
            &mut results,
        );
        crate::rs_apparch_config_07_dev_dependency_direction::check(
            krate,
            &crates_by_path,
            &input.dependency_edges,
            &mut results,
        );
        crate::rs_apparch_config_08_types_purity::check(
            krate,
            &input.external_dependencies,
            &input.rust_policy,
            &mut results,
        );
        crate::rs_apparch_config_09_logic_purity::check(
            krate,
            &input.external_dependencies,
            &input.rust_policy,
            &mut results,
        );
    }
    for patch in &input.patch_bypasses {
        crate::rs_apparch_config_05_patch_replace_bypass::check(
            patch,
            &input.rust_policy,
            &mut results,
        );
    }
    crate::rs_apparch_config_06_same_layer_cycles::check(
        &input.crates,
        &input.dependency_edges,
        &mut results,
    );

    results
}

pub(crate) fn display_crate(krate: &G3RsApparchCrate) -> &str {
    if krate.crate_name.is_empty() {
        &krate.cargo_rel_path
    } else {
        &krate.crate_name
    }
}

pub(crate) fn layer_label(layer: G3RsApparchLayer) -> &'static str {
    match layer {
        G3RsApparchLayer::Types => "types",
        G3RsApparchLayer::Logic => "logic",
        G3RsApparchLayer::IoInbound => "io/inbound",
        G3RsApparchLayer::IoOutbound => "io/outbound",
    }
}

pub(crate) fn forbidden_runtime_dependency(
    source_layer: G3RsApparchLayer,
    target_layer: G3RsApparchLayer,
) -> bool {
    match source_layer {
        G3RsApparchLayer::Types => matches!(
            target_layer,
            G3RsApparchLayer::Types
                | G3RsApparchLayer::Logic
                | G3RsApparchLayer::IoInbound
                | G3RsApparchLayer::IoOutbound
        ),
        G3RsApparchLayer::Logic => matches!(
            target_layer,
            G3RsApparchLayer::Logic | G3RsApparchLayer::IoInbound | G3RsApparchLayer::IoOutbound
        ),
        G3RsApparchLayer::IoOutbound => matches!(
            target_layer,
            G3RsApparchLayer::Logic | G3RsApparchLayer::IoInbound | G3RsApparchLayer::IoOutbound
        ),
        G3RsApparchLayer::IoInbound => false,
    }
}

fn package_member(rel_dir: &str) -> Option<(&str, &str)> {
    rel_dir.split_once("/crates/")
}

pub(crate) fn is_package_internal_assertions_to_runtime_edge(
    source: &G3RsApparchCrate,
    target: &G3RsApparchCrate,
) -> bool {
    let Some((source_package, source_member)) = package_member(&source.rel_dir) else {
        return false;
    };
    let Some((target_package, target_member)) = package_member(&target.rel_dir) else {
        return false;
    };

    source_package == target_package && source_member == "assertions" && target_member == "runtime"
}

pub(crate) fn is_package_internal_runtime_to_assertions_dev_edge(
    source: &G3RsApparchCrate,
    target: &G3RsApparchCrate,
) -> bool {
    let Some((source_package, source_member)) = package_member(&source.rel_dir) else {
        return false;
    };
    let Some((target_package, target_member)) = package_member(&target.rel_dir) else {
        return false;
    };

    source_package == target_package && source_member == "runtime" && target_member == "assertions"
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
