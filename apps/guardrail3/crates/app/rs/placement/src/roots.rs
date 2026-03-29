use std::collections::BTreeSet;

use guardrail3_domain_project_tree::ProjectTree;
use toml::Value;

use crate::classification::{
    RustArchRole, RustRootPlacementRootFacts, classify_root, has_governed_zone_candidate,
};
use crate::overlap::{RustZoneOverlapFacts, collect_overlaps};

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

#[must_use]
pub fn collect(tree: &ProjectTree) -> RustRootPlacementFacts {
    if is_excluded_validation_root(tree) {
        return RustRootPlacementFacts::default();
    }

    let mut root_dirs = BTreeSet::new();
    if tree.file_exists("Cargo.toml") && !is_excluded_live_root_dir("") {
        let _ = root_dirs.insert(String::new());
    }
    root_dirs.extend(
        tree.dirs_with_file("Cargo.toml")
            .into_iter()
            .filter(|rel_dir| !is_excluded_live_root_dir(rel_dir)),
    );

    let mut roots = Vec::new();
    let mut input_failures = Vec::new();
    let zone_context_prefix = zone_context_prefix(tree);

    for rel_dir in root_dirs {
        let cargo_rel_path = if rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            ProjectTree::join_rel(&rel_dir, "Cargo.toml")
        };
        if tree.file_exists(&cargo_rel_path) && tree.file_content(&cargo_rel_path).is_none() {
            input_failures.push(RustRootPlacementInputFailureFacts {
                rel_path: cargo_rel_path.clone(),
                message:
                    "Failed to read eligible live Cargo.toml for Rust root placement discovery."
                        .to_owned(),
            });
        }

        let placement_rel_dir = contextual_rel_dir(zone_context_prefix.as_deref(), &rel_dir);
        let parsed = parse_live_cargo_toml(tree, &cargo_rel_path, &mut input_failures);
        let arch_role = if has_governed_zone_candidate(&placement_rel_dir) {
            if let Some(parsed) = parsed.as_ref() {
                if declares_arch_role(parsed) {
                    input_failures.push(RustRootPlacementInputFailureFacts {
                        rel_path: cargo_rel_path.clone(),
                        message: "Governed Rust roots under `apps/*` or `packages/*` must not declare `arch_role` in Cargo metadata. `arch_role = \"auxiliary\"` is only valid for roots outside governed architecture zones.".to_owned(),
                    });
                }
            }
            None
        } else {
            resolve_arch_role(parsed.as_ref(), &cargo_rel_path, &mut input_failures)
        };
        roots.push(classify_root(
            rel_dir,
            cargo_rel_path,
            &placement_rel_dir,
            arch_role,
        ));
    }

    roots.sort_by(|left, right| left.cargo_rel_path.cmp(&right.cargo_rel_path));
    let overlaps = collect_overlaps(&roots);

    RustRootPlacementFacts {
        roots,
        overlaps,
        input_failures,
    }
}

#[must_use]
pub fn is_excluded_live_root_dir(rel_dir: &str) -> bool {
    is_excluded_path(rel_dir)
}

fn is_excluded_validation_root(tree: &ProjectTree) -> bool {
    is_excluded_path(&tree.root.to_string_lossy().replace('\\', "/"))
}

fn is_excluded_path(path: &str) -> bool {
    let segments: Vec<_> = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();

    if segments.is_empty() {
        return false;
    }

    if segments.contains(&"target") {
        return true;
    }

    if segments
        .windows(2)
        .any(|window| matches!(window, [".claude", "worktrees"]))
    {
        return true;
    }

    segments
        .windows(2)
        .any(|window| matches!(window, ["tests", "fixtures"] | ["tests", "snapshots"]))
}

fn resolve_arch_role(
    parsed: Option<&Value>,
    cargo_rel_path: &str,
    input_failures: &mut Vec<RustRootPlacementInputFailureFacts>,
) -> Option<RustArchRole> {
    let Some(parsed) = parsed else {
        return None;
    };

    match arch_role_from_toml(parsed) {
        Ok(role) => role,
        Err(message) => {
            input_failures.push(RustRootPlacementInputFailureFacts {
                rel_path: cargo_rel_path.to_owned(),
                message,
            });
            None
        }
    }
}

fn parse_live_cargo_toml(
    tree: &ProjectTree,
    cargo_rel_path: &str,
    input_failures: &mut Vec<RustRootPlacementInputFailureFacts>,
) -> Option<Value> {
    let Some(content) = tree.file_content(cargo_rel_path) else {
        return None;
    };

    match toml::from_str::<Value>(content) {
        Ok(parsed) => Some(parsed),
        Err(parse_error) => {
            input_failures.push(RustRootPlacementInputFailureFacts {
                rel_path: cargo_rel_path.to_owned(),
                message: format!(
                    "Failed to parse eligible live Cargo.toml for Rust root placement discovery: {parse_error}"
                ),
            });
            None
        }
    }
}

fn arch_role_from_toml(parsed: &Value) -> Result<Option<RustArchRole>, String> {
    let Some(value) = arch_role_value(parsed) else {
        return Ok(None);
    };

    let Some(value) = value.as_str() else {
        return Err(
            "Invalid `arch_role` in Cargo metadata for Rust root placement discovery: expected a string value like `\"auxiliary\"`."
                .to_owned(),
        );
    };

    match value {
        "auxiliary" => Ok(Some(RustArchRole::Auxiliary)),
        other => Err(format!(
            "Invalid `arch_role = \"{other}\"` in Cargo metadata for Rust root placement discovery. Expected `\"auxiliary\"`."
        )),
    }
}

fn declares_arch_role(parsed: &Value) -> bool {
    arch_role_value(parsed).is_some()
}

fn arch_role_value(parsed: &Value) -> Option<&Value> {
    parsed
        .get("package")
        .and_then(|value| value.get("metadata"))
        .and_then(|value| value.get("guardrail3"))
        .and_then(|value| value.get("arch_role"))
        .or_else(|| {
            parsed
                .get("workspace")
                .and_then(|value| value.get("metadata"))
                .and_then(|value| value.get("guardrail3"))
                .and_then(|value| value.get("arch_role"))
        })
}

fn zone_context_prefix(tree: &ProjectTree) -> Option<String> {
    let segments: Vec<_> = tree
        .root
        .iter()
        .filter_map(|segment| segment.to_str())
        .collect();

    segments
        .iter()
        .enumerate()
        .rev()
        .find_map(|(index, segment)| {
            if (*segment == "apps" || *segment == "packages") && index + 1 < segments.len() {
                Some(format!("{}/{}", segment, segments[index + 1]))
            } else {
                None
            }
        })
}

fn contextual_rel_dir(zone_context_prefix: Option<&str>, rel_dir: &str) -> String {
    match zone_context_prefix {
        Some(prefix) if rel_dir.is_empty() => prefix.to_owned(),
        Some(prefix) => format!("{prefix}/{rel_dir}"),
        None => rel_dir.to_owned(),
    }
}
