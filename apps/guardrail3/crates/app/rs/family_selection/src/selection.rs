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
        RustFamilySelection::new(
            requested_families
                .iter()
                .copied()
                .filter(|family| config_enabled.contains(family))
                .collect(),
        )
    };

    if selection.contains(RustValidateFamily::HooksRs) {
        selection.insert(RustValidateFamily::HooksShared);
    }

    selection
}

fn family_enabled_for_runtime(
    family: RustValidateFamily,
    tree: &ProjectTree,
    config: Option<&GuardrailConfig>,
) -> bool {
    let Some(rust) = config.and_then(|cfg| cfg.rust.as_ref()) else {
        return true;
    };

    let global = rust
        .checks
        .as_ref()
        .and_then(|checks| checks.family_enabled(family))
        .unwrap_or(true);

    if family == RustValidateFamily::Arch {
        return global || scoped_arch_config_present(rust);
    }

    if family_uses_global_only(family) {
        return global;
    }

    let app_count = rust
        .apps
        .as_ref()
        .map_or(0, std::collections::BTreeMap::len);
    let has_packages_scope = rust.packages.is_some();

    if app_count == 0 && !has_packages_scope {
        return global;
    }

    let app_enabled = rust.apps.as_ref().is_some_and(|apps| {
        apps.values()
            .any(|cfg| effective_family_flag(cfg.checks.as_ref(), family, global))
    });
    let packages_enabled = rust
        .packages
        .as_ref()
        .is_some_and(|cfg| effective_family_flag(cfg.checks.as_ref(), family, global));

    if family == RustValidateFamily::Hexarch {
        let discovered_apps = tree.dir_exists("apps");
        if app_count > 0 || discovered_apps {
            return app_enabled || (global && app_count == 0);
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
            | RustValidateFamily::Toolchain
            | RustValidateFamily::HooksShared
            | RustValidateFamily::HooksRs
    )
}

fn scoped_arch_config_present(rust: &RustConfig) -> bool {
    rust.apps.as_ref().is_some_and(|apps| {
        apps.values()
            .any(|cfg| cfg.checks.as_ref().and_then(|checks| checks.arch).is_some())
    }) || rust
        .packages
        .as_ref()
        .and_then(|packages| packages.checks.as_ref())
        .and_then(|checks| checks.arch)
        .is_some()
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
    if tree.file_exists("Cargo.toml") && rust.apps.as_ref().is_none_or(|apps| apps.is_empty()) {
        return true;
    }

    rust.packages.is_none() && tree.dir_exists("packages")
}
