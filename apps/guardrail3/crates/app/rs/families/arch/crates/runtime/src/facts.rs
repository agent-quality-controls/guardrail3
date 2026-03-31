use std::collections::BTreeMap;

use guardrail3_app_rs_family_mapper::{
    RsArchRootView, RsArchRoute, RsArchTopologyIssueKindView,
};
use guardrail3_app_rs_placement::{RustArchitectureOwner, RustRootClassification};
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_validation_model::RustValidateFamily;

#[derive(Debug, Clone)]
pub struct ArchRootFacts {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) classification: RustRootClassification,
    pub(crate) app_zone_candidates: Vec<String>,
    pub(crate) package_zone_candidates: Vec<String>,
    pub(crate) owner_families: Vec<RustArchitectureOwner>,
}

#[derive(Debug, Clone)]
pub struct ZoneOverlapFacts {
    pub(crate) app_root_rel: String,
    pub(crate) app_cargo_rel_path: String,
    pub(crate) package_root_rel: String,
    pub(crate) package_cargo_rel_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchInputFailureKind {
    RequiredInput,
    ScopedArchConfig,
}

#[derive(Debug, Clone)]
pub struct GovernedRootFacts {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) owner: RustArchitectureOwner,
    pub(crate) owner_root_rel: String,
    pub(crate) effective_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct IllegalFamilyFileFacts {
    pub(crate) family: RustValidateFamily,
    pub(crate) rel_path: String,
    pub(crate) reason: String,
}

#[derive(Debug, Clone)]
pub enum ArchTopologyIssueKind {
    TopLevelRootMustBeWorkspace,
    LooseTopLevelPackage,
    NestedWorkspace { parent_workspace_rel: String },
    UndeclaredWorkspaceMember { workspace_root_rel: String },
    WorkspaceMemberPathEscapesRoot {
        workspace_root_rel: String,
        member_pattern: String,
    },
    AuxiliaryTopLevelRootMustBeWorkspace,
}

#[derive(Debug, Clone)]
pub struct TopologyIssueFacts {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) kind: ArchTopologyIssueKind,
}

#[derive(Debug, Clone)]
pub struct ArchInputFailureFacts {
    pub(crate) rel_path: String,
    pub(crate) message: String,
    pub(crate) kind: ArchInputFailureKind,
}

#[derive(Debug, Clone, Default)]
pub struct ArchFacts {
    pub(crate) roots: Vec<ArchRootFacts>,
    pub(crate) overlaps: Vec<ZoneOverlapFacts>,
    pub(crate) governed_roots: Vec<GovernedRootFacts>,
    pub(crate) topology_issues: Vec<TopologyIssueFacts>,
    pub(crate) illegal_family_files: Vec<IllegalFamilyFileFacts>,
    pub(crate) input_failures: Vec<ArchInputFailureFacts>,
    pub(crate) misplaced_root_reporting_enabled: bool,
}

#[derive(Debug, Clone)]
struct ConfigResolution {
    arch_enabled: bool,
    global_hexarch_enabled: bool,
    packages_libarch_enabled: bool,
    app_hexarch_enabled: BTreeMap<String, bool>,
    failures: Vec<ArchInputFailureFacts>,
}

pub fn collect(tree: &ProjectTree, route: &RsArchRoute) -> ArchFacts {
    let config = resolve_config(tree);

    let mut input_failures: Vec<_> = route
        .input_failures()
        .iter()
        .map(|failure| ArchInputFailureFacts {
            rel_path: failure.rel_path().to_owned(),
            message: failure.message().to_owned(),
            kind: ArchInputFailureKind::RequiredInput,
        })
        .collect();
    input_failures.extend(config.failures.iter().cloned());

    let roots = route
        .roots()
        .iter()
        .map(root_from_route)
        .collect::<Vec<_>>();
    let overlaps = route
        .overlaps()
        .iter()
        .map(|overlap| ZoneOverlapFacts {
            app_root_rel: overlap.app_root_rel().to_owned(),
            app_cargo_rel_path: overlap.app_cargo_rel_path().to_owned(),
            package_root_rel: overlap.package_root_rel().to_owned(),
            package_cargo_rel_path: overlap.package_cargo_rel_path().to_owned(),
        })
        .collect::<Vec<_>>();

    let governed_roots = roots
        .iter()
        .filter_map(|root| governed_root(root, &config))
        .collect();
    let topology_issues = route
        .topology_issues()
        .iter()
        .map(|issue| TopologyIssueFacts {
            rel_dir: issue.rel_dir().to_owned(),
            cargo_rel_path: issue.cargo_rel_path().to_owned(),
            kind: match issue.kind() {
                RsArchTopologyIssueKindView::TopLevelRootMustBeWorkspace => {
                    ArchTopologyIssueKind::TopLevelRootMustBeWorkspace
                }
                RsArchTopologyIssueKindView::LooseTopLevelPackage => {
                    ArchTopologyIssueKind::LooseTopLevelPackage
                }
                RsArchTopologyIssueKindView::NestedWorkspace {
                    parent_workspace_rel,
                } => ArchTopologyIssueKind::NestedWorkspace {
                    parent_workspace_rel: parent_workspace_rel.clone(),
                },
                RsArchTopologyIssueKindView::UndeclaredWorkspaceMember {
                    workspace_root_rel,
                } => ArchTopologyIssueKind::UndeclaredWorkspaceMember {
                    workspace_root_rel: workspace_root_rel.clone(),
                },
                RsArchTopologyIssueKindView::WorkspaceMemberPathEscapesRoot {
                    workspace_root_rel,
                    member_pattern,
                } => ArchTopologyIssueKind::WorkspaceMemberPathEscapesRoot {
                    workspace_root_rel: workspace_root_rel.clone(),
                    member_pattern: member_pattern.clone(),
                },
                RsArchTopologyIssueKindView::AuxiliaryTopLevelRootMustBeWorkspace => {
                    ArchTopologyIssueKind::AuxiliaryTopLevelRootMustBeWorkspace
                }
            },
        })
        .collect();
    let illegal_family_files = route
        .family_files()
        .iter()
        .filter(|file| !file.placement_is_legal())
        .filter_map(|file| {
            Some(IllegalFamilyFileFacts {
                family: file.family(),
                rel_path: file.rel_path().to_owned(),
                reason: file.placement_reason()?.to_owned(),
            })
        })
        .collect();

    ArchFacts {
        roots,
        overlaps,
        governed_roots,
        topology_issues,
        illegal_family_files,
        input_failures,
        misplaced_root_reporting_enabled: misplaced_root_reporting_enabled(&config),
    }
}

fn resolve_config(tree: &ProjectTree) -> ConfigResolution {
    let Some(content) = tree.file_content("guardrail3.toml") else {
        if tree.file_exists("guardrail3.toml") {
            return ConfigResolution {
                arch_enabled: true,
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
            arch_enabled: true,
            global_hexarch_enabled: true,
            packages_libarch_enabled: true,
            app_hexarch_enabled: BTreeMap::new(),
            failures: Vec::new(),
        };
    };

    match toml::from_str::<GuardrailConfig>(content) {
        Ok(config) => {
            let rust = config.rust();
            let arch_enabled = rust
                .and_then(guardrail3_domain_config::types::RustConfig::checks)
                .and_then(guardrail3_domain_config::types::RustChecksConfig::arch)
                .unwrap_or(true);
            let global_hexarch_enabled = rust
                .and_then(guardrail3_domain_config::types::RustConfig::checks)
                .and_then(guardrail3_domain_config::types::RustChecksConfig::hexarch)
                .unwrap_or(true);
            let global_libarch_enabled = rust
                .and_then(guardrail3_domain_config::types::RustConfig::checks)
                .and_then(guardrail3_domain_config::types::RustChecksConfig::libarch)
                .unwrap_or(true);
            let app_hexarch_enabled = rust
                .and_then(guardrail3_domain_config::types::RustConfig::apps)
                .map(|apps| {
                    apps.iter()
                        .map(|(name, app)| {
                            (
                                name.clone(),
                                app.checks()
                                    .and_then(
                                        guardrail3_domain_config::types::RustChecksConfig::hexarch,
                                    )
                                    .unwrap_or(global_hexarch_enabled),
                            )
                        })
                        .collect::<BTreeMap<_, _>>()
                })
                .unwrap_or_default();
            let packages_libarch_enabled = rust
                .and_then(guardrail3_domain_config::types::RustConfig::packages)
                .and_then(guardrail3_domain_config::types::CrateConfig::checks)
                .and_then(guardrail3_domain_config::types::RustChecksConfig::libarch)
                .unwrap_or(global_libarch_enabled);

            let mut failures = scoped_arch_failures(&config);
            failures.sort_by(|left, right| left.message.cmp(&right.message));

            ConfigResolution {
                arch_enabled,
                global_hexarch_enabled,
                packages_libarch_enabled,
                app_hexarch_enabled,
                failures,
            }
        }
        Err(parse_error) => ConfigResolution {
            arch_enabled: true,
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

fn misplaced_root_reporting_enabled(config: &ConfigResolution) -> bool {
    config.arch_enabled
        && (config.global_hexarch_enabled
            || config
                .app_hexarch_enabled
                .values()
                .copied()
                .any(std::convert::identity)
            || config.packages_libarch_enabled)
}

fn scoped_arch_failures(config: &GuardrailConfig) -> Vec<ArchInputFailureFacts> {
    let mut failures = Vec::new();
    let Some(rust) = config.rust() else {
        return failures;
    };

    if let Some(apps) = rust.apps() {
        for (app_name, app) in apps {
            if app
                .checks()
                .and_then(guardrail3_domain_config::types::RustChecksConfig::arch)
                .is_some()
            {
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
        .packages()
        .and_then(guardrail3_domain_config::types::CrateConfig::checks)
        .and_then(guardrail3_domain_config::types::RustChecksConfig::arch)
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
    if !root.app_zone_candidates().is_empty() {
        owner_families.push(RustArchitectureOwner::Hexarch);
    }
    if !root.package_zone_candidates().is_empty() {
        owner_families.push(RustArchitectureOwner::Libarch);
    }

    ArchRootFacts {
        rel_dir: root.root().rel_dir().to_owned(),
        cargo_rel_path: root.root().cargo_rel_path().to_owned(),
        classification: root.classification(),
        app_zone_candidates: root.app_zone_candidates().to_vec(),
        package_zone_candidates: root.package_zone_candidates().to_vec(),
        owner_families,
    }
}
