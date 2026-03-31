use std::collections::BTreeMap;

use guardrail3_app_rs_family_mapper::{RsTopologyIssueKindView, RsTopologyRootView, RsTopologyRoute};
use guardrail3_app_rs_placement::{RustArchitectureOwner, RustRootClassification};
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;
use guardrail3_validation_model::RustValidateFamily;

#[derive(Debug, Clone)]
pub struct TopologyRootFacts {
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
pub enum TopologyInputFailureKind {
    RequiredInput,
    ScopedTopologyConfig,
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
pub enum TopologyIssueKind {
    TopLevelRootMustBeWorkspace,
    LooseTopLevelPackage,
    NestedWorkspace {
        parent_workspace_rel: String,
    },
    UndeclaredWorkspaceMember {
        workspace_root_rel: String,
    },
    ExtraWorkspaceMember {
        workspace_root_rel: String,
        member_pattern: String,
    },
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
    pub(crate) kind: TopologyIssueKind,
}

#[derive(Debug, Clone)]
pub struct TopologyInputFailureFacts {
    pub(crate) rel_path: String,
    pub(crate) message: String,
    pub(crate) kind: TopologyInputFailureKind,
}

#[derive(Debug, Clone, Default)]
pub struct TopologyFacts {
    pub(crate) roots: Vec<TopologyRootFacts>,
    pub(crate) overlaps: Vec<ZoneOverlapFacts>,
    pub(crate) governed_roots: Vec<GovernedRootFacts>,
    pub(crate) topology_issues: Vec<TopologyIssueFacts>,
    pub(crate) illegal_family_files: Vec<IllegalFamilyFileFacts>,
    pub(crate) input_failures: Vec<TopologyInputFailureFacts>,
    pub(crate) misplaced_root_reporting_enabled: bool,
}

#[derive(Debug, Clone)]
struct ConfigResolution {
    global_hexarch_enabled: bool,
    packages_libarch_enabled: bool,
    app_hexarch_enabled: BTreeMap<String, bool>,
    failures: Vec<TopologyInputFailureFacts>,
}

pub fn collect(tree: &ProjectTree, route: &RsTopologyRoute) -> TopologyFacts {
    let config = resolve_config(tree);

    let mut input_failures: Vec<_> = route
        .input_failures()
        .iter()
        .map(|failure| TopologyInputFailureFacts {
            rel_path: failure.rel_path().to_owned(),
            message: failure.message().to_owned(),
            kind: TopologyInputFailureKind::RequiredInput,
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
                RsTopologyIssueKindView::TopLevelRootMustBeWorkspace => {
                    TopologyIssueKind::TopLevelRootMustBeWorkspace
                }
                RsTopologyIssueKindView::LooseTopLevelPackage => {
                    TopologyIssueKind::LooseTopLevelPackage
                }
                RsTopologyIssueKindView::NestedWorkspace { parent_workspace_rel } => TopologyIssueKind::NestedWorkspace {
                    parent_workspace_rel: parent_workspace_rel.clone(),
                },
                RsTopologyIssueKindView::UndeclaredWorkspaceMember { workspace_root_rel } => {
                    TopologyIssueKind::UndeclaredWorkspaceMember {
                        workspace_root_rel: workspace_root_rel.clone(),
                    }
                }
                RsTopologyIssueKindView::ExtraWorkspaceMember {
                    workspace_root_rel,
                    member_pattern,
                } => TopologyIssueKind::ExtraWorkspaceMember {
                    workspace_root_rel: workspace_root_rel.clone(),
                    member_pattern: member_pattern.clone(),
                },
                RsTopologyIssueKindView::WorkspaceMemberPathEscapesRoot {
                    workspace_root_rel,
                    member_pattern,
                } => TopologyIssueKind::WorkspaceMemberPathEscapesRoot {
                    workspace_root_rel: workspace_root_rel.clone(),
                    member_pattern: member_pattern.clone(),
                },
                RsTopologyIssueKindView::AuxiliaryTopLevelRootMustBeWorkspace => {
                    TopologyIssueKind::AuxiliaryTopLevelRootMustBeWorkspace
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

    TopologyFacts {
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
                global_hexarch_enabled: true,
                packages_libarch_enabled: true,
                app_hexarch_enabled: BTreeMap::new(),
                failures: vec![TopologyInputFailureFacts {
                    rel_path: "guardrail3.toml".to_owned(),
                    message:
                        "Failed to read guardrail3.toml for Rust architecture placement checks."
                            .to_owned(),
                    kind: TopologyInputFailureKind::RequiredInput,
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
            let rust = config.rust();
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

            let mut failures = scoped_topology_failures(&config);
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
            failures: vec![TopologyInputFailureFacts {
                rel_path: "guardrail3.toml".to_owned(),
                message: format!(
                    "Failed to parse guardrail3.toml for Rust architecture placement checks: {parse_error}"
                ),
                kind: TopologyInputFailureKind::RequiredInput,
            }],
        },
    }
}

fn misplaced_root_reporting_enabled(config: &ConfigResolution) -> bool {
    config.global_hexarch_enabled
        || config
            .app_hexarch_enabled
            .values()
            .copied()
            .any(std::convert::identity)
        || config.packages_libarch_enabled
}

fn scoped_topology_failures(config: &GuardrailConfig) -> Vec<TopologyInputFailureFacts> {
    let mut failures = Vec::new();
    let Some(rust) = config.rust() else {
        return failures;
    };

    if let Some(apps) = rust.apps() {
        for (app_name, app) in apps {
            if app
                .checks()
                .and_then(guardrail3_domain_config::types::RustChecksConfig::topology)
                .is_some()
            {
                failures.push(TopologyInputFailureFacts {
                    rel_path: "guardrail3.toml".to_owned(),
                    message: format!(
                        "Scoped `topology` config under `[rust.apps.{app_name}.checks]` is forbidden. `topology` is global-only and must be configured only under `[rust.checks]`."
                    ),
                    kind: TopologyInputFailureKind::ScopedTopologyConfig,
                });
            }
        }
    }

    if rust
        .packages()
        .and_then(guardrail3_domain_config::types::CrateConfig::checks)
        .and_then(guardrail3_domain_config::types::RustChecksConfig::topology)
        .is_some()
    {
        failures.push(TopologyInputFailureFacts {
            rel_path: "guardrail3.toml".to_owned(),
            message: "Scoped `topology` config under `[rust.packages.checks]` is forbidden. `topology` is global-only and must be configured only under `[rust.checks]`.".to_owned(),
            kind: TopologyInputFailureKind::ScopedTopologyConfig,
        });
    }

    failures
}

fn governed_root(root: &TopologyRootFacts, config: &ConfigResolution) -> Option<GovernedRootFacts> {
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

fn root_from_route(root: &RsTopologyRootView) -> TopologyRootFacts {
    let mut owner_families = Vec::new();
    if !root.app_zone_candidates().is_empty() {
        owner_families.push(RustArchitectureOwner::Hexarch);
    }
    if !root.package_zone_candidates().is_empty() {
        owner_families.push(RustArchitectureOwner::Libarch);
    }

    TopologyRootFacts {
        rel_dir: root.root().rel_dir().to_owned(),
        cargo_rel_path: root.root().cargo_rel_path().to_owned(),
        classification: root.classification(),
        app_zone_candidates: root.app_zone_candidates().to_vec(),
        package_zone_candidates: root.package_zone_candidates().to_vec(),
        owner_families,
    }
}
