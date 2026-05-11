use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntryKind, G3WorkspaceIgnoreState};
use g3ts_astro_seo_types::G3TsAstroSeoEslintDirectiveInput;

/// `SOURCE_EXTENSIONS` constant.
const SOURCE_EXTENSIONS: [&str; 3] = [".astro", ".ts", ".tsx"];

/// `eslint_directives`: eslint directives.
pub(crate) fn eslint_directives(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Vec<G3TsAstroSeoEslintDirectiveInput> {
    source_paths(crawl, app_root_rel_path)
        .into_iter()
        .flat_map(|rel_path| file_directives(crawl, &rel_path))
        .collect()
}

/// `source_paths`: source paths.
fn source_paths(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Vec<String> {
    let src_root = g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "src");
    let src_prefix = format!("{src_root}/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == G3WorkspaceEntryKind::File
                && entry.ignore_state == G3WorkspaceIgnoreState::Included
                && entry.path.rel_path.starts_with(&src_prefix)
                && SOURCE_EXTENSIONS
                    .iter()
                    .any(|extension| entry.path.rel_path.ends_with(extension))
        })
        .map(|entry| entry.path.rel_path.clone())
        .collect()
}

/// `file_directives`: file directives.
fn file_directives(
    crawl: &G3WorkspaceCrawl,
    rel_path: &str,
) -> Vec<G3TsAstroSeoEslintDirectiveInput> {
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
            G3TsAstroSeoEslintDirectiveInput::new(
                finding.rel_path,
                format!("{:?}", finding.directive_kind),
                disabled_rules(finding.disabled_rules),
                all_rules,
                finding.line,
                finding.target_line,
                None,
            )
        })
        .collect()
}

/// `parse_error`: parse error.
fn parse_error(rel_path: &str, reason: String) -> G3TsAstroSeoEslintDirectiveInput {
    G3TsAstroSeoEslintDirectiveInput::new(
        rel_path.to_owned(),
        "ParseError".to_owned(),
        Vec::new(),
        false,
        0,
        None,
        Some(reason),
    )
}

/// `disabled_rules`: disabled rules.
fn disabled_rules(rules: eslint_directive_parser::types::EslintDisabledRuleSet) -> Vec<String> {
    match rules {
        eslint_directive_parser::types::EslintDisabledRuleSet::AllRules => Vec::new(),
        eslint_directive_parser::types::EslintDisabledRuleSet::Rules(rules) => rules,
    }
}
