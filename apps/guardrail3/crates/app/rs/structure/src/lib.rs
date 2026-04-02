use guardrail3_app_rs_ownership::{
    RustFamilyFileFact, RustOwnedSurfaceFacts, collect as collect_owned_surface,
};
use guardrail3_app_rs_placement::{
    RustRootPlacementFacts, RustRootPlacementInputFailureFacts, RustRootPlacementRootFacts,
    RustZoneOverlapFacts, collect as collect_placement,
};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[cfg(feature = "api")]
pub use guardrail3_domain_project_tree::DirEntry;
use guardrail3_domain_project_tree::ProjectTree;

#[derive(Debug, Clone, Default)]
pub struct RustStructureFacts {
    placement: RustRootPlacementFacts,
    owned_surface: RustOwnedSurfaceFacts,
    /// Carried forward from ProjectTree — filtered content for classified roots.
    content: BTreeMap<String, String>,
    /// Carried forward from ProjectTree — project root path.
    root: PathBuf,
    /// Carried forward from ProjectTree — directory structure (for legality glob expansion).
    structure: BTreeMap<String, DirEntry>,
    /// Configurable exclusion patterns from guardrail3.toml.
    excluded_paths: Vec<String>,
}

impl RustStructureFacts {
    #[must_use]
    pub fn new(
        placement: RustRootPlacementFacts,
        owned_surface: RustOwnedSurfaceFacts,
        content: BTreeMap<String, String>,
        root: PathBuf,
        structure: BTreeMap<String, DirEntry>,
    ) -> Self {
        Self {
            placement,
            owned_surface,
            content,
            root,
            structure,
            excluded_paths: Vec::new(),
        }
    }

    #[must_use]
    pub fn placement(&self) -> &RustRootPlacementFacts {
        &self.placement
    }

    #[must_use]
    pub fn owned_surface(&self) -> &RustOwnedSurfaceFacts {
        &self.owned_surface
    }

    #[must_use]
    pub fn roots(&self) -> &[RustRootPlacementRootFacts] {
        self.placement.roots()
    }

    #[must_use]
    pub fn overlaps(&self) -> &[RustZoneOverlapFacts] {
        self.placement.overlaps()
    }

    #[must_use]
    pub fn input_failures(&self) -> &[RustRootPlacementInputFailureFacts] {
        self.placement.input_failures()
    }

    #[must_use]
    pub fn family_files(&self) -> &[RustFamilyFileFact] {
        self.owned_surface.family_files()
    }

    #[must_use]
    pub fn content(&self) -> &BTreeMap<String, String> {
        &self.content
    }

    #[must_use]
    pub fn file_content(&self, rel: &str) -> Option<&str> {
        self.content.get(rel).map(String::as_str)
    }

    #[must_use]
    pub fn root_path(&self) -> &PathBuf {
        &self.root
    }

    #[must_use]
    pub fn dir_structure(&self) -> &BTreeMap<String, DirEntry> {
        &self.structure
    }

    /// Filter structure/content to only include data under the given root directories.
    /// Strips out everything not under a legal root (test fixtures, etc.).
    #[must_use]
    pub fn filter_to_roots(self, root_rels: &[String]) -> Self {
        let excluded = &self.excluded_paths;
        let filtered_structure: BTreeMap<String, DirEntry> = self
            .structure
            .into_iter()
            .filter(|(dir_rel, _)| {
                root_rels
                    .iter()
                    .any(|root| path_is_under(dir_rel, root))
                    && !is_excluded_by_builtin(dir_rel)
                    && !is_excluded_by_config(dir_rel, excluded)
            })
            .collect();

        let filtered_content: BTreeMap<String, String> = self
            .content
            .into_iter()
            .filter(|(rel, _)| {
                root_rels
                    .iter()
                    .any(|root| path_is_under(rel, root))
                    && !is_excluded_by_builtin(rel)
                    && !is_excluded_by_config(rel, excluded)
            })
            .collect();

        Self {
            placement: self.placement,
            owned_surface: self.owned_surface,
            content: filtered_content,
            root: self.root,
            structure: filtered_structure,
            excluded_paths: excluded.to_vec(),
        }
    }

    /// Glob-match directories in the carried-forward structure.
    #[must_use]
    pub fn matching_dir_rels(&self, pattern: &str) -> Vec<String> {
        let normalized = pattern.trim_matches('/');
        let Ok(pat) = glob::Pattern::new(normalized) else {
            return Vec::new();
        };
        self.structure
            .keys()
            .filter(|dir_rel| !dir_rel.is_empty() && pat.matches(dir_rel))
            .cloned()
            .collect()
    }
}

/// Built-in exclusions that always apply (Cargo build output, tool worktrees).
fn is_excluded_by_builtin(path: &str) -> bool {
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.contains(&"target") {
        return true;
    }
    segments
        .windows(2)
        .any(|window| matches!(window, [".claude", "worktrees"]))
}

/// Configurable exclusions from guardrail3.toml `[rust] excluded_paths`.
fn is_excluded_by_config(path: &str, excluded_paths: &[String]) -> bool {
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    for pattern in excluded_paths {
        let pattern_segments: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
        if pattern_segments.is_empty() {
            continue;
        }
        if segments
            .windows(pattern_segments.len())
            .any(|window| window == pattern_segments.as_slice())
        {
            return true;
        }
    }
    false
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

#[must_use]
pub fn collect(tree: ProjectTree, excluded_paths: &[String]) -> RustStructureFacts {
    let placement = collect_placement(&tree);
    let owned_surface = collect_owned_surface(&tree, &placement);
    let content = tree.content().clone();
    let root = tree.root().clone();
    let structure = tree.structure().clone();
    // tree is consumed here — no further access possible
    let mut facts = RustStructureFacts::new(placement, owned_surface, content, root, structure);
    facts.excluded_paths = excluded_paths.to_vec();
    facts
}
