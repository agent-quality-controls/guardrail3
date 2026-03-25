use std::collections::BTreeSet;

use guardrail3_domain_project_tree::ProjectTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RustArchitectureOwner {
    Hexarch,
    Libarch,
}

impl RustArchitectureOwner {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hexarch => "app",
            Self::Libarch => "package",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustRootClassification {
    App,
    Package,
    Other,
    Ambiguous,
}

#[derive(Debug, Clone)]
pub struct RustRootPlacementRootFacts {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub classification: RustRootClassification,
    pub app_zone_candidates: Vec<String>,
    pub package_zone_candidates: Vec<String>,
    pub owner_families: Vec<RustArchitectureOwner>,
}

#[derive(Debug, Clone)]
pub struct RustZoneOverlapFacts {
    pub app_root_rel: String,
    pub app_cargo_rel_path: String,
    pub package_root_rel: String,
    pub package_cargo_rel_path: String,
}

#[derive(Debug, Clone)]
pub struct RustRootPlacementInputFailureFacts {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct RustRootPlacementFacts {
    pub roots: Vec<RustRootPlacementRootFacts>,
    pub overlaps: Vec<RustZoneOverlapFacts>,
    pub input_failures: Vec<RustRootPlacementInputFailureFacts>,
}

pub fn collect(tree: &ProjectTree) -> RustRootPlacementFacts {
    let mut root_dirs = BTreeSet::new();
    if tree.file_exists("Cargo.toml") {
        let _ = root_dirs.insert(String::new());
    }
    root_dirs.extend(tree.dirs_with_file("Cargo.toml"));

    let mut roots = Vec::new();
    let mut input_failures = Vec::new();

    for rel_dir in root_dirs {
        let cargo_rel_path = if rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            ProjectTree::join_rel(&rel_dir, "Cargo.toml")
        };
        if tree.file_content(&cargo_rel_path).is_none() {
            input_failures.push(RustRootPlacementInputFailureFacts {
                rel_path: cargo_rel_path.clone(),
                message: "Failed to read Cargo.toml for Rust root placement discovery.".to_owned(),
            });
        }

        let app_zone_candidates = zone_candidates(&rel_dir, "apps");
        let package_zone_candidates = zone_candidates(&rel_dir, "packages");

        let classification = match (app_zone_candidates.len(), package_zone_candidates.len()) {
            (0, 0) => RustRootClassification::Other,
            (1, 0) => RustRootClassification::App,
            (0, 1) => RustRootClassification::Package,
            _ => RustRootClassification::Ambiguous,
        };

        let mut owner_families = Vec::new();
        if !app_zone_candidates.is_empty() {
            owner_families.push(RustArchitectureOwner::Hexarch);
        }
        if !package_zone_candidates.is_empty() {
            owner_families.push(RustArchitectureOwner::Libarch);
        }

        roots.push(RustRootPlacementRootFacts {
            rel_dir,
            cargo_rel_path,
            classification,
            app_zone_candidates,
            package_zone_candidates,
            owner_families,
        });
    }

    let overlaps = collect_overlaps(&roots);

    RustRootPlacementFacts {
        roots,
        overlaps,
        input_failures,
    }
}

fn zone_candidates(rel_dir: &str, zone_dir: &str) -> Vec<String> {
    let segments: Vec<_> = rel_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.len() < 2 {
        return Vec::new();
    }

    segments
        .iter()
        .enumerate()
        .filter_map(|(index, segment)| {
            if *segment != zone_dir || index + 1 >= segments.len() {
                return None;
            }
            Some(segments[..=index + 1].join("/"))
        })
        .collect()
}

fn collect_overlaps(roots: &[RustRootPlacementRootFacts]) -> Vec<RustZoneOverlapFacts> {
    let mut seen = BTreeSet::new();
    let mut overlaps = Vec::new();

    for app_root in roots
        .iter()
        .filter(|root| !root.app_zone_candidates.is_empty())
    {
        for package_root in roots
            .iter()
            .filter(|root| !root.package_zone_candidates.is_empty())
        {
            if app_root.rel_dir == package_root.rel_dir {
                continue;
            }
            if !dirs_overlap(&app_root.rel_dir, &package_root.rel_dir) {
                continue;
            }
            let key = (app_root.rel_dir.clone(), package_root.rel_dir.clone());
            if !seen.insert(key.clone()) {
                continue;
            }
            overlaps.push(RustZoneOverlapFacts {
                app_root_rel: key.0,
                app_cargo_rel_path: app_root.cargo_rel_path.clone(),
                package_root_rel: key.1,
                package_cargo_rel_path: package_root.cargo_rel_path.clone(),
            });
        }
    }

    overlaps.sort_by(|left, right| {
        left.app_root_rel
            .cmp(&right.app_root_rel)
            .then(left.package_root_rel.cmp(&right.package_root_rel))
    });
    overlaps
}

fn dirs_overlap(left: &str, right: &str) -> bool {
    is_ancestor_dir(left, right) || is_ancestor_dir(right, left)
}

fn is_ancestor_dir(parent: &str, child: &str) -> bool {
    !parent.is_empty()
        && !child.is_empty()
        && child.starts_with(parent)
        && child[parent.len()..].starts_with('/')
}
