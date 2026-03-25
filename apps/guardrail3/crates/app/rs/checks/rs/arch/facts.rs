use std::collections::BTreeMap;

use crate::domain::config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;

use super::super::rust_root_placement::{
    self, RustArchitectureOwner, RustRootPlacementRootFacts, RustZoneOverlapFacts,
};

pub type ArchRootFacts = RustRootPlacementRootFacts;
pub type ZoneOverlapFacts = RustZoneOverlapFacts;

#[derive(Debug, Clone)]
pub struct GovernedRootFacts {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub owner: RustArchitectureOwner,
    pub owner_root_rel: String,
    pub effective_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ArchInputFailureFacts {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct ArchFacts {
    pub roots: Vec<ArchRootFacts>,
    pub overlaps: Vec<ZoneOverlapFacts>,
    pub governed_roots: Vec<GovernedRootFacts>,
    pub input_failures: Vec<ArchInputFailureFacts>,
    pub misplaced_root_reporting_enabled: bool,
}

#[derive(Debug, Clone)]
struct ConfigResolution {
    global_hexarch_enabled: bool,
    packages_libarch_enabled: bool,
    app_hexarch_enabled: BTreeMap<String, bool>,
    misplaced_root_reporting_enabled: bool,
    parse_error: Option<String>,
}

pub fn collect(tree: &ProjectTree) -> ArchFacts {
    let placement = rust_root_placement::collect(tree);
    let config = resolve_config(tree);

    let mut input_failures: Vec<_> = placement
        .input_failures
        .into_iter()
        .map(|failure| ArchInputFailureFacts {
            rel_path: failure.rel_path,
            message: failure.message,
        })
        .collect();

    if let Some(parse_error) = &config.parse_error {
        input_failures.push(ArchInputFailureFacts {
            rel_path: "guardrail3.toml".to_owned(),
            message: parse_error.clone(),
        });
    }

    let governed_roots = placement
        .roots
        .iter()
        .filter_map(|root| governed_root(root, &config))
        .collect();

    ArchFacts {
        roots: placement.roots,
        overlaps: placement.overlaps,
        governed_roots,
        input_failures,
        misplaced_root_reporting_enabled: config.misplaced_root_reporting_enabled,
    }
}

fn resolve_config(tree: &ProjectTree) -> ConfigResolution {
    let Some(content) = tree.file_content("guardrail3.toml") else {
        return ConfigResolution {
            global_hexarch_enabled: true,
            packages_libarch_enabled: true,
            app_hexarch_enabled: BTreeMap::new(),
            misplaced_root_reporting_enabled: true,
            parse_error: None,
        };
    };

    match toml::from_str::<GuardrailConfig>(content) {
        Ok(config) => {
            let rust = config.rust.as_ref();
            let global_hexarch_enabled = rust
                .and_then(|value| value.checks.as_ref())
                .and_then(|checks| checks.hexarch)
                .unwrap_or(true);
            let global_libarch_enabled = rust
                .and_then(|value| value.checks.as_ref())
                .and_then(|checks| checks.libarch)
                .unwrap_or(true);
            let app_hexarch_enabled = rust
                .and_then(|value| value.apps.as_ref())
                .map(|apps| {
                    apps.iter()
                        .map(|(name, app)| {
                            (
                                name.clone(),
                                app.checks
                                    .as_ref()
                                    .and_then(|checks| checks.hexarch)
                                    .unwrap_or(global_hexarch_enabled),
                            )
                        })
                        .collect::<BTreeMap<_, _>>()
                })
                .unwrap_or_default();
            let packages_libarch_enabled = rust
                .and_then(|value| value.packages.as_ref())
                .and_then(|packages| packages.checks.as_ref())
                .and_then(|checks| checks.libarch)
                .unwrap_or(global_libarch_enabled);
            let misplaced_root_reporting_enabled = global_hexarch_enabled
                || global_libarch_enabled
                || app_hexarch_enabled.values().copied().any(|enabled| enabled)
                || packages_libarch_enabled;

            ConfigResolution {
                global_hexarch_enabled,
                packages_libarch_enabled,
                app_hexarch_enabled,
                misplaced_root_reporting_enabled,
                parse_error: None,
            }
        }
        Err(parse_error) => ConfigResolution {
            global_hexarch_enabled: true,
            packages_libarch_enabled: true,
            app_hexarch_enabled: BTreeMap::new(),
            misplaced_root_reporting_enabled: true,
            parse_error: Some(format!(
                "Failed to parse guardrail3.toml for Rust architecture placement checks: {parse_error}"
            )),
        },
    }
}

fn governed_root(root: &ArchRootFacts, config: &ConfigResolution) -> Option<GovernedRootFacts> {
    match root.owner_families.as_slice() {
        [RustArchitectureOwner::Hexarch] => {
            let owner_root_rel = root.app_zone_candidates.first()?.clone();
            let owner_name = owner_root_rel.rsplit('/').next()?;
            let effective_enabled = config
                .app_hexarch_enabled
                .get(owner_name)
                .copied()
                .unwrap_or(config.global_hexarch_enabled);
            Some(GovernedRootFacts {
                rel_dir: root.rel_dir.clone(),
                cargo_rel_path: root.cargo_rel_path.clone(),
                owner: RustArchitectureOwner::Hexarch,
                owner_root_rel,
                effective_enabled,
            })
        }
        [RustArchitectureOwner::Libarch] => {
            let owner_root_rel = root.package_zone_candidates.first()?.clone();
            let _owner_name = owner_root_rel.rsplit('/').next()?;
            Some(GovernedRootFacts {
                rel_dir: root.rel_dir.clone(),
                cargo_rel_path: root.cargo_rel_path.clone(),
                owner: RustArchitectureOwner::Libarch,
                owner_root_rel,
                effective_enabled: config.packages_libarch_enabled,
            })
        }
        _ => None,
    }
}
