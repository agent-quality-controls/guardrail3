use std::collections::BTreeMap;

use guardrail3_app_rs_family_mapper::{RsArchRootView, RsArchRoute};
use guardrail3_app_rs_placement::{RustArchitectureOwner, RustRootClassification};
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;

#[derive(Debug, Clone)]
pub struct ArchRootFacts {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub classification: RustRootClassification,
    pub app_zone_candidates: Vec<String>,
    pub package_zone_candidates: Vec<String>,
    pub owner_families: Vec<RustArchitectureOwner>,
}

#[derive(Debug, Clone)]
pub struct ZoneOverlapFacts {
    pub app_root_rel: String,
    pub app_cargo_rel_path: String,
    pub package_root_rel: String,
    pub package_cargo_rel_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchInputFailureKind {
    RequiredInput,
    ScopedArchConfig,
}

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
    pub kind: ArchInputFailureKind,
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
    failures: Vec<ArchInputFailureFacts>,
}

pub fn collect(tree: &ProjectTree, route: &RsArchRoute) -> ArchFacts {
    let config = resolve_config(tree);

    let mut input_failures: Vec<_> = route
        .input_failures
        .iter()
        .map(|failure| ArchInputFailureFacts {
            rel_path: failure.rel_path.clone(),
            message: failure.message.clone(),
            kind: ArchInputFailureKind::RequiredInput,
        })
        .collect();
    input_failures.extend(config.failures.iter().cloned());

    let roots = route.roots.iter().map(root_from_route).collect::<Vec<_>>();
    let overlaps = route
        .overlaps
        .iter()
        .map(|overlap| ZoneOverlapFacts {
            app_root_rel: overlap.app_root_rel.clone(),
            app_cargo_rel_path: overlap.app_cargo_rel_path.clone(),
            package_root_rel: overlap.package_root_rel.clone(),
            package_cargo_rel_path: overlap.package_cargo_rel_path.clone(),
        })
        .collect::<Vec<_>>();

    let governed_roots = roots
        .iter()
        .filter_map(|root| governed_root(root, &config))
        .collect();

    ArchFacts {
        roots,
        overlaps,
        governed_roots,
        input_failures,
        misplaced_root_reporting_enabled: route.reporting_enabled,
    }
}

fn resolve_config(tree: &ProjectTree) -> ConfigResolution {
    let Some(content) = tree.file_content("guardrail3.toml") else {
        if tree.file_exists("guardrail3.toml") {
            return ConfigResolution {
                global_hexarch_enabled: true,
                packages_libarch_enabled: true,
                app_hexarch_enabled: BTreeMap::new(),
                failures: vec![ArchInputFailureFacts {
                    rel_path: "guardrail3.toml".to_owned(),
                    message:
                        "Failed to read guardrail3.toml for Rust architecture placement checks."
                            .to_owned(),
                    kind: ArchInputFailureKind::RequiredInput,
                }],
            };
        }

        return ConfigResolution {
            global_hexarch_enabled: true,
            packages_libarch_enabled: true,
            app_hexarch_enabled: BTreeMap::new(),
            failures: Vec::new(),
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

            let mut failures = scoped_arch_failures(&config);
            failures.sort_by(|left, right| left.message.cmp(&right.message));

            ConfigResolution {
                global_hexarch_enabled,
                packages_libarch_enabled,
                app_hexarch_enabled,
                failures,
            }
        }
        Err(parse_error) => ConfigResolution {
            global_hexarch_enabled: true,
            packages_libarch_enabled: true,
            app_hexarch_enabled: BTreeMap::new(),
            failures: vec![ArchInputFailureFacts {
                rel_path: "guardrail3.toml".to_owned(),
                message: format!(
                    "Failed to parse guardrail3.toml for Rust architecture placement checks: {parse_error}"
                ),
                kind: ArchInputFailureKind::RequiredInput,
            }],
        },
    }
}

fn scoped_arch_failures(config: &GuardrailConfig) -> Vec<ArchInputFailureFacts> {
    let mut failures = Vec::new();
    let Some(rust) = config.rust.as_ref() else {
        return failures;
    };

    if let Some(apps) = rust.apps.as_ref() {
        for (app_name, app) in apps {
            if app.checks.as_ref().and_then(|checks| checks.arch).is_some() {
                failures.push(ArchInputFailureFacts {
                    rel_path: "guardrail3.toml".to_owned(),
                    message: format!(
                        "Scoped `arch` config under `[rust.apps.{app_name}.checks]` is forbidden. `arch` is global-only and must be configured only under `[rust.checks]`."
                    ),
                    kind: ArchInputFailureKind::ScopedArchConfig,
                });
            }
        }
    }

    if rust
        .packages
        .as_ref()
        .and_then(|packages| packages.checks.as_ref())
        .and_then(|checks| checks.arch)
        .is_some()
    {
        failures.push(ArchInputFailureFacts {
            rel_path: "guardrail3.toml".to_owned(),
            message: "Scoped `arch` config under `[rust.packages.checks]` is forbidden. `arch` is global-only and must be configured only under `[rust.checks]`.".to_owned(),
            kind: ArchInputFailureKind::ScopedArchConfig,
        });
    }

    failures
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

fn root_from_route(root: &RsArchRootView) -> ArchRootFacts {
    let mut owner_families = Vec::new();
    if !root.app_zone_candidates.is_empty() {
        owner_families.push(RustArchitectureOwner::Hexarch);
    }
    if !root.package_zone_candidates.is_empty() {
        owner_families.push(RustArchitectureOwner::Libarch);
    }

    ArchRootFacts {
        rel_dir: root.root.rel_dir.clone(),
        cargo_rel_path: root.root.cargo_rel_path.clone(),
        classification: root.classification,
        app_zone_candidates: root.app_zone_candidates.clone(),
        package_zone_candidates: root.package_zone_candidates.clone(),
        owner_families,
    }
}
