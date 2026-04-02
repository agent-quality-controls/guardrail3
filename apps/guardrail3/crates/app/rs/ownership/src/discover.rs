use std::collections::BTreeSet;

use guardrail3_app_rs_placement::{RustRootPlacementFacts, is_excluded_live_root_dir};
use guardrail3_domain_project_tree::ProjectTreeDiscovery;
use guardrail3_validation_model::RustValidateFamily;

use crate::kinds::{
    RustFamilyFileAttachment, RustFamilyFileFact, RustFamilyFileKind, RustOwnedSurfaceFacts,
};

pub(super) fn collect(
    tree: &dyn ProjectTreeDiscovery,
    placement: &RustRootPlacementFacts,
) -> RustOwnedSurfaceFacts {
    let root_rels = placement
        .roots()
        .iter()
        .map(|root| root.rel_dir().to_owned())
        .collect::<Vec<_>>();
    let mut seen = BTreeSet::<(RustValidateFamily, String)>::new();
    let mut family_files = Vec::new();

    collect_toolchain_files(tree, &root_rels, &mut seen, &mut family_files);
    collect_fmt_files(tree, &root_rels, &mut seen, &mut family_files);
    collect_clippy_files(tree, &root_rels, &mut seen, &mut family_files);
    collect_deny_files(tree, &root_rels, &mut seen, &mut family_files);
    collect_cargo_and_policy_files(tree, &root_rels, &mut seen, &mut family_files);
    collect_release_files(tree, &root_rels, &mut seen, &mut family_files);

    family_files.sort_by(|left, right| {
        left.family()
            .cmp(&right.family())
            .then(left.rel_path().cmp(right.rel_path()))
            .then(left.kind().cmp(&right.kind()))
    });

    RustOwnedSurfaceFacts::new(family_files)
}

fn collect_fmt_files(
    tree: &dyn ProjectTreeDiscovery,
    root_rels: &[String],
    seen: &mut BTreeSet<(RustValidateFamily, String)>,
    out: &mut Vec<RustFamilyFileFact>,
) {
    collect_root_and_dir_file(
        tree,
        root_rels,
        seen,
        out,
        RustValidateFamily::Fmt,
        RustFamilyFileKind::RustfmtToml,
        "rustfmt.toml",
    );
    collect_root_and_dir_file(
        tree,
        root_rels,
        seen,
        out,
        RustValidateFamily::Fmt,
        RustFamilyFileKind::DotRustfmtToml,
        ".rustfmt.toml",
    );
}

fn collect_toolchain_files(
    tree: &dyn ProjectTreeDiscovery,
    root_rels: &[String],
    seen: &mut BTreeSet<(RustValidateFamily, String)>,
    out: &mut Vec<RustFamilyFileFact>,
) {
    collect_root_and_dir_file(
        tree,
        root_rels,
        seen,
        out,
        RustValidateFamily::Toolchain,
        RustFamilyFileKind::RustToolchainToml,
        "rust-toolchain.toml",
    );
    collect_root_and_dir_file(
        tree,
        root_rels,
        seen,
        out,
        RustValidateFamily::Toolchain,
        RustFamilyFileKind::RustToolchainLegacy,
        "rust-toolchain",
    );
}

fn collect_clippy_files(
    tree: &dyn ProjectTreeDiscovery,
    root_rels: &[String],
    seen: &mut BTreeSet<(RustValidateFamily, String)>,
    out: &mut Vec<RustFamilyFileFact>,
) {
    for family in [RustValidateFamily::Clippy, RustValidateFamily::Garde] {
        collect_root_and_dir_file(
            tree,
            root_rels,
            seen,
            out,
            family,
            RustFamilyFileKind::ClippyToml,
            "clippy.toml",
        );
        collect_root_and_dir_file(
            tree,
            root_rels,
            seen,
            out,
            family,
            RustFamilyFileKind::ClippyDotToml,
            ".clippy.toml",
        );
        collect_cargo_dir_file(
            tree,
            root_rels,
            seen,
            out,
            family,
            RustFamilyFileKind::CargoConfigToml,
            "config.toml",
        );
        collect_cargo_dir_file(
            tree,
            root_rels,
            seen,
            out,
            family,
            RustFamilyFileKind::CargoConfigLegacy,
            "config",
        );
    }
}

fn collect_deny_files(
    tree: &dyn ProjectTreeDiscovery,
    root_rels: &[String],
    seen: &mut BTreeSet<(RustValidateFamily, String)>,
    out: &mut Vec<RustFamilyFileFact>,
) {
    if tree.file_exists("deny.toml") {
        push_file(
            root_rels,
            seen,
            out,
            RustValidateFamily::Deny,
            "deny.toml",
            RustFamilyFileKind::DenyToml,
        );
    }
    for dir in tree
        .dirs_with_file("deny.toml")
        .into_iter()
        .filter(|dir| !dir.ends_with("/.cargo") && dir != ".cargo")
        .filter(|dir| !is_excluded_live_root_dir(dir))
    {
        let rel_path = join_rel(&dir, "deny.toml");
        push_file(
            root_rels,
            seen,
            out,
            RustValidateFamily::Deny,
            &rel_path,
            RustFamilyFileKind::DenyToml,
        );
    }
    collect_root_and_dir_file(
        tree,
        root_rels,
        seen,
        out,
        RustValidateFamily::Deny,
        RustFamilyFileKind::DenyDotToml,
        ".deny.toml",
    );
    if tree.file_exists(".cargo/deny.toml") {
        push_file(
            root_rels,
            seen,
            out,
            RustValidateFamily::Deny,
            ".cargo/deny.toml",
            RustFamilyFileKind::CargoDenyToml,
        );
    }
    for dir in tree
        .dirs_with_file("deny.toml")
        .into_iter()
        .filter(|dir| dir.ends_with("/.cargo"))
        .filter(|dir| !is_excluded_live_root_dir(dir.strip_suffix("/.cargo").unwrap_or(dir)))
    {
        let rel_path = join_rel(&dir, "deny.toml");
        push_file(
            root_rels,
            seen,
            out,
            RustValidateFamily::Deny,
            &rel_path,
            RustFamilyFileKind::CargoDenyToml,
        );
    }
}

fn collect_cargo_and_policy_files(
    tree: &dyn ProjectTreeDiscovery,
    root_rels: &[String],
    seen: &mut BTreeSet<(RustValidateFamily, String)>,
    out: &mut Vec<RustFamilyFileFact>,
) {
    for family in [
        RustValidateFamily::Toolchain,
        RustValidateFamily::Clippy,
        RustValidateFamily::Deny,
        RustValidateFamily::Cargo,
        RustValidateFamily::Deps,
        RustValidateFamily::Garde,
        RustValidateFamily::Libarch,
        RustValidateFamily::Release,
    ] {
        collect_root_and_dir_file(
            tree,
            root_rels,
            seen,
            out,
            family,
            RustFamilyFileKind::CargoToml,
            "Cargo.toml",
        );
    }

    for family in [
        RustValidateFamily::Cargo,
        RustValidateFamily::Deps,
        RustValidateFamily::Garde,
    ] {
        collect_root_and_dir_file(
            tree,
            root_rels,
            seen,
            out,
            family,
            RustFamilyFileKind::GuardrailToml,
            "guardrail3.toml",
        );
    }
}

fn collect_release_files(
    tree: &dyn ProjectTreeDiscovery,
    root_rels: &[String],
    seen: &mut BTreeSet<(RustValidateFamily, String)>,
    out: &mut Vec<RustFamilyFileFact>,
) {
    collect_root_and_dir_file(
        tree,
        root_rels,
        seen,
        out,
        RustValidateFamily::Release,
        RustFamilyFileKind::ReleasePlzToml,
        "release-plz.toml",
    );
    collect_root_and_dir_file(
        tree,
        root_rels,
        seen,
        out,
        RustValidateFamily::Release,
        RustFamilyFileKind::CliffToml,
        "cliff.toml",
    );
}

fn collect_root_and_dir_file(
    tree: &dyn ProjectTreeDiscovery,
    root_rels: &[String],
    seen: &mut BTreeSet<(RustValidateFamily, String)>,
    out: &mut Vec<RustFamilyFileFact>,
    family: RustValidateFamily,
    kind: RustFamilyFileKind,
    file_name: &str,
) {
    if tree.file_exists(file_name) {
        push_file(root_rels, seen, out, family, file_name, kind);
    }
    for dir in tree
        .dirs_with_file(file_name)
        .into_iter()
        .filter(|dir| !(file_name == "deny.toml" && (dir == ".cargo" || dir.ends_with("/.cargo"))))
        .filter(|dir| !is_excluded_live_root_dir(dir))
    {
        let rel_path = join_rel(&dir, file_name);
        push_file(root_rels, seen, out, family, &rel_path, kind);
    }
}

fn collect_cargo_dir_file(
    tree: &dyn ProjectTreeDiscovery,
    root_rels: &[String],
    seen: &mut BTreeSet<(RustValidateFamily, String)>,
    out: &mut Vec<RustFamilyFileFact>,
    family: RustValidateFamily,
    kind: RustFamilyFileKind,
    file_name: &str,
) {
    let root_rel_path = format!(".cargo/{file_name}");
    if tree.file_exists(&root_rel_path) {
        push_file(root_rels, seen, out, family, &root_rel_path, kind);
    }
    for dir in tree
        .dirs_with_file(file_name)
        .into_iter()
        .filter(|dir| dir.ends_with("/.cargo"))
        .filter(|dir| !is_excluded_live_root_dir(dir.strip_suffix("/.cargo").unwrap_or(dir)))
    {
        let rel_path = join_rel(&dir, file_name);
        push_file(root_rels, seen, out, family, &rel_path, kind);
    }
}

fn push_file(
    root_rels: &[String],
    seen: &mut BTreeSet<(RustValidateFamily, String)>,
    out: &mut Vec<RustFamilyFileFact>,
    family: RustValidateFamily,
    rel_path: &str,
    kind: RustFamilyFileKind,
) {
    if !seen.insert((family, rel_path.to_owned())) {
        return;
    }
    if let Some(fact) = build_file_fact(root_rels, family, rel_path.to_owned(), kind) {
        out.push(fact);
    }
}

fn build_file_fact(
    root_rels: &[String],
    family: RustValidateFamily,
    rel_path: String,
    kind: RustFamilyFileKind,
) -> Option<RustFamilyFileFact> {
    let owner_rel = logical_owner_rel(&rel_path, kind)?;
    let attachment = attach_owner_rel(&owner_rel, root_rels);
    Some(RustFamilyFileFact::new(family, rel_path, kind, attachment))
}

fn logical_owner_rel(rel_path: &str, kind: RustFamilyFileKind) -> Option<String> {
    match kind {
        RustFamilyFileKind::CargoConfigToml
        | RustFamilyFileKind::CargoConfigLegacy
        | RustFamilyFileKind::CargoDenyToml => {
            let dir = rel_path.rsplit_once('/').map(|(dir, _)| dir).unwrap_or("");
            Some(
                dir.strip_suffix("/.cargo")
                    .unwrap_or("")
                    .trim_matches('/')
                    .to_owned(),
            )
        }
        _ => Some(
            rel_path
                .rsplit_once('/')
                .map(|(dir, _)| dir)
                .unwrap_or("")
                .to_owned(),
        ),
    }
}

fn attach_owner_rel(owner_rel: &str, root_rels: &[String]) -> RustFamilyFileAttachment {
    if root_rels.iter().any(|root_rel| root_rel == owner_rel) {
        return RustFamilyFileAttachment::ExactRoot {
            root_rel: owner_rel.to_owned(),
        };
    }

    if let Some(root_rel) = nearest_ancestor_root(owner_rel, root_rels) {
        return RustFamilyFileAttachment::NestedUnderRoot {
            root_rel: root_rel.to_owned(),
            owner_rel: owner_rel.to_owned(),
        };
    }

    let descendant_roots = root_rels
        .iter()
        .filter(|root_rel| path_is_under(root_rel, owner_rel))
        .cloned()
        .collect::<Vec<_>>();
    if !descendant_roots.is_empty() {
        return RustFamilyFileAttachment::AncestorOfRoots {
            root_rels: descendant_roots,
            owner_rel: owner_rel.to_owned(),
        };
    }

    RustFamilyFileAttachment::OutsideRoots {
        owner_rel: owner_rel.to_owned(),
    }
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

fn join_rel(parent: &str, child: &str) -> String {
    if parent.is_empty() {
        child.to_owned()
    } else {
        format!("{parent}/{child}")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use guardrail3_app_rs_placement;
    use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
    use guardrail3_validation_model::RustValidateFamily;

    use crate::{RustFamilyFileAttachment, RustFamilyFileKind};

    use super::collect;

    #[test]
    fn cargo_config_owner_normalizes_to_parent_workspace() {
        let tree = ProjectTree::new(
            PathBuf::from("/tmp/project"),
            BTreeMap::from([
                (
                    "".to_owned(),
                    DirEntry::new(vec!["apps".to_owned()], vec![], vec![], vec![]),
                ),
                (
                    "apps".to_owned(),
                    DirEntry::new(vec!["api".to_owned()], vec![], vec![], vec![]),
                ),
                (
                    "apps/api".to_owned(),
                    DirEntry::new(
                        vec![".cargo".to_owned()],
                        vec!["Cargo.toml".to_owned()],
                        vec![],
                        vec![],
                    ),
                ),
                (
                    "apps/api/.cargo".to_owned(),
                    DirEntry::new(vec![], vec!["config.toml".to_owned()], vec![], vec![]),
                ),
            ]),
            BTreeMap::from([(
                "apps/api/Cargo.toml".to_owned(),
                "[workspace]\nmembers = []\n".to_owned(),
            )]),
        );

        let placement = guardrail3_app_rs_placement::collect(&tree);
        let facts = collect(&tree, &placement);
        let fact = facts
            .family_files()
            .iter()
            .find(|fact| {
                fact.family() == RustValidateFamily::Clippy
                    && fact.kind() == RustFamilyFileKind::CargoConfigToml
            })
            .expect("expected cargo config ownership fact");

        assert_eq!(fact.logical_owner_rel(), "apps/api");
        assert!(matches!(
            fact.attachment(),
            RustFamilyFileAttachment::ExactRoot { root_rel } if root_rel == "apps/api"
        ));
    }

    #[test]
    fn ancestor_toolchain_file_stays_visible_for_descendant_workspace_roots() {
        let tree = ProjectTree::new(
            PathBuf::from("/tmp/project"),
            BTreeMap::from([
                (
                    "".to_owned(),
                    DirEntry::new(
                        vec!["apps".to_owned()],
                        vec!["rust-toolchain.toml".to_owned()],
                        vec![],
                        vec![],
                    ),
                ),
                (
                    "apps".to_owned(),
                    DirEntry::new(vec!["api".to_owned()], vec![], vec![], vec![]),
                ),
                (
                    "apps/api".to_owned(),
                    DirEntry::new(vec![], vec!["Cargo.toml".to_owned()], vec![], vec![]),
                ),
            ]),
            BTreeMap::from([(
                "apps/api/Cargo.toml".to_owned(),
                "[workspace]\nmembers = []\n".to_owned(),
            )]),
        );

        let placement = guardrail3_app_rs_placement::collect(&tree);
        let facts = collect(&tree, &placement);
        let fact = facts
            .family_files()
            .iter()
            .find(|fact| fact.kind() == RustFamilyFileKind::RustToolchainToml)
            .expect("expected toolchain ownership fact");

        assert!(matches!(
            fact.attachment(),
            RustFamilyFileAttachment::AncestorOfRoots { root_rels, owner_rel }
                if owner_rel.is_empty() && root_rels == &vec!["apps/api".to_owned()]
        ));
    }

    #[test]
    fn excluded_target_surface_stays_invisible() {
        let tree = ProjectTree::new(
            PathBuf::from("/tmp/project"),
            BTreeMap::from([
                (
                    "".to_owned(),
                    DirEntry::new(vec!["target".to_owned()], vec![], vec![], vec![]),
                ),
                (
                    "target".to_owned(),
                    DirEntry::new(vec!["scratch".to_owned()], vec![], vec![], vec![]),
                ),
                (
                    "target/scratch".to_owned(),
                    DirEntry::new(
                        vec![".cargo".to_owned()],
                        vec!["clippy.toml".to_owned()],
                        vec![],
                        vec![],
                    ),
                ),
                (
                    "target/scratch/.cargo".to_owned(),
                    DirEntry::new(vec![], vec!["config.toml".to_owned()], vec![], vec![]),
                ),
            ]),
            BTreeMap::new(),
        );

        let placement = guardrail3_app_rs_placement::collect(&tree);
        let facts = collect(&tree, &placement);
        assert!(facts.family_files().is_empty());
    }

    #[test]
    fn rootless_clippy_file_remains_visible_as_outside_roots() {
        let tree = ProjectTree::new(
            PathBuf::from("/tmp/project"),
            BTreeMap::from([
                (
                    "".to_owned(),
                    DirEntry::new(vec!["tools".to_owned()], vec![], vec![], vec![]),
                ),
                (
                    "tools".to_owned(),
                    DirEntry::new(vec!["helper".to_owned()], vec![], vec![], vec![]),
                ),
                (
                    "tools/helper".to_owned(),
                    DirEntry::new(vec![], vec!["clippy.toml".to_owned()], vec![], vec![]),
                ),
            ]),
            BTreeMap::new(),
        );

        let placement = guardrail3_app_rs_placement::collect(&tree);
        let facts = collect(&tree, &placement);
        let fact = facts
            .family_files()
            .iter()
            .find(|fact| fact.kind() == RustFamilyFileKind::ClippyToml)
            .expect("expected clippy ownership fact");

        assert!(matches!(
            fact.attachment(),
            RustFamilyFileAttachment::OutsideRoots { owner_rel } if owner_rel == "tools/helper"
        ));
    }

    #[test]
    fn root_cargo_deny_file_is_not_downgraded_to_plain_deny_toml() {
        let tree = ProjectTree::new(
            PathBuf::from("/tmp/project"),
            BTreeMap::from([
                (
                    "".to_owned(),
                    DirEntry::new(
                        vec![".cargo".to_owned()],
                        vec!["Cargo.toml".to_owned()],
                        vec![],
                        vec![],
                    ),
                ),
                (
                    ".cargo".to_owned(),
                    DirEntry::new(vec![], vec!["deny.toml".to_owned()], vec![], vec![]),
                ),
            ]),
            BTreeMap::from([
                (
                    "Cargo.toml".to_owned(),
                    "[workspace]\nmembers = []\n".to_owned(),
                ),
                (
                    ".cargo/deny.toml".to_owned(),
                    "[bans]\nmultiple-versions = \"deny\"\n".to_owned(),
                ),
            ]),
        );

        let placement = guardrail3_app_rs_placement::collect(&tree);
        let facts = collect(&tree, &placement);
        let deny_facts = facts
            .family_files()
            .iter()
            .filter(|fact| fact.family() == RustValidateFamily::Deny)
            .collect::<Vec<_>>();

        assert_eq!(deny_facts.len(), 2);
        let cargo_deny = deny_facts
            .iter()
            .find(|fact| fact.rel_path() == ".cargo/deny.toml")
            .expect("expected cargo deny ownership fact");
        assert_eq!(cargo_deny.kind(), RustFamilyFileKind::CargoDenyToml);
        assert_eq!(cargo_deny.logical_owner_rel(), "");
        assert!(matches!(
            cargo_deny.attachment(),
            RustFamilyFileAttachment::ExactRoot { root_rel } if root_rel.is_empty()
        ));
    }

    #[test]
    fn clippy_candidates_include_nested_root_and_cargo_override_files() {
        let tree = ProjectTree::new(
            PathBuf::from("/tmp/project"),
            BTreeMap::from([
                (
                    "".to_owned(),
                    DirEntry::new(
                        vec!["apps".to_owned()],
                        vec!["clippy.toml".to_owned()],
                        vec![],
                        vec![],
                    ),
                ),
                (
                    "apps".to_owned(),
                    DirEntry::new(
                        vec!["backend".to_owned(), "devctl".to_owned()],
                        vec![],
                        vec![],
                        vec![],
                    ),
                ),
                (
                    "apps/backend".to_owned(),
                    DirEntry::new(
                        vec!["src".to_owned()],
                        vec!["Cargo.toml".to_owned()],
                        vec![],
                        vec![],
                    ),
                ),
                (
                    "apps/backend/src".to_owned(),
                    DirEntry::new(vec![], vec!["lib.rs".to_owned()], vec![], vec![]),
                ),
                (
                    "apps/devctl".to_owned(),
                    DirEntry::new(
                        vec![".cargo".to_owned()],
                        vec!["Cargo.toml".to_owned(), "clippy.toml".to_owned()],
                        vec![],
                        vec![],
                    ),
                ),
                (
                    "apps/devctl/.cargo".to_owned(),
                    DirEntry::new(vec![], vec!["config.toml".to_owned()], vec![], vec![]),
                ),
            ]),
            BTreeMap::from([
                ("clippy.toml".to_owned(), "msrv = \"1.85\"\n".to_owned()),
                (
                    "apps/backend/Cargo.toml".to_owned(),
                    "[workspace]\nmembers = []\n".to_owned(),
                ),
                (
                    "apps/devctl/Cargo.toml".to_owned(),
                    "[workspace]\nmembers = []\n".to_owned(),
                ),
                (
                    "apps/devctl/clippy.toml".to_owned(),
                    "msrv = \"1.85\"\n".to_owned(),
                ),
                (
                    "apps/devctl/.cargo/config.toml".to_owned(),
                    "[env]\nCLIPPY_CONF_DIR = \".\"\n".to_owned(),
                ),
            ]),
        );

        let placement = guardrail3_app_rs_placement::collect(&tree);
        let facts = collect(&tree, &placement);
        let files = facts
            .family_files()
            .iter()
            .filter(|fact| fact.family() == RustValidateFamily::Clippy)
            .map(|fact| fact.rel_path().to_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            files,
            vec![
                "apps/backend/Cargo.toml".to_owned(),
                "apps/devctl/.cargo/config.toml".to_owned(),
                "apps/devctl/Cargo.toml".to_owned(),
                "apps/devctl/clippy.toml".to_owned(),
                "clippy.toml".to_owned(),
            ]
        );
    }

    #[test]
    fn cargo_deny_keeps_cargo_specific_kind() {
        let tree = ProjectTree::new(
            PathBuf::from("/tmp/project"),
            BTreeMap::from([
                (
                    "".to_owned(),
                    DirEntry::new(
                        vec![".cargo".to_owned()],
                        vec![
                            "Cargo.toml".to_owned(),
                            "deny.toml".to_owned(),
                            ".deny.toml".to_owned(),
                        ],
                        vec![],
                        vec![],
                    ),
                ),
                (
                    ".cargo".to_owned(),
                    DirEntry::new(vec![], vec!["deny.toml".to_owned()], vec![], vec![]),
                ),
            ]),
            BTreeMap::from([(
                "Cargo.toml".to_owned(),
                "[package]\nname = \"crate\"\n".to_owned(),
            )]),
        );

        let placement = guardrail3_app_rs_placement::collect(&tree);
        let facts = collect(&tree, &placement);
        let file_kinds = facts
            .family_files()
            .iter()
            .filter(|fact| fact.family() == RustValidateFamily::Deny)
            .map(|fact| (fact.rel_path().to_owned(), fact.kind()))
            .collect::<Vec<_>>();

        assert_eq!(
            file_kinds,
            vec![
                (
                    ".cargo/deny.toml".to_owned(),
                    RustFamilyFileKind::CargoDenyToml
                ),
                (".deny.toml".to_owned(), RustFamilyFileKind::DenyDotToml),
                ("Cargo.toml".to_owned(), RustFamilyFileKind::CargoToml),
                ("deny.toml".to_owned(), RustFamilyFileKind::DenyToml),
            ]
        );
    }
}
