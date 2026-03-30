use guardrail3_domain_project_tree::ProjectTree;

pub(super) fn collect_measurements(
    tree: &ProjectTree,
    parsed: Option<&toml::Value>,
    lib_rel_path: Option<&str>,
    cargo_parse_error: Option<&str>,
) -> (
    Option<String>,
    Option<usize>,
    Option<usize>,
    Option<usize>,
    Option<usize>,
    Vec<String>,
) {
    if let Some(error) = cargo_parse_error {
        return (
            Some(format!(
                "Cannot measure library complexity because the package Cargo.toml could not be parsed: {error}"
            )),
            None,
            None,
            None,
            None,
            Vec::new(),
        );
    }

    let Some(parsed) = parsed else {
        return (None, None, None, None, None, Vec::new());
    };

    let dependency_count = Some(
        dependency_names(
            parsed.get("dependencies"),
            parsed
                .get("workspace")
                .and_then(|value| value.get("dependencies"))
                .and_then(toml::Value::as_table),
        )
        .len(),
    );

    let Some(lib_rel_path) = lib_rel_path else {
        return (None, dependency_count, None, None, None, Vec::new());
    };
    if !tree.file_exists(lib_rel_path) {
        return (
            Some(format!(
                "Cannot measure library complexity because facade source `{lib_rel_path}` does not exist."
            )),
            dependency_count,
            None,
            None,
            None,
            Vec::new(),
        );
    }

    let source_root = parent_rel(lib_rel_path);
    if !tree.dir_exists(source_root) {
        return (
            Some(format!(
                "Cannot measure library complexity because source root `{source_root}` does not exist."
            )),
            dependency_count,
            None,
            None,
            None,
            Vec::new(),
        );
    }

    let measurements = measure_source_tree(tree, source_root);
    let mut threshold_reasons = Vec::new();
    if dependency_count.unwrap_or(0) > 12 {
        threshold_reasons.push(format!(
            "direct dependency count {} exceeds 12",
            dependency_count.unwrap_or(0)
        ));
    }
    if measurements.max_module_depth > 3 {
        threshold_reasons.push(format!(
            "module depth {} exceeds 3",
            measurements.max_module_depth
        ));
    }
    if measurements.max_sibling_dirs > 4 {
        threshold_reasons.push(format!(
            "sibling source directories {} exceeds 4",
            measurements.max_sibling_dirs
        ));
    }
    if measurements.max_sibling_rs_files > 6 {
        threshold_reasons.push(format!(
            "sibling .rs files {} exceeds 6",
            measurements.max_sibling_rs_files
        ));
    }

    (
        None,
        dependency_count,
        Some(measurements.max_module_depth),
        Some(measurements.max_sibling_dirs),
        Some(measurements.max_sibling_rs_files),
        threshold_reasons,
    )
}

#[derive(Debug, Clone, Default)]
struct SourceTreeMeasurements {
    max_module_depth: usize,
    max_sibling_dirs: usize,
    max_sibling_rs_files: usize,
}

fn dependency_names(
    section: Option<&toml::Value>,
    workspace_dependencies: Option<&toml::map::Map<String, toml::Value>>,
) -> std::collections::BTreeSet<String> {
    let Some(table) = section.and_then(toml::Value::as_table) else {
        return std::collections::BTreeSet::new();
    };
    table.iter()
        .map(|(alias, value)| dependency_package_name(alias, value, workspace_dependencies))
        .collect()
}

fn dependency_package_name(
    alias: &str,
    value: &toml::Value,
    workspace_dependencies: Option<&toml::map::Map<String, toml::Value>>,
) -> String {
    let dep_table = value.as_table();
    let workspace_value = dep_table
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .filter(|enabled| *enabled)
        .and_then(|_| workspace_dependencies.and_then(|deps| deps.get(alias)));

    dep_table
        .and_then(|table| table.get("package"))
        .and_then(toml::Value::as_str)
        .or_else(|| {
            workspace_value
                .and_then(toml::Value::as_table)
                .and_then(|table| table.get("package"))
                .and_then(toml::Value::as_str)
        })
        .unwrap_or(alias)
        .to_owned()
}

fn target_segments(rel_path: &str) -> Vec<&str> {
    rel_path.split('/').filter(|segment| !segment.is_empty()).collect()
}

fn measure_source_tree(tree: &ProjectTree, source_root: &str) -> SourceTreeMeasurements {
    let mut measurements = SourceTreeMeasurements::default();
    for (dir_rel, entry) in tree.structure() {
        if !dir_is_within_source_root(dir_rel, source_root) {
            continue;
        }
        measurements.max_sibling_dirs = measurements.max_sibling_dirs.max(entry.dirs().len());
        let rs_file_count = entry.files().iter().filter(|file| file.ends_with(".rs")).count();
        measurements.max_sibling_rs_files = measurements.max_sibling_rs_files.max(rs_file_count);

        for file_name in entry.files().iter().filter(|file| file.ends_with(".rs")) {
            let rel_path = ProjectTree::join_rel(dir_rel, file_name);
            measurements.max_module_depth =
                measurements.max_module_depth.max(module_depth(source_root, &rel_path));
        }
    }
    measurements
}

fn dir_is_within_source_root(dir_rel: &str, source_root: &str) -> bool {
    dir_rel == source_root
        || dir_rel
            .strip_prefix(source_root)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn module_depth(source_root: &str, rel_path: &str) -> usize {
    let source_segments = target_segments(source_root);
    let path_segments = target_segments(rel_path);
    if path_segments.len() <= source_segments.len() {
        return 0;
    }
    let relative = &path_segments[source_segments.len()..];
    let file_name = *relative.last().unwrap_or(&"");
    let dir_depth = relative.len().saturating_sub(1);
    if file_name == "lib.rs" || file_name == "mod.rs" {
        dir_depth
    } else {
        dir_depth + 1
    }
}

fn parent_rel(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}
