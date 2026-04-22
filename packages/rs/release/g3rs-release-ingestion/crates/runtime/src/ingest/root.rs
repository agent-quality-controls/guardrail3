use g3rs_release_types::G3RsReleaseInputFailure;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use super::collect::{ParsedCrate, RootCargo, push_all_failures};

pub(super) fn parse_root_cargo(
    crawl: &G3RsWorkspaceCrawl,
    config_failures: &mut Vec<G3RsReleaseInputFailure>,
    filetree_failures: &mut Vec<G3RsReleaseInputFailure>,
    source_failures: &mut Vec<G3RsReleaseInputFailure>,
) -> Option<RootCargo> {
    let Some(entry) = crate::select::select_cargo_toml(crawl) else {
        push_all_failures(
            config_failures,
            filetree_failures,
            source_failures,
            "Cargo.toml",
            "Release workspace root is missing Cargo.toml.",
        );
        return None;
    };
    if !entry.readable {
        push_all_failures(
            config_failures,
            filetree_failures,
            source_failures,
            &entry.path.rel_path,
            "Failed to read root Cargo.toml for release checks: file is not readable.",
        );
        return None;
    }

    let content = match crate::parse::read_to_string(&entry.path.abs_path) {
        Ok(content) => content,
        Err(error) => {
            push_all_failures(
                config_failures,
                filetree_failures,
                source_failures,
                &entry.path.rel_path,
                format!("Failed to read root Cargo.toml for release checks: {error}"),
            );
            return None;
        }
    };

    let cargo = match crate::parse::parse_cargo_toml(&content, &entry.path.abs_path) {
        Ok(cargo) => cargo,
        Err(error) => {
            push_all_failures(
                config_failures,
                filetree_failures,
                source_failures,
                &entry.path.rel_path,
                format!("Failed to parse root Cargo.toml for release checks: {error}"),
            );
            return None;
        }
    };

    Some(RootCargo {
        cargo,
        cargo_abs_path: entry.path.abs_path.clone(),
    })
}

pub(super) fn collect_parsed_crates(
    crawl: &G3RsWorkspaceCrawl,
    root: Option<&RootCargo>,
    config_failures: &mut Vec<G3RsReleaseInputFailure>,
    filetree_failures: &mut Vec<G3RsReleaseInputFailure>,
    source_failures: &mut Vec<G3RsReleaseInputFailure>,
) -> Vec<ParsedCrate> {
    let Some(root) = root else {
        return Vec::new();
    };

    let mut crates = Vec::new();
    if root.cargo.package.is_some() {
        crates.push(ParsedCrate {
            rel_dir: String::new(),
            cargo_rel_path: "Cargo.toml".to_owned(),
            cargo_abs_path: root.cargo_abs_path.clone(),
            cargo: root.cargo.clone(),
        });
    }

    let member_rels = match root.cargo.workspace.as_ref() {
        Some(_) => match crate::select::collect_member_rels(crawl, &root.cargo) {
            Ok(member_rels) => member_rels,
            Err(reason) => {
                push_all_failures(
                    config_failures,
                    filetree_failures,
                    source_failures,
                    "Cargo.toml",
                    format!("Failed to normalize workspace members for release checks: {reason}"),
                );
                Vec::new()
            }
        },
        None => Vec::new(),
    };

    for member_rel in member_rels {
        if member_rel.is_empty() {
            continue;
        }
        let Some(entry) = crate::select::select_member_manifest(crawl, &member_rel) else {
            push_all_failures(
                config_failures,
                filetree_failures,
                source_failures,
                crate::select::member_manifest_rel_path(&member_rel),
                format!("Declared workspace member `{member_rel}` is missing Cargo.toml."),
            );
            continue;
        };
        if !entry.readable {
            push_all_failures(
                config_failures,
                filetree_failures,
                source_failures,
                &entry.path.rel_path,
                "Failed to read member Cargo.toml for release checks: file is not readable.",
            );
            continue;
        }

        let content = match crate::parse::read_to_string(&entry.path.abs_path) {
            Ok(content) => content,
            Err(error) => {
                push_all_failures(
                    config_failures,
                    filetree_failures,
                    source_failures,
                    &entry.path.rel_path,
                    format!("Failed to read member Cargo.toml for release checks: {error}"),
                );
                continue;
            }
        };

        let cargo = match crate::parse::parse_cargo_toml(&content, &entry.path.abs_path) {
            Ok(cargo) => cargo,
            Err(error) => {
                push_all_failures(
                    config_failures,
                    filetree_failures,
                    source_failures,
                    &entry.path.rel_path,
                    format!("Failed to parse member Cargo.toml for release checks: {error}"),
                );
                continue;
            }
        };

        crates.push(ParsedCrate {
            rel_dir: member_rel,
            cargo_rel_path: entry.path.rel_path.clone(),
            cargo_abs_path: entry.path.abs_path.clone(),
            cargo,
        });
    }

    crates.sort_by(|left, right| left.cargo_rel_path.cmp(&right.cargo_rel_path));
    crates
}
