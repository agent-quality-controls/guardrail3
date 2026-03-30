#[derive(Debug, Clone)]
pub struct DependencyEdgeFacts {
    pub(crate) dep_name: String,
    pub(crate) dep_package_name: String,
    pub(crate) section_label: String,
    pub(crate) target_label: Option<String>,
    pub(crate) has_path: bool,
    pub(crate) version_req: Option<String>,
}

pub fn dependency_edges(
    parsed: &toml::Value,
    workspace_versions: &toml::map::Map<String, toml::Value>,
) -> Vec<DependencyEdgeFacts> {
    let mut edges = Vec::new();
    collect_dependency_edges_from_table(
        parsed,
        "dependencies",
        None,
        workspace_versions,
        &mut edges,
    );
    collect_dependency_edges_from_table(
        parsed,
        "build-dependencies",
        None,
        workspace_versions,
        &mut edges,
    );
    if let Some(target_table) = parsed.get("target").and_then(toml::Value::as_table) {
        for (target_name, target_config) in target_table {
            collect_dependency_edges_from_table(
                target_config,
                "dependencies",
                Some(target_name.as_str()),
                workspace_versions,
                &mut edges,
            );
            collect_dependency_edges_from_table(
                target_config,
                "build-dependencies",
                Some(target_name.as_str()),
                workspace_versions,
                &mut edges,
            );
        }
    }
    edges
}

fn collect_dependency_edges_from_table(
    table: &toml::Value,
    section_label: &str,
    target_label: Option<&str>,
    workspace_versions: &toml::map::Map<String, toml::Value>,
    edges: &mut Vec<DependencyEdgeFacts>,
) {
    let Some(section) = table.get(section_label).and_then(toml::Value::as_table) else {
        return;
    };
    for (dep_name, dep_value) in section {
        let workspace_value = workspace_versions.get(dep_name);
        let mut has_path = dep_value
            .as_table()
            .and_then(|table| table.get("path"))
            .and_then(toml::Value::as_str)
            .is_some();
        let workspace_inherited = dep_value
            .as_table()
            .and_then(|table| table.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        if workspace_inherited {
            has_path = has_path
                || workspace_versions
                    .get(dep_name)
                    .and_then(extract_workspace_dependency_path)
                    .is_some();
        }
        let dep_package_name = dep_value
            .as_table()
            .and_then(|table| table.get("package"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
            .or_else(|| {
                if workspace_inherited {
                    workspace_value.and_then(extract_workspace_dependency_package)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| dep_name.clone());
        let version_req = dep_value
            .as_str()
            .map(str::to_owned)
            .or_else(|| {
                dep_value
                    .as_table()
                    .and_then(|table| table.get("version"))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
            })
            .or_else(|| {
                if workspace_inherited {
                    workspace_versions
                        .get(dep_name)
                        .and_then(extract_workspace_dependency_version)
                } else {
                    None
                }
            });
        edges.push(DependencyEdgeFacts {
            dep_name: dep_name.clone(),
            dep_package_name,
            section_label: section_label.to_owned(),
            target_label: target_label.map(str::to_owned),
            has_path,
            version_req,
        });
    }
}

fn extract_workspace_dependency_version(value: &toml::Value) -> Option<String> {
    value.as_str().map(str::to_owned).or_else(|| {
        value
            .as_table()
            .and_then(|table| table.get("version"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
    })
}

fn extract_workspace_dependency_path(value: &toml::Value) -> Option<String> {
    value
        .as_table()
        .and_then(|table| table.get("path"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

fn extract_workspace_dependency_package(value: &toml::Value) -> Option<String> {
    value
        .as_table()
        .and_then(|table| table.get("package"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}
