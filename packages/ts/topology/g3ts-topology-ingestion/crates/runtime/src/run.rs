use std::path::Path;

use g3ts_topology_ingestion_types::G3TsTopologyIngestionError as IngestionError;
use g3ts_topology_types::{
    G3TsTopologyDescendantGuardrail3TsToml, G3TsTopologyFileTreeChecksInput,
    G3TsTopologyFileTreeInputFailure, G3TsTopologyNestedGuardrail3TsTomlInput,
};
use walkdir::WalkDir;

/// Filename of the TS adoption marker that pairs with `package.json`.
const GUARDRAIL3_TS_TOML: &str = "guardrail3-ts.toml";
/// Filename of the npm/pnpm package manifest at the unit root.
const PACKAGE_JSON: &str = "package.json";

/// Walks `unit_root` and returns topology facts for the file-tree checks.
///
/// # Errors
///
/// Returns [`IngestionError::UnitRootMissing`] if `unit_root` is not a
/// directory, or [`IngestionError::UnitRootNotAdopted`] if either marker
/// of the adopted-unit pair is missing at `unit_root`.
pub fn ingest_for_file_tree_checks(
    unit_root: &Path,
) -> Result<G3TsTopologyFileTreeChecksInput, IngestionError> {
    if !unit_root.is_dir() {
        return Err(IngestionError::UnitRootMissing {
            path: unit_root.to_path_buf(),
        });
    }

    let unit_package_json = unit_root.join(PACKAGE_JSON);
    let unit_toml = unit_root.join(GUARDRAIL3_TS_TOML);

    if !unit_package_json.is_file() {
        return Err(IngestionError::UnitRootNotAdopted {
            path: unit_root.to_path_buf(),
            reason: format!("missing `{PACKAGE_JSON}` at unit root"),
        });
    }
    if !unit_toml.is_file() {
        return Err(IngestionError::UnitRootNotAdopted {
            path: unit_root.to_path_buf(),
            reason: format!("missing `{GUARDRAIL3_TS_TOML}` at unit root"),
        });
    }

    let (descendants, input_failures) = collect_descendant_tomls(unit_root);

    let mut input = G3TsTopologyFileTreeChecksInput {
        unit_root_rel_dir: String::new(),
        unit_root_package_json_rel_path: PACKAGE_JSON.to_owned(),
        unit_root_guardrail3_ts_toml_rel_path: GUARDRAIL3_TS_TOML.to_owned(),
        descendant_guardrail3_ts_tomls: descendants,
        input_failures,
        nested_guardrail3_ts_tomls: Vec::new(),
    };
    input.nested_guardrail3_ts_tomls = derive_nested_facts(&input);
    Ok(input)
}

/// Output of [`collect_descendant_tomls`]: discovered descendant markers
/// paired with any walk failures recorded as input facts.
type DescendantScan = (
    Vec<G3TsTopologyDescendantGuardrail3TsToml>,
    Vec<G3TsTopologyFileTreeInputFailure>,
);

/// Walks `unit_root` and returns descendant `guardrail3-ts.toml` facts and
/// any walk-time failures encountered along the way.
fn collect_descendant_tomls(unit_root: &Path) -> DescendantScan {
    let mut descendants = Vec::new();
    let mut failures = Vec::new();

    for entry in WalkDir::new(unit_root).follow_links(false) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                let path = err
                    .path()
                    .map_or_else(|| unit_root.to_path_buf(), Path::to_path_buf);
                failures.push(G3TsTopologyFileTreeInputFailure {
                    rel_path: rel_path_string(unit_root, &path),
                    message: err.to_string(),
                });
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }
        if entry.file_name() != GUARDRAIL3_TS_TOML {
            continue;
        }
        let abs = entry.path();
        // reason: the unit-root marker is not a "descendant".
        if abs == unit_root.join(GUARDRAIL3_TS_TOML) {
            continue;
        }
        if is_excluded_path(unit_root, abs) {
            continue;
        }

        let toml_rel_path = rel_path_string(unit_root, abs);
        let rel_dir = parent_rel(&toml_rel_path).to_owned();
        let sibling_package_json = abs
            .parent()
            .map(|parent| parent.join(PACKAGE_JSON))
            .is_some_and(|p| p.is_file());

        descendants.push(G3TsTopologyDescendantGuardrail3TsToml {
            rel_dir,
            toml_rel_path,
            has_sibling_package_json: sibling_package_json,
        });
    }

    descendants.sort_by(|left, right| left.toml_rel_path.cmp(&right.toml_rel_path));
    descendants.dedup_by(|left, right| left.toml_rel_path == right.toml_rel_path);
    failures.sort_by(|left, right| {
        left.rel_path
            .cmp(&right.rel_path)
            .then(left.message.cmp(&right.message))
    });

    (descendants, failures)
}

/// Projects descendant facts into nested-marker facts attributed to the
/// owning unit's relative root.
fn derive_nested_facts(
    input: &G3TsTopologyFileTreeChecksInput,
) -> Vec<G3TsTopologyNestedGuardrail3TsTomlInput> {
    let mut facts = input
        .descendant_guardrail3_ts_tomls
        .iter()
        .map(|descendant| G3TsTopologyNestedGuardrail3TsTomlInput {
            rel_dir: descendant.rel_dir.clone(),
            toml_rel_path: descendant.toml_rel_path.clone(),
            parent_unit_rel: input.unit_root_rel_dir.clone(),
        })
        .collect::<Vec<_>>();
    facts.sort_by(|left, right| left.toml_rel_path.cmp(&right.toml_rel_path));
    facts.dedup_by(|left, right| left.toml_rel_path == right.toml_rel_path);
    facts
}

/// Renders `abs` relative to `root` using forward slashes; falls back to
/// the absolute path when stripping fails.
fn rel_path_string(root: &Path, abs: &Path) -> String {
    abs.strip_prefix(root).map_or_else(
        |_| abs.to_string_lossy().replace('\\', "/"),
        |rel| rel.to_string_lossy().replace('\\', "/"),
    )
}

/// Returns the parent directory of a forward-slash relative path, or
/// the empty string when the path has no separator.
fn parent_rel(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

/// Returns true when `abs` falls inside an excluded subtree of `unit_root`
/// (build outputs, dependency caches, fixtures, snapshots).
fn is_excluded_path(unit_root: &Path, abs: &Path) -> bool {
    let Ok(rel) = abs.strip_prefix(unit_root) else {
        return false;
    };
    let segments = rel
        .components()
        .filter_map(|c| match c {
            std::path::Component::Normal(s) => s.to_str(),
            std::path::Component::Prefix(_)
            | std::path::Component::RootDir
            | std::path::Component::CurDir
            | std::path::Component::ParentDir => None,
        })
        .collect::<Vec<_>>();
    contains_sequence(&segments, &["target"])
        || contains_sequence(&segments, &["node_modules"])
        || contains_sequence(&segments, &[".git"])
        || contains_sequence(&segments, &["tests", "fixtures"])
        || contains_sequence(&segments, &["tests", "snapshots"])
}

/// Returns true when `sequence` appears as a contiguous subslice of
/// `segments`. Empty `sequence` returns false.
fn contains_sequence(segments: &[&str], sequence: &[&str]) -> bool {
    if sequence.is_empty() {
        return false;
    }
    segments
        .windows(sequence.len())
        .any(|window| window == sequence)
}
