use cargo_toml_parser::parse;
use g3rs_topology_file_tree_checks_types::G3RsTopologyFileTreeChecksInput;
use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyDescendantCargoRoot, G3RsTopologyWorkspaceFamily,
    G3RsTopologyWorkspaceFamilyFile, G3RsTopologyWorkspaceFamilyFileAttachment,
    G3RsTopologyWorkspaceFamilyFileKind,
};
use guardrail3_check_types::G3CheckResult;

pub(crate) fn input(
    workspace_toml: &str,
    descendants: Vec<(&str, Option<G3RsTopologyCargoManifestKind>)>,
    family_files: Vec<G3RsTopologyWorkspaceFamilyFile>,
) -> G3RsTopologyFileTreeChecksInput {
    G3RsTopologyFileTreeChecksInput {
        workspace_root_rel_dir: String::new(),
        workspace_root_cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_manifest: parse(workspace_toml).expect("workspace parse"),
        descendant_cargo_roots: descendants
            .into_iter()
            .map(|(rel_dir, manifest_kind)| G3RsTopologyDescendantCargoRoot {
                rel_dir: rel_dir.to_owned(),
                cargo_rel_path: format!("{rel_dir}/Cargo.toml"),
                manifest_kind,
            })
            .collect(),
        family_files,
        input_failures: Vec::new(),
    }
}

pub(crate) fn family_file(
    family: G3RsTopologyWorkspaceFamily,
    rel_path: &str,
    kind: G3RsTopologyWorkspaceFamilyFileKind,
    attachment: G3RsTopologyWorkspaceFamilyFileAttachment,
) -> G3RsTopologyWorkspaceFamilyFile {
    G3RsTopologyWorkspaceFamilyFile {
        family,
        rel_path: rel_path.to_owned(),
        kind,
        attachment,
    }
}

pub(crate) fn titles(results: &[G3CheckResult], rule_id: &str) -> Vec<String> {
    results
        .iter()
        .filter(|result| result.id() == rule_id)
        .map(|result| result.title().to_owned())
        .collect()
}
