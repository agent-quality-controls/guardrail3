use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-03";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let repo = input.repo;
    if !repo.release_plz_exists {
        return;
    }
    if repo.release_plz_parsed.is_none() {
        return;
    }
    let workspace = repo
        .release_plz_parsed
        .as_ref()
        .and_then(|parsed| parsed.get("workspace"))
        .and_then(toml::Value::as_table);

    if workspace.is_none() {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "release-plz.toml missing [workspace]".to_owned(),
            message: "`release-plz.toml` is missing a `[workspace]` section.".to_owned(),
            file: Some(repo.release_plz_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }

    if let Some(workspace) = workspace {
        if workspace
            .get("changelog_config")
            .and_then(toml::Value::as_str)
            != Some("cliff.toml")
        {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "release-plz.toml missing canonical changelog_config".to_owned(),
                message:
                    "`release-plz.toml` should set `[workspace].changelog_config = \"cliff.toml\"`."
                        .to_owned(),
                file: Some(repo.release_plz_rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
        if workspace
            .get("git_release_enable")
            .and_then(toml::Value::as_bool)
            != Some(true)
        {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "release-plz.toml missing git_release_enable = true".to_owned(),
                message: "`release-plz.toml` should set `[workspace].git_release_enable = true`."
                    .to_owned(),
                file: Some(repo.release_plz_rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
        if workspace
            .get("release_always")
            .and_then(toml::Value::as_bool)
            != Some(false)
        {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "release-plz.toml missing release_always = false".to_owned(),
                message: "`release-plz.toml` should set `[workspace].release_always = false`."
                    .to_owned(),
                file: Some(repo.release_plz_rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }

    let missing = repo
        .publishable_crate_names
        .difference(&repo.release_plz_package_names)
        .cloned()
        .collect::<Vec<_>>();
    if missing.is_empty()
        && !results
            .iter()
            .any(|result| result.id == ID && !result.inventory)
    {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "release-plz baseline and package coverage complete".to_owned(),
                message:
                    "`release-plz.toml` has the canonical workspace baseline and covers all publishable crates."
                        .to_owned(),
                file: Some(repo.release_plz_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }
    for crate_name in missing {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("release-plz missing crate `{crate_name}`"),
            message: format!(
                "Publishable crate `{crate_name}` is missing from `release-plz.toml` `[[package]]` coverage."
            ),
            file: Some(repo.release_plz_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}


#[cfg(test)]
#[allow(dead_code)]
pub(super) fn run_tree(
    tree: &guardrail3_domain_project_tree::ProjectTree,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree(tree, tc, thorough)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn run_family(
    root: &std::path::Path,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_family(root, thorough)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn copy_fixture() -> tempfile::TempDir {
    crate::test_fixtures::copy_fixture()
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn errors_by_id<'a>(
    results: &'a [guardrail3_domain_report::CheckResult],
    id: &str,
) -> Vec<&'a guardrail3_domain_report::CheckResult> {
    results
        .iter()
        .filter(|result| result.id == id)
        .collect()
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    crate::test_fixtures::crate_facts(name)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::test_fixtures::crate_input(krate)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::test_fixtures::repo_input(repo)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn edge_facts() -> crate::facts::ReleaseEdgeFacts {
    crate::test_fixtures::edge_facts()
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn edge_input(
    edge: &crate::facts::ReleaseEdgeFacts,
) -> crate::inputs::ReleaseEdgeInput<'_> {
    crate::test_fixtures::edge_input(edge)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn workflow_from_yaml(rel_path: &str, yaml: &str) -> crate::facts::WorkflowFacts {
    crate::test_fixtures::workflow_from_yaml(rel_path, yaml)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn dependency_edges(
    parsed: &toml::Value,
    workspace_dependencies: &toml::map::Map<String, toml::Value>,
) -> Vec<crate::release_support::DependencyEdgeFacts> {
    crate::release_support::dependency_edges(parsed, workspace_dependencies)
}

#[cfg(test)]
#[allow(unused_imports)]
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root, write_file};

#[cfg(test)]
#[path = "rs_release_03_release_plz_coverage_tests/mod.rs"]
mod rs_release_03_release_plz_coverage_tests;
