use g3rs_topology_types::G3RsTopologyWorkspaceFamily;

/// Returns a friendly directory label, mapping the empty string to `"."`.
pub(crate) const fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

/// Returns the canonical label for a workspace family used in rule messages.
pub(crate) const fn family_label(family: G3RsTopologyWorkspaceFamily) -> &'static str {
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
