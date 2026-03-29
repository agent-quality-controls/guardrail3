use std::collections::{BTreeMap, BTreeSet};

use crate::classification::RustRootPlacementRootFacts;

#[derive(Debug, Clone)]
pub struct RustZoneOverlapFacts {
    pub app_root_rel: String,
    pub app_cargo_rel_path: String,
    pub package_root_rel: String,
    pub package_cargo_rel_path: String,
}

#[must_use]
pub fn collect_overlaps(roots: &[RustRootPlacementRootFacts]) -> Vec<RustZoneOverlapFacts> {
    let roots_by_rel_dir = roots
        .iter()
        .map(|root| (root.rel_dir.as_str(), root))
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut overlaps = Vec::new();

    for root in roots {
        if root.app_zone_candidates.is_empty() || root.package_zone_candidates.is_empty() {
            continue;
        }

        for app_root_rel in &root.app_zone_candidates {
            let Some(app_root) = roots_by_rel_dir.get(app_root_rel.as_str()).copied() else {
                continue;
            };

            for package_root_rel in &root.package_zone_candidates {
                let Some(package_root) = roots_by_rel_dir.get(package_root_rel.as_str()).copied()
                else {
                    continue;
                };

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
