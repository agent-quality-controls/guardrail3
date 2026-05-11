use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_astro_state_types::{
    G3TsAstroStateAppRootInput, G3TsAstroStateFileTreeChecksInput,
    G3TsAstroStateForbiddenPathInput, G3TsAstroStateLegacyGeneratedPathInput,
    G3TsAstroStatePolicySurfaceState,
};

#[must_use]
pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroStateFileTreeChecksInput {
    let app_root_rel_paths = crate::roots::astro_app_roots(crawl);
    let app_roots: Vec<G3TsAstroStateAppRootInput> = app_root_rel_paths
        .iter()
        .map(|app_root_rel_path| G3TsAstroStateAppRootInput {
            app_root_rel_path: app_root_rel_path.clone(),
        })
        .collect();
    let strict_content_roots = app_roots
        .iter()
        .filter(|root| {
            crate::policy::has_strict_astro_state_boundary(crawl, &root.app_root_rel_path)
        })
        .collect::<Vec<_>>();
    let legacy_generated_paths = strict_content_roots
        .iter()
        .flat_map(|root| {
            crate::policy::legacy_generated_state_paths(
                crawl,
                &root.app_root_rel_path,
                &app_root_rel_paths,
            )
            .into_iter()
            .map(|rel_path| G3TsAstroStateLegacyGeneratedPathInput {
                app_root_rel_path: root.app_root_rel_path.clone(),
                rel_path,
            })
        })
        .collect();
    let forbidden_state_paths = strict_content_roots
        .iter()
        .flat_map(|root| {
            let astro_policy =
                crate::policy::ingest_state_policy_surface(crawl, &root.app_root_rel_path);
            forbidden_state_paths(
                crawl,
                &root.app_root_rel_path,
                &app_root_rel_paths,
                &astro_policy,
            )
            .into_iter()
            .map(|rel_path| G3TsAstroStateForbiddenPathInput {
                app_root_rel_path: root.app_root_rel_path.clone(),
                rel_path,
            })
        })
        .collect();
    G3TsAstroStateFileTreeChecksInput {
        strict_app_roots: strict_content_roots
            .into_iter()
            .map(
                |root| g3ts_astro_state_types::G3TsAstroStateStrictAppRootInput {
                    app_root_rel_path: root.app_root_rel_path.clone(),
                },
            )
            .collect(),
        legacy_generated_paths,
        forbidden_state_paths,
    }
}

/// Returns rel paths under the app root that match the policy's forbidden state globs.
fn forbidden_state_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    app_root_rel_paths: &[String],
    astro_policy: &G3TsAstroStatePolicySurfaceState,
) -> Vec<String> {
    let G3TsAstroStatePolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };
    let mut builder = globset::GlobSetBuilder::new();
    for pattern in &snapshot.forbidden_state {
        let Ok(glob) = globset::Glob::new(pattern) else {
            return Vec::new();
        };
        let _builder = builder.add(glob);
    }
    let Ok(globs) = builder.build() else {
        return Vec::new();
    };

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.readable
                && matches!(
                    entry.kind,
                    g3_workspace_crawl::G3WorkspaceEntryKind::File
                        | g3_workspace_crawl::G3WorkspaceEntryKind::Directory
                )
                && g3ts_astro_check_support::surfaces::is_under_app_root(
                    &entry.path.rel_path,
                    app_root_rel_path,
                )
                && g3ts_astro_check_support::surfaces::nearest_app_root(
                    &entry.path.rel_path,
                    app_root_rel_paths,
                )
                .is_some_and(|nearest| nearest == app_root_rel_path)
                && globs.is_match(g3ts_astro_check_support::surfaces::app_relative_path(
                    &entry.path.rel_path,
                    app_root_rel_path,
                ))
        })
        .map(|entry| entry.path.rel_path.clone())
        .collect()
}
