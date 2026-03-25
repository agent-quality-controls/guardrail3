pub mod domain {
    pub use guardrail3_domain_config as config;
    pub use guardrail3_domain_report as report;
}

pub mod app {
    pub use guardrail3_app_core as core;

    pub mod rs {
        pub mod checks {
            pub mod hooks {
                pub use guardrail3_app_rs_family_hooks_rs as rs;
                pub use guardrail3_app_rs_family_hooks_shared as shared;
            }

            pub mod rs {
                pub use guardrail3_app_rs_family_arch as arch;
                pub use guardrail3_app_rs_family_cargo as cargo;
                pub use guardrail3_app_rs_family_clippy as clippy;
                pub use guardrail3_app_rs_family_code as code;
                pub use guardrail3_app_rs_family_deny as deny;
                pub use guardrail3_app_rs_family_deps as deps;
                pub use guardrail3_app_rs_family_fmt as fmt;
                pub use guardrail3_app_rs_family_garde as garde;
                pub use guardrail3_app_rs_family_hexarch as hexarch;
                pub use guardrail3_app_rs_family_release as release;
                pub use guardrail3_app_rs_family_test as test;
                pub use guardrail3_app_rs_family_toolchain as toolchain;
            }
        }
    }
}

use std::collections::BTreeSet;
use std::path::Path;

use crate::domain::config::types::{GuardrailConfig, RustChecksConfig};
use crate::domain::report::{Report, Section, rust_validate_family_section_name};
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_outbound_traits::{FileSystem, ToolChecker};
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

pub fn run(
    fs: &dyn FileSystem,
    path: &Path,
    scoped_files: Option<&BTreeSet<String>>,
    requested_families: &[RustValidateFamily],
    thorough: bool,
    tc: &dyn ToolChecker,
) -> Result<Report, String> {
    let tree = crate::app::core::project_walker::walk_project(fs, path);
    let config = load_config(&tree)?;
    let selected = resolve_selected_families(&tree, config.as_ref(), requested_families);

    let mut report = Report::new(path.display().to_string(), vec!["Rust".to_owned()]);

    for family in selected.iter() {
        let results = match family {
            RustValidateFamily::Arch => crate::app::rs::checks::rs::arch::check(&tree),
            RustValidateFamily::Fmt => crate::app::rs::checks::rs::fmt::check(&tree),
            RustValidateFamily::Toolchain => crate::app::rs::checks::rs::toolchain::check(&tree),
            RustValidateFamily::Clippy => crate::app::rs::checks::rs::clippy::check(&tree),
            RustValidateFamily::Deny => crate::app::rs::checks::rs::deny::check(&tree),
            RustValidateFamily::Cargo => crate::app::rs::checks::rs::cargo::check(&tree),
            RustValidateFamily::Code => {
                crate::app::rs::checks::rs::code::check(&tree, scoped_files)
            }
            RustValidateFamily::Hexarch => crate::app::rs::checks::rs::hexarch::check(&tree),
            RustValidateFamily::Deps => crate::app::rs::checks::rs::deps::check(&tree, tc),
            RustValidateFamily::Garde => {
                crate::app::rs::checks::rs::garde::check(&tree, scoped_files)
            }
            RustValidateFamily::Test => {
                crate::app::rs::checks::rs::test::check(&tree, tc, scoped_files)
            }
            RustValidateFamily::Release => {
                crate::app::rs::checks::rs::release::check(&tree, tc, thorough)
            }
            RustValidateFamily::HooksShared => {
                crate::app::rs::checks::hooks::shared::check(fs, path, &tree, tc)
            }
            RustValidateFamily::HooksRs => crate::app::rs::checks::hooks::rs::check(&tree, tc),
        };
        report.add_section(Section {
            name: rust_validate_family_section_name(family).to_owned(),
            results,
        });
    }

    Ok(report)
}

fn load_config(tree: &ProjectTree) -> Result<Option<GuardrailConfig>, String> {
    let Some(content) = tree.file_content("guardrail3.toml") else {
        return Ok(None);
    };
    toml::from_str::<GuardrailConfig>(content)
        .map(Some)
        .map_err(|error| format!("Error parsing guardrail3.toml: {error}"))
}

fn resolve_selected_families(
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

fn effective_family_flag(
    checks: Option<&RustChecksConfig>,
    family: RustValidateFamily,
    global: bool,
) -> bool {
    checks
        .and_then(|value| value.family_enabled(family))
        .unwrap_or(global)
}

fn has_unscoped_rust_root(
    tree: &ProjectTree,
    rust: &crate::domain::config::types::RustConfig,
) -> bool {
    if tree.file_exists("Cargo.toml") && rust.apps.as_ref().is_none_or(|apps| apps.is_empty()) {
        return true;
    }

    rust.packages.is_none() && tree.dir_exists("packages")
}
