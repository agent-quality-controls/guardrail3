/// Select config entries from a workspace crawl.
use cargo_toml_parser::CargoToml;
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};
use glob::Pattern;

/// Find `Cargo.toml` at the workspace root.
pub(crate) fn select_workspace_cargo_toml(
    crawl: &G3RsWorkspaceCrawl,
) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("Cargo.toml")
}

/// Find `guardrail3-rs.toml` at the workspace root.
pub(crate) fn select_workspace_guardrail3_rs_toml(
    crawl: &G3RsWorkspaceCrawl,
) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("guardrail3-rs.toml")
}

/// Find member `Cargo.toml` files declared by `[workspace].members`.
pub(crate) fn select_member_cargo_tomls<'a>(
    crawl: &'a G3RsWorkspaceCrawl,
    workspace_cargo: &CargoToml,
) -> Result<Vec<&'a G3RsWorkspaceEntry>, String> {
    let patterns = workspace_cargo
        .workspace
        .as_ref()
        .map(|workspace| {
            workspace
                .members
                .iter()
                .map(|pattern| {
                    Pattern::new(pattern).map_err(|err| {
                        format!("invalid workspace member pattern `{pattern}`: {err}")
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let mut members = crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == G3RsWorkspaceEntryKind::File
                && entry.path.rel_path != "Cargo.toml"
                && entry.path.rel_path.ends_with("/Cargo.toml")
        })
        .filter(|entry| {
            let Some(member_dir) = entry.path.rel_path.strip_suffix("/Cargo.toml") else {
                return false;
            };
            patterns.iter().any(|pattern| pattern.matches(member_dir))
        })
        .collect::<Vec<_>>();
    members.sort_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path));
    Ok(members)
}
