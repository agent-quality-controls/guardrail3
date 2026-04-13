use std::collections::BTreeSet;

use g3rs_topology_file_tree_checks_types::G3RsTopologyFileTreeChecksInput;
use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyFileTreeInputFailure, G3RsTopologyWorkspaceFamily,
    G3RsTopologyWorkspaceFamilyFile, G3RsTopologyWorkspaceFamilyFileAttachment,
    G3RsTopologyWorkspaceFamilyFileKind,
};
use glob::Pattern;

#[derive(Debug, Clone)]
pub(crate) struct TopologyIssue {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) kind: TopologyIssueKind,
}

#[derive(Debug, Clone)]
pub(crate) enum TopologyIssueKind {
    NestedWorkspace {
        parent_workspace_rel: String,
    },
    UndeclaredWorkspaceMember {
        workspace_root_rel: String,
    },
    ExtraWorkspaceMember {
        workspace_root_rel: String,
        member_pattern: String,
    },
    WorkspaceMemberPathEscapesRoot {
        workspace_root_rel: String,
        member_pattern: String,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct IllegalFamilyFilePlacement {
    pub(crate) family: G3RsTopologyWorkspaceFamily,
    pub(crate) rel_path: String,
    pub(crate) reason: String,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TopologyFacts {
    pub(crate) input_failures: Vec<G3RsTopologyFileTreeInputFailure>,
    pub(crate) issues: Vec<TopologyIssue>,
    pub(crate) illegal_family_files: Vec<IllegalFamilyFilePlacement>,
}

#[derive(Debug, Clone)]
struct ResolvedMemberPattern {
    raw: String,
    matched_unresolved_dirs: Vec<String>,
    resolved_child_dirs: Vec<String>,
}

pub(crate) fn collect_facts(input: &G3RsTopologyFileTreeChecksInput) -> TopologyFacts {
    let workspace_root_rel = input.workspace_root_rel_dir.as_str();
    let workspace_roots = std::iter::once(workspace_root_rel.to_owned())
        .chain(
            input
                .descendant_cargo_roots
                .iter()
                .filter(|root| {
                    matches!(
                        root.manifest_kind,
                        Some(
                            G3RsTopologyCargoManifestKind::Workspace
                                | G3RsTopologyCargoManifestKind::Hybrid
                        )
                    )
                })
                .map(|root| root.rel_dir.clone()),
        )
        .collect::<BTreeSet<_>>();

    let mut issues = collect_nested_workspace_issues(input, &workspace_roots);
    let escaping_patterns = collect_escaping_member_patterns(input);
    let actual_children = collect_actual_children(input, &workspace_roots);
    let member_patterns = collect_member_patterns(input, &actual_children);

    issues.extend(
        escaping_patterns
            .iter()
            .map(|member_pattern| TopologyIssue {
                rel_dir: workspace_root_rel.to_owned(),
                cargo_rel_path: input.workspace_root_cargo_rel_path.clone(),
                kind: TopologyIssueKind::WorkspaceMemberPathEscapesRoot {
                    workspace_root_rel: workspace_root_rel.to_owned(),
                    member_pattern: member_pattern.clone(),
                },
            }),
    );

    for child in &actual_children {
        let declared = member_patterns.iter().any(|member| {
            member
                .resolved_child_dirs
                .iter()
                .any(|resolved| resolved == child)
        });
        if declared {
            continue;
        }
        let cargo_rel_path = join_rel(child, "Cargo.toml");
        issues.push(TopologyIssue {
            rel_dir: child.clone(),
            cargo_rel_path,
            kind: TopologyIssueKind::UndeclaredWorkspaceMember {
                workspace_root_rel: workspace_root_rel.to_owned(),
            },
        });
    }

    let actual_children_set = actual_children.iter().cloned().collect::<BTreeSet<_>>();
    for member in &member_patterns {
        let covers_real_child = !member.resolved_child_dirs.is_empty();
        if covers_real_child || !member.matched_unresolved_dirs.is_empty() {
            continue;
        }
        issues.push(TopologyIssue {
            rel_dir: workspace_root_rel.to_owned(),
            cargo_rel_path: input.workspace_root_cargo_rel_path.clone(),
            kind: TopologyIssueKind::ExtraWorkspaceMember {
                workspace_root_rel: workspace_root_rel.to_owned(),
                member_pattern: member.raw.clone(),
            },
        });
    }

    issues.sort_by(|left, right| {
        left.cargo_rel_path
            .cmp(&right.cargo_rel_path)
            .then(issue_sort_key(&left.kind).cmp(&issue_sort_key(&right.kind)))
    });
    issues.dedup_by(|left, right| {
        left.rel_dir == right.rel_dir
            && left.cargo_rel_path == right.cargo_rel_path
            && issue_sort_key(&left.kind) == issue_sort_key(&right.kind)
    });

    let legal_member_roots = actual_children
        .iter()
        .filter(|rel_dir| {
            input.descendant_cargo_roots.iter().any(|root| {
                root.rel_dir == **rel_dir
                    && root.manifest_kind == Some(G3RsTopologyCargoManifestKind::Package)
                    && member_patterns.iter().any(|member| {
                        member
                            .resolved_child_dirs
                            .iter()
                            .any(|resolved| resolved == *rel_dir)
                    })
            })
        })
        .cloned()
        .collect::<BTreeSet<_>>();

    let mut illegal_family_files = input
        .family_files
        .iter()
        .filter_map(|file| {
            classify_illegal_family_file(
                file,
                workspace_root_rel,
                &actual_children_set,
                &legal_member_roots,
            )
        })
        .collect::<Vec<_>>();
    illegal_family_files.sort_by(|left, right| {
        left.family
            .cmp(&right.family)
            .then(left.rel_path.cmp(&right.rel_path))
            .then(left.reason.cmp(&right.reason))
    });

    TopologyFacts {
        input_failures: input.input_failures.clone(),
        issues,
        illegal_family_files,
    }
}

fn collect_nested_workspace_issues(
    input: &G3RsTopologyFileTreeChecksInput,
    workspace_roots: &BTreeSet<String>,
) -> Vec<TopologyIssue> {
    input
        .descendant_cargo_roots
        .iter()
        .filter(|root| {
            matches!(
                root.manifest_kind,
                Some(
                    G3RsTopologyCargoManifestKind::Workspace
                        | G3RsTopologyCargoManifestKind::Hybrid
                )
            )
        })
        .filter_map(|root| {
            nearest_ancestor_workspace(&root.rel_dir, workspace_roots).map(|parent_workspace_rel| {
                TopologyIssue {
                    rel_dir: root.rel_dir.clone(),
                    cargo_rel_path: root.cargo_rel_path.clone(),
                    kind: TopologyIssueKind::NestedWorkspace {
                        parent_workspace_rel: parent_workspace_rel.to_owned(),
                    },
                }
            })
        })
        .collect()
}

fn collect_escaping_member_patterns(input: &G3RsTopologyFileTreeChecksInput) -> Vec<String> {
    input
        .workspace_manifest
        .workspace
        .as_ref()
        .map(|workspace| {
            workspace
                .members
                .iter()
                .filter(|member| member_pattern_escapes_root(member))
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn collect_member_patterns(
    input: &G3RsTopologyFileTreeChecksInput,
    actual_children: &[String],
) -> Vec<ResolvedMemberPattern> {
    let workspace_root_rel = input.workspace_root_rel_dir.as_str();
    let unresolved_descendant_rels = input
        .descendant_cargo_roots
        .iter()
        .filter(|root| root.manifest_kind.is_none())
        .map(|root| root.rel_dir.clone())
        .collect::<Vec<_>>();

    input
        .workspace_manifest
        .workspace
        .as_ref()
        .map(|workspace| {
            workspace
                .members
                .iter()
                .filter(|member| !member_pattern_escapes_root(member))
                .map(|member| ResolvedMemberPattern {
                    raw: member.clone(),
                    matched_unresolved_dirs: resolve_member_pattern(
                        workspace_root_rel,
                        member,
                        &unresolved_descendant_rels,
                    ),
                    resolved_child_dirs: resolve_member_pattern(
                        workspace_root_rel,
                        member,
                        actual_children,
                    ),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn collect_actual_children(
    input: &G3RsTopologyFileTreeChecksInput,
    workspace_roots: &BTreeSet<String>,
) -> Vec<String> {
    let workspace_root_rel = input.workspace_root_rel_dir.as_str();
    input
        .descendant_cargo_roots
        .iter()
        .filter(|root| root.rel_dir != workspace_root_rel)
        .filter(|root| root.manifest_kind == Some(G3RsTopologyCargoManifestKind::Package))
        .filter(|root| {
            nearest_ancestor_workspace(&root.rel_dir, workspace_roots) == Some(workspace_root_rel)
        })
        .map(|root| root.rel_dir.clone())
        .collect()
}

fn resolve_member_pattern(
    workspace_root_rel: &str,
    member: &str,
    descendant_root_rels: &[String],
) -> Vec<String> {
    let normalized = normalize_member_pattern(member);
    let pattern = join_rel(workspace_root_rel, &normalized);
    if !contains_glob_meta(&normalized) {
        return descendant_root_rels
            .iter()
            .filter(|rel_dir| *rel_dir == &pattern)
            .cloned()
            .collect();
    }

    let Ok(pattern) = Pattern::new(&pattern) else {
        return Vec::new();
    };
    descendant_root_rels
        .iter()
        .filter(|rel_dir| pattern.matches(rel_dir))
        .cloned()
        .collect()
}

fn classify_illegal_family_file(
    file: &G3RsTopologyWorkspaceFamilyFile,
    workspace_root_rel: &str,
    actual_children: &BTreeSet<String>,
    legal_member_roots: &BTreeSet<String>,
) -> Option<IllegalFamilyFilePlacement> {
    if file.kind == G3RsTopologyWorkspaceFamilyFileKind::CargoToml {
        return None;
    }

    let reason = if file.family == G3RsTopologyWorkspaceFamily::Fmt {
        match &file.attachment {
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { root_rel }
                if root_rel == workspace_root_rel =>
            {
                return None;
            }
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { .. }
            | G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot { .. } => {
                "fmt files must live at the validation root, not inside a workspace member or nested subdirectory.".to_owned()
            }
        }
    } else if file_is_legal_root_sidecar(file, workspace_root_rel) {
        return None;
    } else {
        match &file.attachment {
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { root_rel }
                if root_rel == workspace_root_rel =>
            {
                return None;
            }
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { root_rel }
                if legal_member_roots.contains(root_rel) =>
            {
                format!(
                    "`{}` is attached to legal workspace member `{}`. Workspace-local `{}` files must live at the workspace root `{}` instead of inside a member crate.",
                    file.rel_path,
                    display_dir(root_rel),
                    family_label(file.family),
                    display_dir(workspace_root_rel),
                )
            }
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { root_rel } => {
                let root_label = if actual_children.contains(root_rel) {
                    format!("illegal child root `{}`", display_dir(root_rel))
                } else {
                    format!("non-member root `{}`", display_dir(root_rel))
                };
                format!(
                    "`{}` is attached to {root_label}. Workspace-local `{}` files must live at the workspace root `{}`.",
                    file.rel_path,
                    family_label(file.family),
                    display_dir(workspace_root_rel),
                )
            }
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot { owner_rel, .. } => {
                format!(
                    "`{}` is nested under `{}`. Workspace-local `{}` files must live directly at the workspace root `{}` rather than in nested subdirectories.",
                    file.rel_path,
                    display_dir(owner_rel),
                    family_label(file.family),
                    display_dir(workspace_root_rel),
                )
            }
        }
    };

    Some(IllegalFamilyFilePlacement {
        family: file.family,
        rel_path: file.rel_path.clone(),
        reason,
    })
}

fn file_is_legal_root_sidecar(
    file: &G3RsTopologyWorkspaceFamilyFile,
    workspace_root_rel: &str,
) -> bool {
    match (&file.kind, &file.attachment) {
        (
            G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml
            | G3RsTopologyWorkspaceFamilyFileKind::CargoConfigLegacy
            | G3RsTopologyWorkspaceFamilyFileKind::CargoDenyToml
            | G3RsTopologyWorkspaceFamilyFileKind::MutantsToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
                root_rel,
                owner_rel,
            },
        ) => root_rel == workspace_root_rel && owner_rel == &join_rel(workspace_root_rel, ".cargo"),
        (
            G3RsTopologyWorkspaceFamilyFileKind::NextestToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
                root_rel,
                owner_rel,
            },
        ) => {
            root_rel == workspace_root_rel && owner_rel == &join_rel(workspace_root_rel, ".config")
        }
        _ => false,
    }
}

fn nearest_ancestor_workspace<'a>(
    rel_dir: &str,
    workspace_rels: &'a BTreeSet<String>,
) -> Option<&'a str> {
    workspace_rels
        .iter()
        .filter(|workspace_rel| {
            workspace_rel.as_str() != rel_dir && path_is_under(rel_dir, workspace_rel)
        })
        .max_by_key(|workspace_rel| workspace_rel.len())
        .map(std::string::String::as_str)
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn join_rel(parent: &str, child: &str) -> String {
    if parent.is_empty() {
        child.to_owned()
    } else if child.is_empty() {
        parent.to_owned()
    } else {
        format!("{parent}/{child}")
    }
}

fn member_pattern_escapes_root(member: &str) -> bool {
    member.starts_with('/')
        || member.starts_with('\\')
        || has_windows_drive_absolute_prefix(member)
        || member
            .split(|ch| ch == '/' || ch == '\\')
            .any(|segment| segment == "..")
}

fn has_windows_drive_absolute_prefix(member: &str) -> bool {
    let bytes = member.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes[2] == b'/' || bytes[2] == b'\\')
}

fn normalize_member_pattern(member: &str) -> String {
    member
        .split('/')
        .filter(|segment| !segment.is_empty() && *segment != ".")
        .collect::<Vec<_>>()
        .join("/")
}

fn contains_glob_meta(member: &str) -> bool {
    member.contains('*') || member.contains('?') || member.contains('[')
}

fn issue_sort_key(kind: &TopologyIssueKind) -> (&'static str, String) {
    match kind {
        TopologyIssueKind::NestedWorkspace {
            parent_workspace_rel,
        } => ("nested-workspace", parent_workspace_rel.clone()),
        TopologyIssueKind::UndeclaredWorkspaceMember { workspace_root_rel } => {
            ("undeclared-member", workspace_root_rel.clone())
        }
        TopologyIssueKind::ExtraWorkspaceMember {
            workspace_root_rel,
            member_pattern,
        } => (
            "extra-member",
            format!("{workspace_root_rel}:{member_pattern}"),
        ),
        TopologyIssueKind::WorkspaceMemberPathEscapesRoot {
            workspace_root_rel,
            member_pattern,
        } => (
            "member-path-escape",
            format!("{workspace_root_rel}:{member_pattern}"),
        ),
    }
}

pub(crate) fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

pub(crate) fn family_label(family: G3RsTopologyWorkspaceFamily) -> &'static str {
    match family {
        G3RsTopologyWorkspaceFamily::Toolchain => "toolchain",
        G3RsTopologyWorkspaceFamily::Fmt => "fmt",
        G3RsTopologyWorkspaceFamily::Clippy => "clippy",
        G3RsTopologyWorkspaceFamily::Deny => "deny",
        G3RsTopologyWorkspaceFamily::Cargo => "cargo",
        G3RsTopologyWorkspaceFamily::Deps => "deps",
        G3RsTopologyWorkspaceFamily::Garde => "garde",
        G3RsTopologyWorkspaceFamily::Release => "release",
        G3RsTopologyWorkspaceFamily::Test => "test",
    }
}
