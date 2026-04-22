use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyFileTreeChecksInput, G3RsTopologyWorkspaceFamily,
    G3RsTopologyWorkspaceFamilyFileAttachment, G3RsTopologyWorkspaceFamilyFileKind,
    G3RsTopologyWorkspaceMemberIssueKind,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    severity: G3Severity,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    line: Option<usize>,
    inventory: bool,
}

#[must_use]
pub fn findings<'a>(results: &'a [G3CheckResult], id: &str) -> Vec<Finding<'a>> {
    let mut findings = results
        .iter()
        .filter(|result| result.id() == id)
        .map(Finding::from_result)
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| {
        (
            format!("{:?}", left.severity),
            left.title,
            left.message,
            left.file,
            left.line,
            left.inventory,
        )
            .cmp(&(
                format!("{:?}", right.severity),
                right.title,
                right.message,
                right.file,
                right.line,
                right.inventory,
            ))
    });
    findings
}

#[must_use]
pub fn count(results: &[G3CheckResult], id: &str) -> usize {
    findings(results, id).len()
}

pub fn assert_present(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
) {
    let findings = findings(results, id);
    assert!(
        findings.iter().any(|finding| finding.title == title
            && finding.file == file
            && finding.inventory == inventory),
        "{findings:#?}"
    );
}

pub fn assert_message_contains(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
    needle: &str,
) {
    let findings = findings(results, id);
    assert!(
        findings.iter().any(|finding| {
            finding.title == title
                && finding.file == file
                && finding.inventory == inventory
                && finding.message.contains(needle)
        }),
        "{findings:#?}"
    );
}

pub fn assert_empty(results: &[G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}

pub fn assert_no_input_failures(input: &G3RsTopologyFileTreeChecksInput) {
    assert!(
        input.input_failures.is_empty(),
        "{:#?}",
        input.input_failures
    );
}

pub fn assert_input_failure_contains(
    input: &G3RsTopologyFileTreeChecksInput,
    rel_path: &str,
    needle: &str,
) {
    assert!(
        input
            .input_failures
            .iter()
            .any(|failure| { failure.rel_path == rel_path && failure.message.contains(needle) }),
        "{:#?}",
        input.input_failures
    );
}

pub fn assert_descendant_root(
    input: &G3RsTopologyFileTreeChecksInput,
    rel_dir: &str,
    cargo_rel_path: &str,
    manifest_kind: Option<G3RsTopologyCargoManifestKind>,
) {
    assert!(
        input.descendant_cargo_roots.iter().any(|root| {
            root.rel_dir == rel_dir
                && root.cargo_rel_path == cargo_rel_path
                && root.manifest_kind == manifest_kind
        }),
        "{:#?}",
        input.descendant_cargo_roots
    );
}

pub fn assert_family_file(
    input: &G3RsTopologyFileTreeChecksInput,
    family: G3RsTopologyWorkspaceFamily,
    rel_path: &str,
    kind: G3RsTopologyWorkspaceFamilyFileKind,
) {
    assert!(
        input.family_files.iter().any(|file| {
            file.family == family && file.rel_path == rel_path && file.kind == kind
        }),
        "{:#?}",
        input.family_files
    );
}

pub fn assert_family_file_attachment(
    input: &G3RsTopologyFileTreeChecksInput,
    family: G3RsTopologyWorkspaceFamily,
    rel_path: &str,
    kind: G3RsTopologyWorkspaceFamilyFileKind,
    attachment: G3RsTopologyWorkspaceFamilyFileAttachment,
) {
    assert!(
        input.family_files.iter().any(|file| {
            file.family == family
                && file.rel_path == rel_path
                && file.kind == kind
                && file.attachment == attachment
        }),
        "{:#?}",
        input.family_files
    );
}

pub fn assert_exact_family_file_count(
    input: &G3RsTopologyFileTreeChecksInput,
    rel_path: &str,
    expected: usize,
) {
    let actual = input
        .family_files
        .iter()
        .filter(|file| file.rel_path == rel_path)
        .count();
    assert_eq!(
        actual, expected,
        "unexpected family file count for {rel_path}"
    );
}

pub fn assert_nested_workspace(
    input: &G3RsTopologyFileTreeChecksInput,
    rel_dir: &str,
    cargo_rel_path: &str,
    parent_workspace_rel: &str,
) {
    assert!(
        input.nested_workspaces.iter().any(|nested| {
            nested.rel_dir == rel_dir
                && nested.cargo_rel_path == cargo_rel_path
                && nested.parent_workspace_rel == parent_workspace_rel
        }),
        "{:#?}",
        input.nested_workspaces
    );
}

pub fn assert_escaping_member_path(
    input: &G3RsTopologyFileTreeChecksInput,
    cargo_rel_path: &str,
    workspace_root_rel: &str,
    member_pattern: &str,
) {
    assert!(
        input.escaping_member_paths.iter().any(|escaping| {
            escaping.cargo_rel_path == cargo_rel_path
                && escaping.workspace_root_rel == workspace_root_rel
                && escaping.member_pattern == member_pattern
        }),
        "{:#?}",
        input.escaping_member_paths
    );
}

pub fn assert_undeclared_member_issue(
    input: &G3RsTopologyFileTreeChecksInput,
    rel_dir: &str,
    cargo_rel_path: &str,
    workspace_root_rel: &str,
) {
    assert!(
        input.membership_issues.iter().any(|issue| {
            issue.rel_dir == rel_dir
                && issue.cargo_rel_path == cargo_rel_path
                && issue.kind
                    == G3RsTopologyWorkspaceMemberIssueKind::Undeclared {
                        workspace_root_rel: workspace_root_rel.to_owned(),
                    }
        }),
        "{:#?}",
        input.membership_issues
    );
}

pub fn assert_extra_member_issue(
    input: &G3RsTopologyFileTreeChecksInput,
    cargo_rel_path: &str,
    workspace_root_rel: &str,
    member_pattern: &str,
) {
    assert!(
        input.membership_issues.iter().any(|issue| {
            issue.cargo_rel_path == cargo_rel_path
                && issue.kind
                    == G3RsTopologyWorkspaceMemberIssueKind::Extra {
                        workspace_root_rel: workspace_root_rel.to_owned(),
                        member_pattern: member_pattern.to_owned(),
                    }
        }),
        "{:#?}",
        input.membership_issues
    );
}

pub fn assert_illegal_family_file(
    input: &G3RsTopologyFileTreeChecksInput,
    family: G3RsTopologyWorkspaceFamily,
    rel_path: &str,
    message_needle: &str,
) {
    assert!(
        input.illegal_family_files.iter().any(|file| {
            file.family == family
                && file.rel_path == rel_path
                && file.reason.contains(message_needle)
        }),
        "{:#?}",
        input.illegal_family_files
    );
}

impl<'a> Finding<'a> {
    fn from_result(result: &'a G3CheckResult) -> Self {
        Self {
            severity: result.severity(),
            title: result.title(),
            message: result.message(),
            file: result.file(),
            line: result.line(),
            inventory: result.inventory(),
        }
    }
}
