use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use super::dependency_entries::policy_for_member;
use super::{InputFailureFacts, LockfileFacts, MemberFacts, ParsedGuardrail, WorkspaceFacts};

pub(super) fn collect_lockfiles(
    tree: &ProjectTree,
    workspaces: &[WorkspaceFacts],
    members: &[MemberFacts],
    parsed_guardrail: Option<&ParsedGuardrail>,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<LockfileFacts> {
    let mut root_profiles = BTreeMap::new();
    let mut reported_gitignore_failures = BTreeSet::new();
    for member in members {
        let _ = root_profiles
            .entry(member.rel_dir.clone())
            .or_insert_with(|| member.profile_name.clone());
    }

    let mut root_rels = BTreeSet::new();
    for workspace in workspaces {
        let _ = root_rels.insert(workspace.root_rel_dir.clone());
        let _ = root_profiles
            .entry(workspace.root_rel_dir.clone())
            .or_insert_with(|| policy_for_member(&workspace.root_rel_dir, parsed_guardrail).0);
    }
    for member in members {
        if member.workspace_root_rel_dir.is_none()
            && !is_nested_under_any_workspace_root(&member.rel_dir, workspaces)
        {
            let _ = root_rels.insert(member.rel_dir.clone());
        }
    }

    root_rels
        .into_iter()
        .map(|root_rel_dir| {
            let cargo_lock_rel_path = if root_rel_dir.is_empty() {
                "Cargo.lock".to_owned()
            } else {
                format!("{root_rel_dir}/Cargo.lock")
            };
            let (cargo_lock_ignored, gitignore_rel_path) = lockfile_ignore_status(
                tree,
                &root_rel_dir,
                &cargo_lock_rel_path,
                input_failures,
                &mut reported_gitignore_failures,
            );
            LockfileFacts {
                root_rel_dir: root_rel_dir.clone(),
                cargo_lock_rel_path: cargo_lock_rel_path.clone(),
                cargo_lock_exists: tree.file_exists(&cargo_lock_rel_path),
                cargo_lock_ignored,
                gitignore_rel_path,
                profile_name: root_profiles.get(&root_rel_dir).cloned().flatten(),
            }
        })
        .collect()
}

fn is_nested_under_any_workspace_root(rel_dir: &str, workspaces: &[WorkspaceFacts]) -> bool {
    workspaces.iter().any(|workspace| {
        let root_rel_dir = workspace.root_rel_dir.as_str();
        if root_rel_dir.is_empty() {
            !rel_dir.is_empty()
        } else {
            rel_dir
                .strip_prefix(root_rel_dir)
                .is_some_and(|rest| rest.starts_with('/'))
        }
    })
}

fn lockfile_ignore_status(
    tree: &ProjectTree,
    root_rel_dir: &str,
    cargo_lock_rel_path: &str,
    input_failures: &mut Vec<InputFailureFacts>,
    reported_gitignore_failures: &mut BTreeSet<String>,
) -> (bool, Option<String>) {
    let mut ignored = false;
    let mut source = None;

    for gitignore_rel_path in ancestor_gitignore_rels(root_rel_dir) {
        let Some(content) = tree.file_content(&gitignore_rel_path) else {
            if tree.file_exists(&gitignore_rel_path)
                && reported_gitignore_failures.insert(gitignore_rel_path.clone())
            {
                input_failures.push(InputFailureFacts {
                    rel_path: gitignore_rel_path.clone(),
                    message: "Failed to read `.gitignore` for Cargo.lock masking checks."
                        .to_owned(),
                });
            }
            continue;
        };
        for line in content.lines() {
            if let Some(next_ignored) =
                cargo_lock_ignore_match(line, &gitignore_rel_path, cargo_lock_rel_path)
            {
                ignored = next_ignored;
                source = if ignored {
                    Some(gitignore_rel_path.clone())
                } else {
                    None
                };
            }
        }
    }

    (ignored, source)
}

fn ancestor_gitignore_rels(root_rel_dir: &str) -> Vec<String> {
    let mut rels = vec![".gitignore".to_owned()];
    if root_rel_dir.is_empty() {
        return rels;
    }

    let mut current = String::new();
    for segment in root_rel_dir.split('/') {
        current = if current.is_empty() {
            segment.to_owned()
        } else {
            format!("{current}/{segment}")
        };
        rels.push(format!("{current}/.gitignore"));
    }
    rels
}

fn cargo_lock_ignore_match(
    line: &str,
    gitignore_rel_path: &str,
    cargo_lock_rel_path: &str,
) -> Option<bool> {
    let gitignore_dir_rel = gitignore_rel_path
        .strip_suffix("/.gitignore")
        .unwrap_or_default();
    let candidate_rel = if gitignore_dir_rel.is_empty() {
        cargo_lock_rel_path.to_owned()
    } else if let Some(rest) = cargo_lock_rel_path.strip_prefix(&format!("{gitignore_dir_rel}/")) {
        rest.to_owned()
    } else {
        return None;
    };

    let basename = "Cargo.lock";
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let (ignored, pattern_text) = if let Some(pattern) = trimmed.strip_prefix('!') {
        (false, pattern)
    } else {
        (true, trimmed)
    };
    let anchored = pattern_text.starts_with('/');
    let normalized = pattern_text.trim_start_matches('/');
    if normalized.is_empty() {
        return None;
    }

    let matched = if normalized == "Cargo.lock" {
        if anchored {
            candidate_rel == basename
        } else {
            true
        }
    } else if !normalized.contains('/') {
        glob::Pattern::new(normalized).ok().is_some_and(|pattern| {
            if anchored {
                pattern.matches(&candidate_rel)
            } else {
                pattern.matches(basename)
            }
        })
    } else {
        glob::Pattern::new(normalized)
            .ok()
            .is_some_and(|pattern| pattern.matches(&candidate_rel))
    };

    matched.then_some(ignored)
}
