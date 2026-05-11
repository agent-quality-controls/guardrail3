use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use g3_workspace_crawl as workspace_crawl;
use g3ts_hooks_types as hook_types;
use hook_shell_parser::{command_query::shell_words, parse_script};

use crate::fs::{direct_files, executable, read_to_string};
use crate::process::{git_root, read_hooks_path};

#[derive(Debug)]
/// Internal struct `SelectedHookSurface`.
struct SelectedHookSurface {
    /// Internal field `rel_path`.
    rel_path: String,
    /// Internal field `abs_path`.
    abs_path: PathBuf,
    /// Internal field `has_modular_dir`.
    has_modular_dir: bool,
}

#[derive(Debug)]
/// Internal struct `HookScriptSurface`.
struct HookScriptSurface {
    /// Internal field `rel_path`.
    rel_path: String,
    /// Internal field `abs_path`.
    abs_path: PathBuf,
}

#[must_use]
pub fn ingest_for_source_checks(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
) -> Vec<hook_types::G3TsHooksSourceChecksInput> {
    let hook_root =
        git_root(crawl.root_abs_path.as_path()).unwrap_or_else(|| crawl.root_abs_path.clone());
    let hooks_path = read_hooks_path(hook_root.as_path());
    let Some(selected) =
        select_pre_commit_surface(crawl, hook_root.as_path(), hooks_path.as_deref())
    else {
        return Vec::new();
    };
    let selected_content = read_to_string(selected.abs_path.as_path());
    let selected_parsed = parse_script(selected_content.as_str());
    let app_package_roots = app_package_roots(crawl);
    let category_roots = category_roots_for_selected_hook(
        crawl,
        hook_root.as_path(),
        &selected_parsed,
        &app_package_roots,
    );
    let enabled_categories = enabled_categories(crawl, &category_roots);
    let mut inputs = Vec::new();
    inputs.push(hook_types::G3TsHooksSourceChecksInput::new(
        selected.rel_path.clone(),
        hook_types::G3TsHookScriptKind::PreCommit,
        selected_parsed,
        selected.has_modular_dir,
        app_package_roots.clone(),
        enabled_categories,
        Vec::new(),
    ));
    if let Some(script) = verifier_surface(crawl, hook_root.as_path()) {
        let content = read_to_string(script.abs_path.as_path());
        inputs.push(hook_types::G3TsHooksSourceChecksInput::new(
            script.rel_path,
            hook_types::G3TsHookScriptKind::Verifier,
            parse_script(content.as_str()),
            selected.has_modular_dir,
            app_package_roots,
            enabled_categories,
            Vec::new(),
        ));
    }
    inputs
}

/// Internal function `verifier_surface`.
fn verifier_surface(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
    hook_root: &Path,
) -> Option<HookScriptSurface> {
    hook_surface(crawl, hook_root, "scripts/g3ts/verify")
}

#[must_use]
pub fn ingest_for_file_tree_checks(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
) -> hook_types::G3TsHooksFileTreeChecksInput {
    let active = hooks_scope_is_active(crawl);
    if !active {
        return hook_types::G3TsHooksFileTreeChecksInput::new(
            active,
            None,
            false,
            Vec::new(),
            Vec::new(),
            None,
            Vec::new(),
        );
    }
    let hook_root =
        git_root(crawl.root_abs_path.as_path()).unwrap_or_else(|| crawl.root_abs_path.clone());
    let hooks_path = read_hooks_path(hook_root.as_path());
    let normalized_hooks_path = normalized_hooks_path(hook_root.as_path(), hooks_path.as_deref());
    let selected = select_pre_commit_surface(crawl, hook_root.as_path(), hooks_path.as_deref());
    let has_modular_dir = hook_root.join(".githooks/pre-commit.d").is_dir()
        || has_entry_dir(crawl, ".githooks/pre-commit.d");
    hook_types::G3TsHooksFileTreeChecksInput::new(
        active,
        selected.map(|surface| {
            script_file_fact_from_path(surface.rel_path, surface.abs_path.as_path())
        }),
        has_modular_dir,
        if has_modular_dir {
            direct_modular_entries(crawl, hook_root.as_path())
                .into_iter()
                .map(script_file_fact_from_surface)
                .collect()
        } else {
            Vec::new()
        },
        direct_file_names(crawl, ".guardrail3/overrides/pre-commit.d/"),
        normalized_hooks_path,
        trust_risks(crawl),
    )
}

/// Convenience entry point that reads `PATH` from the process environment.
#[expect(
    clippy::disallowed_methods,
    reason = "ingestion entry point intentionally reads $PATH at the process boundary to derive available CLIs; alternative is an explicit `_with_path` overload that callers use in tests"
)]
#[must_use]
pub fn ingest_for_config_checks(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
) -> hook_types::G3TsHooksConfigChecksInput {
    ingest_for_config_checks_with_path(crawl, std::env::var_os("PATH").as_deref())
}

#[must_use]
pub fn ingest_for_config_checks_with_path(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
    path_env: Option<&OsStr>,
) -> hook_types::G3TsHooksConfigChecksInput {
    let active = hooks_scope_is_active(crawl);
    let hook_root =
        git_root(crawl.root_abs_path.as_path()).unwrap_or_else(|| crawl.root_abs_path.clone());
    let hooks_path = read_hooks_path(hook_root.as_path());
    hook_types::G3TsHooksConfigChecksInput::new(
        active,
        select_pre_commit_surface(crawl, hook_root.as_path(), hooks_path.as_deref()).map(
            |surface| {
                hook_types::G3TsHooksSelectedHookConfigFact::new(
                    surface.rel_path,
                    parse_script(read_to_string(surface.abs_path.as_path()).as_str()),
                )
            },
        ),
        discover_installed_tools(path_env),
        Vec::new(),
    )
}

/// Internal function `hooks_scope_is_active`.
fn hooks_scope_is_active(crawl: &workspace_crawl::G3WorkspaceCrawl) -> bool {
    has_entry_file(crawl, "package.json") || !app_package_roots(crawl).is_empty()
}

/// Internal function `select_pre_commit_surface`.
fn select_pre_commit_surface(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
    hook_root: &Path,
    hooks_path: Option<&str>,
) -> Option<SelectedHookSurface> {
    let normalized = normalized_hooks_path(hook_root, hooks_path);
    let surface = match normalized.as_deref() {
        Some(".githooks") => hook_surface(crawl, hook_root, ".githooks/pre-commit"),
        Some("hooks") => hook_surface(crawl, hook_root, "hooks/pre-commit"),
        Some(_) => None,
        None => hook_surface(crawl, hook_root, ".githooks/pre-commit")
            .or_else(|| hook_surface(crawl, hook_root, "hooks/pre-commit")),
    }?;
    Some(SelectedHookSurface {
        has_modular_dir: surface.rel_path == ".githooks/pre-commit"
            && (hook_root.join(".githooks/pre-commit.d").is_dir()
                || has_entry_dir(crawl, ".githooks/pre-commit.d")),
        rel_path: surface.rel_path,
        abs_path: surface.abs_path,
    })
}

/// Internal function `hook_surface`.
fn hook_surface(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
    hook_root: &Path,
    rel_path: &str,
) -> Option<HookScriptSurface> {
    if let Some(entry) = entry(crawl, rel_path) {
        return Some(HookScriptSurface {
            rel_path: entry.path.rel_path.clone(),
            abs_path: entry.path.abs_path.clone(),
        });
    }
    let abs_path = hook_root.join(rel_path);
    abs_path.is_file().then(|| HookScriptSurface {
        rel_path: rel_path.to_owned(),
        abs_path,
    })
}

/// Internal function `normalized_hooks_path`.
fn normalized_hooks_path(hook_root: &Path, hooks_path: Option<&str>) -> Option<String> {
    let hooks_path = hooks_path?;
    let hooks_path = hooks_path.trim_end_matches('/');
    let path = Path::new(hooks_path);
    if path.is_absolute() {
        return path.strip_prefix(hook_root).ok().and_then(|path| {
            if path.as_os_str().is_empty() {
                Some(".".to_owned())
            } else {
                path.to_str().map(ToOwned::to_owned)
            }
        });
    }
    Some(
        hooks_path
            .strip_prefix("./")
            .unwrap_or(hooks_path)
            .to_owned(),
    )
}

/// Internal function `entry`.
fn entry<'a>(
    crawl: &'a workspace_crawl::G3WorkspaceCrawl,
    rel_path: &str,
) -> Option<&'a workspace_crawl::G3WorkspaceEntry> {
    crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
}

/// Internal function `has_entry_file`.
fn has_entry_file(crawl: &workspace_crawl::G3WorkspaceCrawl, rel_path: &str) -> bool {
    entry(crawl, rel_path)
        .is_some_and(|entry| entry.kind == workspace_crawl::G3WorkspaceEntryKind::File)
}

/// Internal function `has_entry_dir`.
fn has_entry_dir(crawl: &workspace_crawl::G3WorkspaceCrawl, rel_path: &str) -> bool {
    entry(crawl, rel_path)
        .is_some_and(|entry| entry.kind == workspace_crawl::G3WorkspaceEntryKind::Directory)
}

/// Internal function `direct_modular_entries`.
fn direct_modular_entries(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
    hook_root: &Path,
) -> Vec<HookScriptSurface> {
    let mut entries = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == workspace_crawl::G3WorkspaceEntryKind::File)
        .filter(|entry| direct_child(&entry.path.rel_path, ".githooks/pre-commit.d/"))
        .map(|entry| HookScriptSurface {
            rel_path: entry.path.rel_path.clone(),
            abs_path: entry.path.abs_path.clone(),
        })
        .collect::<Vec<_>>();
    let modular_root = hook_root.join(".githooks/pre-commit.d");
    for abs_path in direct_files(modular_root.as_path()) {
        if let Some(file_name) = abs_path.file_name().and_then(OsStr::to_str) {
            let rel_path = format!(".githooks/pre-commit.d/{file_name}");
            if !entries.iter().any(|entry| entry.rel_path == rel_path) {
                entries.push(HookScriptSurface { rel_path, abs_path });
            }
        }
    }
    entries.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    entries
}

/// Internal function `direct_file_names`.
fn direct_file_names(crawl: &workspace_crawl::G3WorkspaceCrawl, prefix: &str) -> Vec<String> {
    let mut names = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == workspace_crawl::G3WorkspaceEntryKind::File)
        .filter_map(|entry| {
            let suffix = entry.path.rel_path.strip_prefix(prefix)?;
            (!suffix.is_empty() && !suffix.contains('/')).then(|| suffix.to_owned())
        })
        .collect::<Vec<_>>();
    names.sort();
    names
}

/// Internal function `direct_child`.
fn direct_child(rel_path: &str, prefix: &str) -> bool {
    rel_path
        .strip_prefix(prefix)
        .is_some_and(|suffix| !suffix.is_empty() && !suffix.contains('/'))
}

/// Internal function `script_file_fact_from_surface`.
fn script_file_fact_from_surface(script: HookScriptSurface) -> hook_types::G3TsHooksScriptFileFact {
    script_file_fact_from_path(script.rel_path, script.abs_path.as_path())
}

/// Internal function `script_file_fact_from_path`.
fn script_file_fact_from_path(
    rel_path: String,
    abs_path: &Path,
) -> hook_types::G3TsHooksScriptFileFact {
    let content = read_to_string(abs_path);
    hook_types::G3TsHooksScriptFileFact::new(
        rel_path,
        content.lines().count(),
        content.len(),
        executable(abs_path),
    )
}

/// Internal function `discover_installed_tools`.
fn discover_installed_tools(path_env: Option<&OsStr>) -> Vec<String> {
    let mut tools = Vec::new();
    let Some(path_env) = path_env else {
        return tools;
    };
    for directory in std::env::split_paths(path_env) {
        for tool in ["g3ts", "pnpm", "npm", "yarn", "bun"] {
            let tool_path = directory.join(tool);
            if tool_path.is_file()
                && executable(tool_path.as_path()) == Some(true)
                && !tools.iter().any(|item| item == tool)
            {
                tools.push(tool.to_owned());
            }
        }
    }
    tools.sort();
    tools
}

/// Internal function `app_package_roots`.
fn app_package_roots(crawl: &workspace_crawl::G3WorkspaceCrawl) -> Vec<String> {
    let current_root = current_root(crawl);
    let mut roots = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == workspace_crawl::G3WorkspaceEntryKind::File)
        .filter_map(|entry| entry.path.rel_path.strip_suffix("/package.json"))
        .filter(|root| root.starts_with("apps/") || root.starts_with("packages/") || *root == ".")
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    if has_entry_file(crawl, "package.json") {
        roots.push(current_root);
    }
    roots.sort();
    roots.dedup();
    roots
}

/// Internal function `enabled_categories`.
fn enabled_categories(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
    category_roots: &[String],
) -> hook_types::G3TsHooksEnabledCategories {
    hook_types::G3TsHooksEnabledCategories::new(
        stylelint_enabled(crawl, category_roots),
        package_policy_enabled(crawl, category_roots),
        typecov_enabled(crawl, category_roots),
    )
}

/// Internal function `category_roots_for_selected_hook`.
fn category_roots_for_selected_hook(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
    hook_root: &Path,
    parsed: &hook_shell_parser::types::ParsedShellScript,
    app_roots: &[String],
) -> Vec<String> {
    let current_root = current_root(crawl);
    let scope_roots = verifier_scope_roots(hook_root, parsed);
    let mut roots = if scope_roots.is_empty() || scope_roots.iter().any(|root| root == ".") {
        app_roots.to_vec()
    } else {
        scope_roots
    };
    if roots.is_empty() && has_entry_file(crawl, "package.json") {
        roots.push(".".to_owned());
    }
    if has_entry_file(crawl, "package.json")
        && roots.iter().any(|root| root == current_root.as_str())
    {
        roots.push(".".to_owned());
    }
    roots.sort();
    roots.dedup();
    roots
}

/// Internal function `current_root`.
fn current_root(crawl: &workspace_crawl::G3WorkspaceCrawl) -> String {
    git_root(crawl.root_abs_path.as_path())
        .and_then(|git_root| {
            crawl
                .root_abs_path
                .strip_prefix(git_root)
                .ok()
                .map(Path::to_path_buf)
        })
        .and_then(|path| path.to_str().map(ToOwned::to_owned))
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| ".".to_owned())
}

/// Internal function `verifier_scope_roots`.
fn verifier_scope_roots(
    hook_root: &Path,
    parsed: &hook_shell_parser::types::ParsedShellScript,
) -> Vec<String> {
    let mut roots = parsed
        .executable_lines
        .iter()
        .filter_map(|line| {
            let words = shell_words(line.command_text.as_str());
            let command = words.first()?;
            if !matches!(
                command.as_str(),
                "scripts/g3ts/verify"
                    | "./scripts/g3ts/verify"
                    | "$REPO_ROOT/scripts/g3ts/verify"
                    | "${REPO_ROOT}/scripts/g3ts/verify"
            ) {
                return None;
            }
            let scope = words.windows(2).find_map(|window| match window {
                [flag, value] if flag == "--scope" => Some(value.as_str()),
                _ => None,
            })?;
            normalize_scope_root(hook_root, scope)
        })
        .collect::<Vec<_>>();
    roots.sort();
    roots.dedup();
    roots
}

/// Internal function `normalize_scope_root`.
fn normalize_scope_root(hook_root: &Path, scope: &str) -> Option<String> {
    if scope == "." || scope == "$REPO_ROOT" || scope == "${REPO_ROOT}" {
        return Some(".".to_owned());
    }
    if let Some(scope) = scope
        .strip_prefix("$REPO_ROOT/")
        .or_else(|| scope.strip_prefix("${REPO_ROOT}/"))
    {
        return Some(scope.to_owned());
    }
    let path = Path::new(scope);
    if path.is_absolute() {
        return path
            .strip_prefix(hook_root)
            .ok()
            .and_then(|path| path.to_str())
            .map(ToOwned::to_owned);
    }
    Some(scope.strip_prefix("./").unwrap_or(scope).to_owned())
}

/// Internal function `stylelint_enabled`.
fn stylelint_enabled(crawl: &workspace_crawl::G3WorkspaceCrawl, app_roots: &[String]) -> bool {
    app_roots.iter().any(|root| {
        [
            "stylelint.config.js",
            "stylelint.config.mjs",
            "stylelint.config.cjs",
            "stylelint.config.ts",
            ".stylelintrc",
            ".stylelintrc.json",
            ".stylelintrc.yaml",
            ".stylelintrc.yml",
            ".stylelintrc.js",
            ".stylelintrc.cjs",
            ".stylelintrc.mjs",
        ]
        .iter()
        .any(|file| has_entry_file(crawl, root_file(root, file).as_str()))
    })
}

/// Internal function `package_policy_enabled`.
fn package_policy_enabled(crawl: &workspace_crawl::G3WorkspaceCrawl, app_roots: &[String]) -> bool {
    app_roots
        .iter()
        .any(|root| has_entry_file(crawl, root_file(root, "package.json").as_str()))
}

/// Internal function `typecov_enabled`.
fn typecov_enabled(crawl: &workspace_crawl::G3WorkspaceCrawl, app_roots: &[String]) -> bool {
    app_roots.iter().any(|root| {
        package_content(crawl, root).is_some_and(|content| {
            content.contains("\"typecov\"") || content.contains("\"type-coverage\"")
        }) || [
            "type-coverage.json",
            "type-coverage.config.js",
            "type-coverage.config.mjs",
            "type-coverage.config.cjs",
            "type-coverage.config.ts",
        ]
        .iter()
        .any(|file| has_entry_file(crawl, root_file(root, file).as_str()))
    })
}

/// Internal function `package_content`.
fn package_content(crawl: &workspace_crawl::G3WorkspaceCrawl, root: &str) -> Option<String> {
    let rel_path = root_file(root, "package.json");
    entry(crawl, rel_path.as_str()).map(|entry| read_to_string(entry.path.abs_path.as_path()))
}

/// Internal function `root_file`.
fn root_file(root: &str, file: &str) -> String {
    if root == "." {
        file.to_owned()
    } else {
        format!("{root}/{file}")
    }
}

/// Internal function `trust_risks`.
fn trust_risks(crawl: &workspace_crawl::G3WorkspaceCrawl) -> Vec<String> {
    [".husky/pre-commit", "lefthook.yml", ".lintstagedrc"]
        .into_iter()
        .filter(|path| has_entry_file(crawl, path))
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
