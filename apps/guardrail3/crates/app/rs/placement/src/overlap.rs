use std::collections::BTreeSet;

use crate::classification::{RustRootClassification, RustRootPlacementRootFacts};

#[derive(Debug, Clone)]
pub struct RustZoneOverlapFacts {
    pub app_root_rel: String,
    pub app_cargo_rel_path: String,
    pub package_root_rel: String,
    pub package_cargo_rel_path: String,
}

#[must_use]
pub fn collect_overlaps(roots: &[RustRootPlacementRootFacts]) -> Vec<RustZoneOverlapFacts> {
    let mut seen = BTreeSet::new();
    let mut overlaps = Vec::new();

    for app_root in roots
        .iter()
        .filter(|root| root.classification == RustRootClassification::App)
    {
        for package_root in roots
            .iter()
            .filter(|root| root.classification == RustRootClassification::Package)
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
