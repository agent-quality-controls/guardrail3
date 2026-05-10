use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use g3ts_style_types::{G3TsStyleEslintDirectiveInput, G3TsStylePolicySurfaceState};

/// Source-file extensions scanned for inline `ESLint` disable directives.
const SOURCE_EXTENSIONS: [&str; 5] = [".astro", ".js", ".jsx", ".ts", ".tsx"];

/// Collect all inline `ESLint` disable directives within source files
/// matching the style policy globs at `app_root_rel_path`.
pub(crate) fn eslint_directives(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    policy: &G3TsStylePolicySurfaceState,
) -> Vec<G3TsStyleEslintDirectiveInput> {
    let Some(globs) = style_source_globs(app_root_rel_path, policy) else {
        return Vec::new();
    };
    let globs = match compile_globs(&globs) {
        Ok(globs) => globs,
        Err(error) => {
            return vec![G3TsStyleEslintDirectiveInput {
                rel_path: crate::roots::scoped_rel_path(app_root_rel_path, "guardrail3-ts.toml"),
                directive_kind: String::new(),
                disabled_rules: Vec::new(),
                all_rules: false,
                line: 0,
                target_line: None,
                parse_error: Some(format!(
                    "`[ts.style].source_globs` could not be compiled for ESLint disable inventory: {error}"
                )),
            }];
        }
    };

    source_paths(crawl, &globs)
        .into_iter()
        .flat_map(|rel_path| file_directives(crawl, &rel_path))
        .collect()
}

/// Enumerate workspace-relative paths of included source files matching
/// `globs` and having a known source-file extension.
fn source_paths(crawl: &G3WorkspaceCrawl, globs: &globset::GlobSet) -> Vec<String> {
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

/// Scope `policy.source_globs` onto `app_root_rel_path` when the policy is
/// parsed; otherwise return None.
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

/// Compile `globs` into a single `GlobSet` for matcher reuse.
fn compile_globs(globs: &[String]) -> Result<globset::GlobSet, globset::Error> {
    let mut builder = globset::GlobSetBuilder::new();
    for glob in globs {
        let _ = builder.add(globset::Glob::new(glob)?);
    }
    builder.build()
}

/// Read and parse `rel_path` for inline `ESLint` disable directives,
/// returning a single parse-error finding when the parser fails.
fn file_directives(crawl: &G3WorkspaceCrawl, rel_path: &str) -> Vec<G3TsStyleEslintDirectiveInput> {
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
            G3TsStyleEslintDirectiveInput {
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

/// Build a parse-error directive finding for `rel_path` carrying `reason`.
fn parse_error(rel_path: &str, reason: String) -> G3TsStyleEslintDirectiveInput {
    G3TsStyleEslintDirectiveInput {
        rel_path: rel_path.to_owned(),
        directive_kind: String::new(),
        disabled_rules: Vec::new(),
        all_rules: false,
        line: 0,
        target_line: None,
        parse_error: Some(reason),
    }
}

/// Materialize the list of disabled rules from a parsed disable directive.
fn disabled_rules(rules: eslint_directive_parser::types::EslintDisabledRuleSet) -> Vec<String> {
    match rules {
        eslint_directive_parser::types::EslintDisabledRuleSet::AllRules => Vec::new(),
        eslint_directive_parser::types::EslintDisabledRuleSet::Rules(rules) => rules,
    }
}

#[cfg(test)]
#[path = "eslint_directives_tests/mod.rs"]
mod eslint_directives_tests;
