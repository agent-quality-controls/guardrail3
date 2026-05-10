#![expect(
    clippy::too_many_lines,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::BTreeSet;

use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyFileTreeChecksInput,
    G3RsTopologyWorkspaceMemberIssueInput, G3RsTopologyWorkspaceMemberIssueKind,
};

use super::FileTreeFacts;
use super::illegal_family_files::classify_illegal_family_file;
use super::membership::{
    collect_actual_children, collect_escaping_member_patterns, collect_member_patterns,
};
use super::nested::{collect_nested_guardrail3_rs_toml_issues, collect_nested_workspace_issues};
use super::paths::{join_rel, membership_issue_sort_key};

/// `collect` function.
pub(crate) fn collect(input: &G3RsTopologyFileTreeChecksInput) -> FileTreeFacts {
    let workspace_root_rel = input.workspace_root_rel_dir.as_str();
    let workspace_roots = std::iter::once(workspace_root_rel.to_owned())
        .chain(input.descendant_cargo_roots.iter().filter_map(|root| {
            if matches!(
                root.manifest_kind,
                Some(
                    G3RsTopologyCargoManifestKind::Workspace
                        | G3RsTopologyCargoManifestKind::Hybrid
                )
            ) {
                Some(root.rel_dir.clone())
            } else {
                None
            }
        }))
        .collect::<BTreeSet<_>>();

    let mut nested_guardrail3_rs_tomls = collect_nested_guardrail3_rs_toml_issues(input);
    nested_guardrail3_rs_tomls.sort_by(|left, right| {
        left.guardrail3_rs_toml_rel_path
            .cmp(&right.guardrail3_rs_toml_rel_path)
            .then(
                left.outer_adopted_unit_rel
                    .cmp(&right.outer_adopted_unit_rel),
            )
    });
    nested_guardrail3_rs_tomls.dedup_by(|left, right| {
        left.rel_dir == right.rel_dir
            && left.guardrail3_rs_toml_rel_path == right.guardrail3_rs_toml_rel_path
            && left.outer_adopted_unit_rel == right.outer_adopted_unit_rel
    });

    let mut nested_workspaces = collect_nested_workspace_issues(input, &workspace_roots);
    nested_workspaces.sort_by(|left, right| {
        left.cargo_rel_path
            .cmp(&right.cargo_rel_path)
            .then(left.parent_workspace_rel.cmp(&right.parent_workspace_rel))
    });
    nested_workspaces.dedup_by(|left, right| {
        left.rel_dir == right.rel_dir
            && left.cargo_rel_path == right.cargo_rel_path
            && left.parent_workspace_rel == right.parent_workspace_rel
    });

    let mut escaping_member_paths = collect_escaping_member_patterns(input)
        .into_iter()
        .map(
            |member_pattern| g3rs_topology_types::G3RsTopologyEscapingWorkspaceMemberPathInput {
                cargo_rel_path: input.workspace_root_cargo_rel_path.clone(),
                workspace_root_rel: workspace_root_rel.to_owned(),
                member_pattern,
            },
        )
        .collect::<Vec<_>>();
    escaping_member_paths.sort_by(|left, right| {
        left.cargo_rel_path
            .cmp(&right.cargo_rel_path)
            .then(left.workspace_root_rel.cmp(&right.workspace_root_rel))
            .then(left.member_pattern.cmp(&right.member_pattern))
    });
    escaping_member_paths.dedup_by(|left, right| {
        left.cargo_rel_path == right.cargo_rel_path
            && left.workspace_root_rel == right.workspace_root_rel
            && left.member_pattern == right.member_pattern
    });

    let actual_children = collect_actual_children(input, &workspace_roots);
    let member_patterns = collect_member_patterns(input, &actual_children);

    let mut membership_issues = Vec::new();
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
        membership_issues.push(G3RsTopologyWorkspaceMemberIssueInput {
            rel_dir: child.clone(),
            cargo_rel_path: join_rel(child, "Cargo.toml"),
            kind: G3RsTopologyWorkspaceMemberIssueKind::Undeclared {
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
        membership_issues.push(G3RsTopologyWorkspaceMemberIssueInput {
            rel_dir: workspace_root_rel.to_owned(),
            cargo_rel_path: input.workspace_root_cargo_rel_path.clone(),
            kind: G3RsTopologyWorkspaceMemberIssueKind::Extra {
                workspace_root_rel: workspace_root_rel.to_owned(),
                member_pattern: member.raw.clone(),
            },
        });
    }

    membership_issues.sort_by(|left, right| {
        left.cargo_rel_path.cmp(&right.cargo_rel_path).then(
            membership_issue_sort_key(&left.kind).cmp(&membership_issue_sort_key(&right.kind)),
        )
    });
    membership_issues.dedup_by(|left, right| {
        left.rel_dir == right.rel_dir
            && left.cargo_rel_path == right.cargo_rel_path
            && membership_issue_sort_key(&left.kind) == membership_issue_sort_key(&right.kind)
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

    FileTreeFacts {
        nested_workspaces,
        nested_guardrail3_rs_tomls,
        membership_issues,
        escaping_member_paths,
        illegal_family_files,
    }
}
