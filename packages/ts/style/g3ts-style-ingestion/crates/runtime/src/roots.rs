use std::collections::BTreeSet;

use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;

#[must_use]
pub(crate) fn style_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut roots = BTreeSet::new();
    for entry in &crawl.entries {
        if entry.path.rel_path.ends_with("guardrail3-ts.toml")
            || package_manifest_has_style_surface(entry)
        {
            let _inserted = roots.insert(parent_rel_path(&entry.path.rel_path));
        }
    }
    roots.into_iter().collect()
}

fn package_manifest_has_style_surface(entry: &g3_workspace_crawl::G3RsWorkspaceEntry) -> bool {
    if !entry.path.rel_path.ends_with("package.json") || !entry.readable {
        return false;
    }
    let Ok(document) = package_json_parser::from_path_document(&entry.path.abs_path) else {
        return false;
    };
    let Some(typed) = package_json_parser::typed(&document) else {
        return false;
    };
    typed
        .dependencies
        .iter()
        .chain(typed.dev_dependencies.iter())
        .any(|dependency| {
            matches!(
                dependency.as_str(),
                "tailwindcss" | "stylelint" | "g3ts-eslint-plugin-style-policy"
            )
        })
}

pub(crate) fn scoped_rel_path(scope: &str, local: &str) -> String {
    if scope.is_empty() || scope == "." {
        return local.to_owned();
    }
    format!(
        "{}/{}",
        scope.trim_end_matches('/'),
        local.trim_start_matches('/')
    )
}

fn parent_rel_path(rel_path: &str) -> String {
    std::path::Path::new(rel_path)
        .parent()
        .and_then(std::path::Path::to_str)
        .filter(|parent| !parent.is_empty())
        .map_or_else(|| ".".to_owned(), str::to_owned)
}
