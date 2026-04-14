use cargo_toml_parser::CargoToml;
use g3rs_topology_ingestion_types::{
    G3RsTopologyFileTreeChecksInput, G3RsTopologyIngestionError as IngestionError,
};
use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyDescendantCargoRoot,
    G3RsTopologyFileTreeInputFailure, G3RsTopologyWorkspaceFamily, G3RsTopologyWorkspaceFamilyFile,
    G3RsTopologyWorkspaceFamilyFileAttachment, G3RsTopologyWorkspaceFamilyFileKind,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use crate::view::CrawlView;

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsTopologyFileTreeChecksInput, IngestionError> {
    let view = CrawlView::new(crawl);
    let workspace_manifest = parse_required_root_manifest(&view)?;
    let (descendant_cargo_roots, input_failures) = collect_descendant_cargo_roots(&view);
    let family_files = collect_family_files(&view, &descendant_cargo_roots);

    Ok(G3RsTopologyFileTreeChecksInput {
        workspace_root_rel_dir: String::new(),
        workspace_root_cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_manifest,
        descendant_cargo_roots,
        family_files,
        input_failures,
    })
}

fn parse_required_root_manifest(view: &CrawlView<'_>) -> Result<CargoToml, IngestionError> {
    let root_path = view.root_abs_path().join("Cargo.toml");
    let Some(entry) = view.entry("Cargo.toml") else {
        return Err(IngestionError::Unreadable {
            path: root_path,
            reason: "file is missing from crawl".to_owned(),
        });
    };
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = view
        .read_file("Cargo.toml")
        .map_err(|err| IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: err.to_string(),
        })?;
    let parsed = cargo_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;
    if parsed.workspace.is_none() {
        return Err(IngestionError::RootManifestNotWorkspace {
            path: entry.path.abs_path.clone(),
        });
    }
    Ok(parsed)
}

fn collect_descendant_cargo_roots(
    view: &CrawlView<'_>,
) -> (
    Vec<G3RsTopologyDescendantCargoRoot>,
    Vec<G3RsTopologyFileTreeInputFailure>,
) {
    let mut roots = Vec::new();
    let mut failures = Vec::new();

    let mut cargo_entries = view
        .included_file_entries()
        .filter(|entry| entry.path.rel_path != "Cargo.toml")
        .filter(|entry| entry.path.rel_path.ends_with("/Cargo.toml"))
        .filter(|entry| !is_excluded_live_topology_path(&entry.path.rel_path))
        .collect::<Vec<_>>();
    cargo_entries.sort_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path));

    for entry in cargo_entries {
        let cargo_rel_path = entry.path.rel_path.clone();
        let rel_dir = parent_dir(&cargo_rel_path).to_owned();

        if !entry.readable {
            roots.push(G3RsTopologyDescendantCargoRoot {
                rel_dir,
                cargo_rel_path: cargo_rel_path.clone(),
                manifest_kind: None,
            });
            failures.push(G3RsTopologyFileTreeInputFailure {
                rel_path: cargo_rel_path,
                message: "file is not readable".to_owned(),
            });
            continue;
        }

        let content = match view.read_file(&cargo_rel_path) {
            Ok(content) => content,
            Err(err) => {
                roots.push(G3RsTopologyDescendantCargoRoot {
                    rel_dir,
                    cargo_rel_path: cargo_rel_path.clone(),
                    manifest_kind: None,
                });
                failures.push(G3RsTopologyFileTreeInputFailure {
                    rel_path: cargo_rel_path,
                    message: err.to_string(),
                });
                continue;
            }
        };

        match cargo_toml_parser::parse(&content) {
            Ok(parsed) => {
                roots.push(G3RsTopologyDescendantCargoRoot {
                    rel_dir,
                    cargo_rel_path,
                    manifest_kind: classify_manifest_kind(&parsed),
                });
            }
            Err(err) => {
                roots.push(G3RsTopologyDescendantCargoRoot {
                    rel_dir,
                    cargo_rel_path: cargo_rel_path.clone(),
                    manifest_kind: None,
                });
                failures.push(G3RsTopologyFileTreeInputFailure {
                    rel_path: cargo_rel_path,
                    message: err.to_string(),
                });
            }
        }
    }

    (roots, failures)
}

fn classify_manifest_kind(parsed: &CargoToml) -> Option<G3RsTopologyCargoManifestKind> {
    match (
        parsed.workspace.is_some(),
        parsed.package.is_some() || parsed.project.is_some(),
    ) {
        (true, true) => Some(G3RsTopologyCargoManifestKind::Hybrid),
        (true, false) => Some(G3RsTopologyCargoManifestKind::Workspace),
        (false, true) => Some(G3RsTopologyCargoManifestKind::Package),
        (false, false) => None,
    }
}

fn collect_family_files(
    view: &CrawlView<'_>,
    descendant_cargo_roots: &[G3RsTopologyDescendantCargoRoot],
) -> Vec<G3RsTopologyWorkspaceFamilyFile> {
    let root_rels = std::iter::once(String::new())
        .chain(
            descendant_cargo_roots
                .iter()
                .map(|root| root.rel_dir.clone()),
        )
        .collect::<Vec<_>>();
    let mut files = view
        .included_file_entries()
        .filter(|entry| !is_excluded_live_topology_path(&entry.path.rel_path))
        .flat_map(|entry| classify_family_file(&entry.path.rel_path, &root_rels))
        .collect::<Vec<_>>();
    files.sort_by(|left, right| {
        left.family
            .cmp(&right.family)
            .then(left.rel_path.cmp(&right.rel_path))
            .then(left.kind.cmp(&right.kind))
    });
    files.dedup();
    files
}

fn classify_family_file(
    rel_path: &str,
    root_rels: &[String],
) -> Vec<G3RsTopologyWorkspaceFamilyFile> {
    let file = |family, kind| family_file(family, rel_path, kind, root_rels);

    if rel_path == "Cargo.toml" || rel_path.ends_with("/Cargo.toml") {
        return vec![
            file(
                G3RsTopologyWorkspaceFamily::Toolchain,
                G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Clippy,
                G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Deny,
                G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Cargo,
                G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Deps,
                G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Garde,
                G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Release,
                G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
            ),
        ];
    }
    if rel_path == "guardrail3-rs.toml" || rel_path.ends_with("/guardrail3-rs.toml") {
        return vec![
            file(
                G3RsTopologyWorkspaceFamily::Cargo,
                G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Deps,
                G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Garde,
                G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml,
            ),
        ];
    }
    if rel_path == "rustfmt.toml" || rel_path.ends_with("/rustfmt.toml") {
        return vec![file(
            G3RsTopologyWorkspaceFamily::Fmt,
            G3RsTopologyWorkspaceFamilyFileKind::RustfmtToml,
        )];
    }
    if rel_path == ".rustfmt.toml" || rel_path.ends_with("/.rustfmt.toml") {
        return vec![file(
            G3RsTopologyWorkspaceFamily::Fmt,
            G3RsTopologyWorkspaceFamilyFileKind::DotRustfmtToml,
        )];
    }
    if rel_path == "rust-toolchain.toml" || rel_path.ends_with("/rust-toolchain.toml") {
        return vec![file(
            G3RsTopologyWorkspaceFamily::Toolchain,
            G3RsTopologyWorkspaceFamilyFileKind::RustToolchainToml,
        )];
    }
    if rel_path == "rust-toolchain" || rel_path.ends_with("/rust-toolchain") {
        return vec![file(
            G3RsTopologyWorkspaceFamily::Toolchain,
            G3RsTopologyWorkspaceFamilyFileKind::RustToolchainLegacy,
        )];
    }
    if rel_path == "clippy.toml" || rel_path.ends_with("/clippy.toml") {
        return vec![
            file(
                G3RsTopologyWorkspaceFamily::Clippy,
                G3RsTopologyWorkspaceFamilyFileKind::ClippyToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Garde,
                G3RsTopologyWorkspaceFamilyFileKind::ClippyToml,
            ),
        ];
    }
    if rel_path == ".clippy.toml" || rel_path.ends_with("/.clippy.toml") {
        return vec![
            file(
                G3RsTopologyWorkspaceFamily::Clippy,
                G3RsTopologyWorkspaceFamilyFileKind::ClippyDotToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Garde,
                G3RsTopologyWorkspaceFamilyFileKind::ClippyDotToml,
            ),
        ];
    }
    if rel_path == ".cargo/config.toml" || rel_path.ends_with("/.cargo/config.toml") {
        return vec![
            file(
                G3RsTopologyWorkspaceFamily::Clippy,
                G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Garde,
                G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml,
            ),
        ];
    }
    if rel_path == ".cargo/config" || rel_path.ends_with("/.cargo/config") {
        return vec![
            file(
                G3RsTopologyWorkspaceFamily::Clippy,
                G3RsTopologyWorkspaceFamilyFileKind::CargoConfigLegacy,
            ),
            file(
                G3RsTopologyWorkspaceFamily::Garde,
                G3RsTopologyWorkspaceFamilyFileKind::CargoConfigLegacy,
            ),
        ];
    }
    if rel_path == "deny.toml" || rel_path.ends_with("/deny.toml") {
        if rel_path == ".cargo/deny.toml" || rel_path.ends_with("/.cargo/deny.toml") {
            return vec![file(
                G3RsTopologyWorkspaceFamily::Deny,
                G3RsTopologyWorkspaceFamilyFileKind::CargoDenyToml,
            )];
        }
        return vec![file(
            G3RsTopologyWorkspaceFamily::Deny,
            G3RsTopologyWorkspaceFamilyFileKind::DenyToml,
        )];
    }
    if rel_path == ".deny.toml" || rel_path.ends_with("/.deny.toml") {
        return vec![file(
            G3RsTopologyWorkspaceFamily::Deny,
            G3RsTopologyWorkspaceFamilyFileKind::DenyDotToml,
        )];
    }
    if rel_path == "release-plz.toml" || rel_path.ends_with("/release-plz.toml") {
        return vec![file(
            G3RsTopologyWorkspaceFamily::Release,
            G3RsTopologyWorkspaceFamilyFileKind::ReleasePlzToml,
        )];
    }
    if rel_path == "cliff.toml" || rel_path.ends_with("/cliff.toml") {
        return vec![file(
            G3RsTopologyWorkspaceFamily::Release,
            G3RsTopologyWorkspaceFamilyFileKind::CliffToml,
        )];
    }
    if rel_path == ".cargo/mutants.toml" || rel_path.ends_with("/.cargo/mutants.toml") {
        return vec![file(
            G3RsTopologyWorkspaceFamily::Test,
            G3RsTopologyWorkspaceFamilyFileKind::MutantsToml,
        )];
    }
    if rel_path == ".config/nextest.toml" || rel_path.ends_with("/.config/nextest.toml") {
        return vec![file(
            G3RsTopologyWorkspaceFamily::Test,
            G3RsTopologyWorkspaceFamilyFileKind::NextestToml,
        )];
    }

    Vec::new()
}

fn family_file(
    family: G3RsTopologyWorkspaceFamily,
    rel_path: &str,
    kind: G3RsTopologyWorkspaceFamilyFileKind,
    root_rels: &[String],
) -> G3RsTopologyWorkspaceFamilyFile {
    G3RsTopologyWorkspaceFamilyFile {
        family,
        rel_path: rel_path.to_owned(),
        kind,
        attachment: attach_owner_rel(&logical_owner_rel(rel_path, kind), root_rels),
    }
}

fn parent_dir(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

fn logical_owner_rel(rel_path: &str, kind: G3RsTopologyWorkspaceFamilyFileKind) -> String {
    match kind {
        G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml
        | G3RsTopologyWorkspaceFamilyFileKind::CargoConfigLegacy
        | G3RsTopologyWorkspaceFamilyFileKind::CargoDenyToml
        | G3RsTopologyWorkspaceFamilyFileKind::MutantsToml => parent_dir(rel_path)
            .strip_suffix("/.cargo")
            .unwrap_or("")
            .trim_matches('/')
            .to_owned(),
        G3RsTopologyWorkspaceFamilyFileKind::NextestToml => parent_dir(rel_path)
            .strip_suffix("/.config")
            .unwrap_or("")
            .trim_matches('/')
            .to_owned(),
        _ => parent_dir(rel_path).to_owned(),
    }
}

fn attach_owner_rel(
    owner_rel: &str,
    root_rels: &[String],
) -> G3RsTopologyWorkspaceFamilyFileAttachment {
    if root_rels.iter().any(|root_rel| root_rel == owner_rel) {
        return G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot {
            root_rel: owner_rel.to_owned(),
        };
    }

    if let Some(root_rel) = nearest_ancestor_root(owner_rel, root_rels) {
        return G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
            root_rel: root_rel.to_owned(),
            owner_rel: owner_rel.to_owned(),
        };
    }

    panic!("topology family file owner `{owner_rel}` is outside the pointed workspace roots");
}

fn nearest_ancestor_root<'a>(owner_rel: &str, root_rels: &'a [String]) -> Option<&'a str> {
    root_rels
        .iter()
        .filter_map(|root_rel| {
            if path_is_under(owner_rel, root_rel) {
                Some(root_rel.as_str())
            } else {
                None
            }
        })
        .max_by_key(|root_rel| root_rel.len())
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn is_excluded_live_topology_path(rel_path: &str) -> bool {
    path_contains_sequence(rel_path, &["tests", "fixtures"])
        || path_contains_sequence(rel_path, &["tests", "snapshots"])
        || path_contains_segment(rel_path, "target")
        || path_contains_sequence(rel_path, &[".claude", "worktrees"])
}

fn path_contains_sequence(rel_path: &str, sequence: &[&str]) -> bool {
    let segments = rel_path.split('/').collect::<Vec<_>>();
    segments
        .windows(sequence.len())
        .any(|window| window == sequence)
}

fn path_contains_segment(rel_path: &str, segment: &str) -> bool {
    rel_path.split('/').any(|part| part == segment)
}
