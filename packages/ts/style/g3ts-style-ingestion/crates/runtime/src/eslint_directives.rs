use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use g3ts_style_types::{G3TsStyleEslintDirectiveInput, G3TsStylePolicySurfaceState};

const SOURCE_EXTENSIONS: [&str; 5] = [".astro", ".js", ".jsx", ".ts", ".tsx"];

pub(crate) fn eslint_directives(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    policy: &G3TsStylePolicySurfaceState,
) -> Vec<G3TsStyleEslintDirectiveInput> {
    source_paths(crawl, app_root_rel_path, policy)
        .into_iter()
        .flat_map(|rel_path| file_directives(crawl, &rel_path))
        .collect()
}

fn source_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    policy: &G3TsStylePolicySurfaceState,
) -> Vec<String> {
    let Some(globs) = style_source_globs(app_root_rel_path, policy) else {
        return Vec::new();
    };
    let Ok(globs) = compile_globs(&globs) else {
        return Vec::new();
    };

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == G3WorkspaceEntryKind::File
                && entry.ignore_state == G3WorkspaceIgnoreState::Included
                && SOURCE_EXTENSIONS
                    .iter()
                    .any(|extension| entry.path.rel_path.ends_with(extension))
                && globs.is_match(&entry.path.rel_path)
        })
        .map(|entry| entry.path.rel_path.clone())
        .collect()
}

fn style_source_globs(
    app_root_rel_path: &str,
    policy: &G3TsStylePolicySurfaceState,
) -> Option<Vec<String>> {
    let G3TsStylePolicySurfaceState::Parsed { snapshot } = policy else {
        return None;
    };
    Some(
        snapshot
            .source_globs
            .iter()
            .map(|glob| crate::roots::scoped_rel_path(app_root_rel_path, glob))
            .collect(),
    )
}

fn compile_globs(globs: &[String]) -> Result<globset::GlobSet, globset::Error> {
    let mut builder = globset::GlobSetBuilder::new();
    for glob in globs {
        let _ = builder.add(globset::Glob::new(glob)?);
    }
    builder.build()
}

fn file_directives(
    crawl: &G3WorkspaceCrawl,
    rel_path: &str,
) -> Vec<G3TsStyleEslintDirectiveInput> {
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
            G3TsStyleEslintDirectiveInput::parsed(
                finding.rel_path,
                format!("{:?}", finding.directive_kind),
                disabled_rules(finding.disabled_rules),
                all_rules,
                finding.line,
                finding.target_line,
            )
        })
        .collect()
}

fn parse_error(rel_path: &str, reason: String) -> G3TsStyleEslintDirectiveInput {
    G3TsStyleEslintDirectiveInput::parse_error(rel_path.to_owned(), reason)
}

fn disabled_rules(rules: eslint_directive_parser::types::EslintDisabledRuleSet) -> Vec<String> {
    match rules {
        eslint_directive_parser::types::EslintDisabledRuleSet::AllRules => Vec::new(),
        eslint_directive_parser::types::EslintDisabledRuleSet::Rules(rules) => rules,
    }
}

#[cfg(test)]
#[path = "eslint_directives_tests/mod.rs"]
mod eslint_directives_tests;
