use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-05";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable || krate.readme_declared_false || !krate.readme_exists {
        return;
    }
    let Some(content) = &krate.readme_content else {
        return;
    };
    if content.len() < 200 {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("{}: README is a stub", krate.name),
            message: format!(
                "README at `{}` is only {} bytes.",
                krate.readme_rel_path,
                content.len()
            ),
            file: Some(krate.readme_rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    }
    if !has_markdown_heading(content) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("{}: README has no heading", krate.name),
            message: format!(
                "README at `{}` has no markdown heading.",
                krate.readme_rel_path
            ),
            file: Some(krate.readme_rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    }
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: README quality looks good", krate.name),
            message: format!(
                "README at `{}` has content and headings.",
                krate.readme_rel_path
            ),
            file: Some(krate.readme_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
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
#[path = "rs_pub_05_readme_quality_tests/mod.rs"]
mod rs_pub_05_readme_quality_tests;

fn has_markdown_heading(content: &str) -> bool {
    let mut in_fenced_code = false;
    let mut last_text_line_can_be_setext_heading = false;
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fenced_code = !in_fenced_code;
            last_text_line_can_be_setext_heading = false;
            continue;
        }
        if in_fenced_code || line.starts_with("    ") || line.starts_with('\t') {
            last_text_line_can_be_setext_heading = false;
            continue;
        }
        if let Some(after_hashes) = trimmed.strip_prefix('#') {
            if after_hashes.starts_with('#') {
                let heading_text = trimmed.trim_start_matches('#');
                if heading_text.starts_with(char::is_whitespace) {
                    return true;
                }
                last_text_line_can_be_setext_heading = false;
                continue;
            }
            if after_hashes.starts_with(char::is_whitespace) {
                return true;
            }
            last_text_line_can_be_setext_heading = false;
        }
        if trimmed == "#" {
            return true;
        }
        if !trimmed.is_empty() && trimmed.chars().all(|ch| ch == '=' || ch == '-') {
            if last_text_line_can_be_setext_heading {
                return true;
            }
            last_text_line_can_be_setext_heading = false;
            continue;
        }
        last_text_line_can_be_setext_heading = !trimmed.is_empty();
    }
    false
}
