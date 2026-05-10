use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use g3ts_astro_mdx_types::{G3TsAstroMdxEslintDirectiveInput, G3TsAstroMdxPolicySurfaceState};

/// `eslint_directives` helper.
pub(crate) fn eslint_directives(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> Vec<G3TsAstroMdxEslintDirectiveInput> {
    source_paths(crawl, app_root_rel_path, astro_policy)
        .into_iter()
        .flat_map(|rel_path| file_directives(crawl, &rel_path))
        .collect()
}

/// `source_paths` helper.
fn source_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> Vec<String> {
    let content_root = content_root(astro_policy);
    let scoped_content_root =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, &content_root);
    let scoped_content_prefix = format!("{scoped_content_root}/");
    let component_maps = mdx_component_map_roots(app_root_rel_path, astro_policy);

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == G3WorkspaceEntryKind::File
                && entry.ignore_state == G3WorkspaceIgnoreState::Included
                && (is_mdx_content(&entry.path.rel_path, &scoped_content_prefix)
                    || is_component_map_source(&entry.path.rel_path, &component_maps))
        })
        .map(|entry| entry.path.rel_path.clone())
        .collect()
}

/// `content_root` helper.
fn content_root(astro_policy: &G3TsAstroMdxPolicySurfaceState) -> String {
    match astro_policy {
        G3TsAstroMdxPolicySurfaceState::Parsed { snapshot } => snapshot
            .content_root
            .as_deref()
            .unwrap_or("src/content")
            .trim_end_matches('/')
            .to_owned(),
        G3TsAstroMdxPolicySurfaceState::Missing { .. }
        | G3TsAstroMdxPolicySurfaceState::Unreadable { .. }
        | G3TsAstroMdxPolicySurfaceState::ParseError { .. }
        | G3TsAstroMdxPolicySurfaceState::MissingAstroPolicy { .. } => "src/content".to_owned(),
    }
}

/// `mdx_component_map_roots` helper.
fn mdx_component_map_roots(
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> Vec<String> {
    let G3TsAstroMdxPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    snapshot
        .mdx_component_maps
        .iter()
        .map(|path| {
            g3ts_astro_check_support::surfaces::scoped_rel_path(
                app_root_rel_path,
                path.trim_end_matches('/'),
            )
        })
        .collect()
}

/// `is_mdx_content` helper.
fn is_mdx_content(rel_path: &str, scoped_content_prefix: &str) -> bool {
    rel_path.starts_with(scoped_content_prefix)
        && std::path::Path::new(rel_path)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("mdx"))
}

/// `is_component_map_source` helper.
fn is_component_map_source(rel_path: &str, component_maps: &[String]) -> bool {
    component_maps.iter().any(|root| {
        let prefix = format!("{root}/");
        (rel_path == root || rel_path.starts_with(&prefix))
            && [".ts", ".tsx", ".js", ".jsx"]
                .iter()
                .any(|extension| rel_path.ends_with(extension))
    })
}

/// `file_directives` helper.
fn file_directives(
    crawl: &G3WorkspaceCrawl,
    rel_path: &str,
) -> Vec<G3TsAstroMdxEslintDirectiveInput> {
    let abs_path = crawl.root_abs_path.join(rel_path);
    let document = match eslint_directive_parser::from_path_document(&abs_path, rel_path) {
        Ok(document) => document,
        Err(err) => return vec![parse_error(rel_path, err.to_string())],
    };
    let findings = match document.typed.state {
        eslint_directive_parser::types::EslintDirectiveParseState::Parsed { findings } => findings,
        eslint_directive_parser::types::EslintDirectiveParseState::Unsupported { reason }
        | eslint_directive_parser::types::EslintDirectiveParseState::ParseError { reason }
        | eslint_directive_parser::types::EslintDirectiveParseState::Ambiguous { reason } => {
            return vec![parse_error(rel_path, reason)];
        }
    };

    findings
        .into_iter()
        .map(|finding| {
            let all_rules = matches!(
                finding.disabled_rules,
                eslint_directive_parser::types::EslintDisabledRuleSet::AllRules
            );
            G3TsAstroMdxEslintDirectiveInput {
                rel_path: finding.rel_path,
                directive_kind: format!("{:?}", finding.directive_kind),
                disabled_rules: disabled_rules(finding.disabled_rules),
                all_rules,
                line: finding.line,
                target_line: finding.target_line,
                parse_error: None,
            }
        })
        .collect()
}

/// `parse_error` helper.
fn parse_error(rel_path: &str, reason: String) -> G3TsAstroMdxEslintDirectiveInput {
    G3TsAstroMdxEslintDirectiveInput {
        rel_path: rel_path.to_owned(),
        directive_kind: "ParseError".to_owned(),
        disabled_rules: Vec::new(),
        all_rules: false,
        line: 0,
        target_line: None,
        parse_error: Some(reason),
    }
}

/// `disabled_rules` helper.
fn disabled_rules(rules: eslint_directive_parser::types::EslintDisabledRuleSet) -> Vec<String> {
    match rules {
        eslint_directive_parser::types::EslintDisabledRuleSet::AllRules => Vec::new(),
        eslint_directive_parser::types::EslintDisabledRuleSet::Rules(rules) => rules,
    }
}
