use std::collections::BTreeSet;

use guardrail3_domain_config::types::{GuardrailConfig, RustChecksConfig, RustConfig};
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

#[must_use]
pub fn resolve(
    tree: &ProjectTree,
    config: Option<&GuardrailConfig>,
    requested_families: &[RustValidateFamily],
) -> RustFamilySelection {
    let config_enabled: BTreeSet<_> = RustValidateFamily::all()
        .iter()
        .copied()
        .filter(|family| family_enabled_for_runtime(*family, tree, config))
        .collect();

    let mut selection = if requested_families.is_empty() {
        RustFamilySelection::new(config_enabled)
    } else {
        RustFamilySelection::new(requested_families.iter().copied().collect())
    };

    selection.insert(RustValidateFamily::Arch);

    if selection.contains(RustValidateFamily::HooksRs) {
        selection.insert(RustValidateFamily::HooksShared);
    }

    selection
}

#[cfg(test)]
pub(crate) fn resolve_for_tests(
    tree: &ProjectTree,
    config: Option<&GuardrailConfig>,
    requested_families: &[RustValidateFamily],
) -> RustFamilySelection {
    resolve(tree, config, requested_families)
}

#[must_use]
pub fn minimal_tree_for_tests() -> ProjectTree {
    let mut structure = std::collections::BTreeMap::new();
    let _ = structure.insert(
        String::new(),
        guardrail3_domain_project_tree::DirEntry::new(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
    );

    ProjectTree::new(
        std::path::PathBuf::from("/tmp/guardrail3-family-selection-tests"),
        structure,
        std::collections::BTreeMap::new(),
    )
}

#[must_use]
pub fn config_for_explicit_arch_request_for_tests() -> GuardrailConfig {
    GuardrailConfig::new(
        None,
        None,
        Some(RustConfig::new(
            None,
            None,
            None,
            None,
            Some(RustChecksConfig::new(
                Some(false),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(true),
                Some(true),
                None,
                None,
                None,
                None,
                None,
                None,
            )),
        )),
        None,
        None,
    )
}

#[must_use]
pub fn config_for_enabled_family_filtering_for_tests() -> GuardrailConfig {
    GuardrailConfig::new(
        None,
        None,
        Some(RustConfig::new(
            None,
            None,
            None,
            None,
            Some(RustChecksConfig::new(
                Some(false),
                Some(true),
                None,
                None,
                None,
                None,
                None,
                Some(false),
                Some(false),
                None,
                None,
                None,
                None,
                None,
                None,
            )),
        )),
        None,
        None,
    )
}

#[must_use]
pub fn explicit_arch_request_for_tests() -> Vec<RustValidateFamily> {
    vec![RustValidateFamily::Arch]
}

fn family_enabled_for_runtime(
    family: RustValidateFamily,
    tree: &ProjectTree,
    config: Option<&GuardrailConfig>,
) -> bool {
    if family == RustValidateFamily::Arch {
        return true;
    }

    let Some(rust) = config.and_then(GuardrailConfig::rust) else {
        return true;
    };

    let global = rust
        .checks()
        .and_then(|checks| checks.family_enabled(family))
        .unwrap_or(true);

    if family_uses_global_only(family) {
        return global;
    }

    let app_count = rust.apps().map_or(0, std::collections::BTreeMap::len);
    let has_packages_scope = rust.packages().is_some();

    if app_count == 0 && !has_packages_scope {
        return global;
    }

    let app_enabled = rust.apps().is_some_and(|apps| {
        apps.values()
            .any(|cfg| effective_family_flag(cfg.checks(), family, global))
    });
    let packages_enabled = rust
        .packages()
        .is_some_and(|cfg| effective_family_flag(cfg.checks(), family, global));

    if family == RustValidateFamily::Hexarch {
        let discovered_apps = tree.dir_exists("apps");
        if app_count > 0 || discovered_apps {
            return app_enabled || (global && app_count == 0);
        }
        return global;
    }

    if family == RustValidateFamily::Libarch {
        let discovered_packages = tree.dir_exists("packages");
        if has_packages_scope || discovered_packages {
            return packages_enabled || (global && !has_packages_scope);
        }
        return global;
    }

    app_enabled || packages_enabled || (global && has_unscoped_rust_root(tree, rust))
}

fn family_uses_global_only(family: RustValidateFamily) -> bool {
    matches!(
        family,
        RustValidateFamily::Arch
            | RustValidateFamily::Fmt
            | RustValidateFamily::Code
            | RustValidateFamily::Test
            | RustValidateFamily::HooksShared
            | RustValidateFamily::HooksRs
    )
}

fn effective_family_flag(
    checks: Option<&RustChecksConfig>,
    family: RustValidateFamily,
    global: bool,
) -> bool {
    checks
        .and_then(|value| value.family_enabled(family))
        .unwrap_or(global)
}

fn has_unscoped_rust_root(tree: &ProjectTree, rust: &RustConfig) -> bool {
    if tree.file_exists("Cargo.toml") && rust.apps().is_none_or(|apps| apps.is_empty()) {
        return true;
    }

    rust.packages().is_none() && tree.dir_exists("packages")
}

#[cfg(test)]
#[path = "selection_tests/mod.rs"] // reason: test-only sidecar module wiring
mod selection_tests;
