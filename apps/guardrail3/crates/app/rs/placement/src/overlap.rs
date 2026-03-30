use std::collections::{BTreeMap, BTreeSet};

use crate::classification::RustRootPlacementRootFacts;

#[derive(Debug, Clone)]
pub struct RustZoneOverlapFacts {
    app_root_rel: String,
    app_cargo_rel_path: String,
    package_root_rel: String,
    package_cargo_rel_path: String,
}

impl RustZoneOverlapFacts {
    #[must_use]
    pub fn new(
        app_root_rel: String,
        app_cargo_rel_path: String,
        package_root_rel: String,
        package_cargo_rel_path: String,
    ) -> Self {
        Self {
            app_root_rel,
            app_cargo_rel_path,
            package_root_rel,
            package_cargo_rel_path,
        }
    }

    #[must_use]
    pub fn app_root_rel(&self) -> &str {
        &self.app_root_rel
    }

    #[must_use]
    pub fn app_cargo_rel_path(&self) -> &str {
        &self.app_cargo_rel_path
    }

    #[must_use]
    pub fn package_root_rel(&self) -> &str {
        &self.package_root_rel
    }

    #[must_use]
    pub fn package_cargo_rel_path(&self) -> &str {
        &self.package_cargo_rel_path
    }
}

#[must_use]
pub fn collect_overlaps(roots: &[RustRootPlacementRootFacts]) -> Vec<RustZoneOverlapFacts> {
    let roots_by_rel_dir = roots
        .iter()
        .map(|root| (root.rel_dir(), root))
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut overlaps = Vec::new();

    for root in roots {
        if root.app_zone_candidates().is_empty() || root.package_zone_candidates().is_empty() {
            continue;
        }

        for app_root_rel in root.app_zone_candidates() {
            let Some(app_root) = roots_by_rel_dir.get(app_root_rel.as_str()).copied() else {
                continue;
            };

            for package_root_rel in root.package_zone_candidates() {
                let Some(package_root) = roots_by_rel_dir.get(package_root_rel.as_str()).copied()
                else {
                    continue;
                };

                if app_root.rel_dir() == package_root.rel_dir() {
                    continue;
                }
                if !dirs_overlap(app_root.rel_dir(), package_root.rel_dir()) {
                    continue;
                }

                let key = (
                    app_root.rel_dir().to_owned(),
                    package_root.rel_dir().to_owned(),
                );
                if !seen.insert(key.clone()) {
                    continue;
                }

                overlaps.push(RustZoneOverlapFacts::new(
                    key.0,
                    app_root.cargo_rel_path().to_owned(),
                    key.1,
                    package_root.cargo_rel_path().to_owned(),
                ));
            }
        }
    }

    overlaps.sort_by(|left, right| {
        left.app_root_rel()
            .cmp(right.app_root_rel())
            .then(left.package_root_rel().cmp(right.package_root_rel()))
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
