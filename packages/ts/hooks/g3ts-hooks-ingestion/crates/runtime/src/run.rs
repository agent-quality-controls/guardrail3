use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use g3_workspace_crawl as workspace_crawl;
use g3ts_hooks_types as hook_types;
use hook_shell_parser::parse_script;

use crate::fs::{direct_files, executable, read_to_string};
use crate::process::{git_root, read_hooks_path};

#[derive(Debug)]
struct SelectedHookSurface {
    rel_path: String,
    abs_path: PathBuf,
    has_modular_dir: bool,
}

#[derive(Debug)]
struct HookScriptSurface {
    rel_path: String,
    abs_path: PathBuf,
}

#[must_use]
pub fn ingest_for_source_checks(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
) -> Vec<hook_types::G3TsHooksSourceChecksInput> {
    let hook_root =
        git_root(crawl.root_abs_path.as_path()).unwrap_or_else(|| crawl.root_abs_path.clone());
    let hooks_path = read_hooks_path(hook_root.as_path());
    let Some(selected) =
        select_pre_commit_surface(crawl, hook_root.as_path(), hooks_path.as_deref())
    else {
        return Vec::new();
    };
    let app_package_roots = app_package_roots(crawl);
    let enabled_categories = enabled_categories(crawl, &app_package_roots);
    let mut inputs = Vec::new();
    let content = read_to_string(selected.abs_path.as_path());
    inputs.push(hook_types::G3TsHooksSourceChecksInput::new(
        selected.rel_path.clone(),
        hook_types::G3TsHookScriptKind::PreCommit,
        parse_script(content.as_str()),
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

fn verifier_surface(crawl: &workspace_crawl::G3RsWorkspaceCrawl, hook_root: &Path) -> Option<HookScriptSurface> {
    hook_surface(crawl, hook_root, "scripts/g3ts/verify")
}

#[must_use]
pub fn ingest_for_file_tree_checks(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
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
    let normalized_hooks_path = normalized_hooks_path(hooks_path.as_deref()).map(ToOwned::to_owned);
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

#[must_use]
pub fn ingest_for_config_checks(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
) -> hook_types::G3TsHooksConfigChecksInput {
    ingest_for_config_checks_with_path(crawl, std::env::var_os("PATH").as_deref())
}

#[must_use]
pub fn ingest_for_config_checks_with_path(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
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

fn hooks_scope_is_active(crawl: &workspace_crawl::G3RsWorkspaceCrawl) -> bool {
    has_entry_file(crawl, "package.json") || !app_package_roots(crawl).is_empty()
}

fn select_pre_commit_surface(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
    hook_root: &Path,
    hooks_path: Option<&str>,
) -> Option<SelectedHookSurface> {
    let surface = match normalized_hooks_path(hooks_path) {
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

fn hook_surface(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
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

fn normalized_hooks_path(hooks_path: Option<&str>) -> Option<&str> {
    let hooks_path = hooks_path?;
    let hooks_path = hooks_path.trim_end_matches('/');
    Some(hooks_path.strip_prefix("./").unwrap_or(hooks_path))
}

fn entry<'a>(
    crawl: &'a workspace_crawl::G3RsWorkspaceCrawl,
    rel_path: &str,
) -> Option<&'a workspace_crawl::G3RsWorkspaceEntry> {
    crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
}

fn has_entry_file(crawl: &workspace_crawl::G3RsWorkspaceCrawl, rel_path: &str) -> bool {
    entry(crawl, rel_path)
        .is_some_and(|entry| entry.kind == workspace_crawl::G3RsWorkspaceEntryKind::File)
}

fn has_entry_dir(crawl: &workspace_crawl::G3RsWorkspaceCrawl, rel_path: &str) -> bool {
    entry(crawl, rel_path)
        .is_some_and(|entry| entry.kind == workspace_crawl::G3RsWorkspaceEntryKind::Directory)
}

fn direct_modular_entries(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
    hook_root: &Path,
) -> Vec<HookScriptSurface> {
    let mut entries = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == workspace_crawl::G3RsWorkspaceEntryKind::File)
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

fn direct_file_names(crawl: &workspace_crawl::G3RsWorkspaceCrawl, prefix: &str) -> Vec<String> {
    let mut names = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == workspace_crawl::G3RsWorkspaceEntryKind::File)
        .filter_map(|entry| {
            let suffix = entry.path.rel_path.strip_prefix(prefix)?;
            (!suffix.is_empty() && !suffix.contains('/')).then(|| suffix.to_owned())
        })
        .collect::<Vec<_>>();
    names.sort();
    names
}

fn direct_child(rel_path: &str, prefix: &str) -> bool {
    rel_path
        .strip_prefix(prefix)
        .is_some_and(|suffix| !suffix.is_empty() && !suffix.contains('/'))
}

fn script_file_fact_from_surface(script: HookScriptSurface) -> hook_types::G3TsHooksScriptFileFact {
    script_file_fact_from_path(script.rel_path, script.abs_path.as_path())
}

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

fn app_package_roots(crawl: &workspace_crawl::G3RsWorkspaceCrawl) -> Vec<String> {
    let current_root = git_root(crawl.root_abs_path.as_path())
        .and_then(|git_root| {
            crawl
                .root_abs_path
                .strip_prefix(git_root)
                .ok()
                .map(Path::to_path_buf)
        })
        .and_then(|path| path.to_str().map(ToOwned::to_owned))
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| ".".to_owned());
    let mut roots = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == workspace_crawl::G3RsWorkspaceEntryKind::File)
        .filter_map(|entry| entry.path.rel_path.strip_suffix("/package.json"))
        .filter(|root| root.starts_with("apps/") || *root == ".")
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    if has_entry_file(crawl, "package.json") {
        roots.push(current_root);
    }
    roots.sort();
    roots.dedup();
    roots
}

fn enabled_categories(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
    app_roots: &[String],
) -> hook_types::G3TsHooksEnabledCategories {
    let category_roots = category_roots(crawl, app_roots);
    hook_types::G3TsHooksEnabledCategories::new(
        stylelint_enabled(crawl, &category_roots),
        package_policy_enabled(crawl, &category_roots),
        typecov_enabled(crawl, &category_roots),
    )
}

fn category_roots(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
    app_roots: &[String],
) -> Vec<String> {
    let mut roots = app_roots.to_vec();
    if has_entry_file(crawl, "package.json") {
        roots.push(".".to_owned());
    }
    roots.sort();
    roots.dedup();
    roots
}

fn stylelint_enabled(crawl: &workspace_crawl::G3RsWorkspaceCrawl, app_roots: &[String]) -> bool {
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

fn package_policy_enabled(crawl: &workspace_crawl::G3RsWorkspaceCrawl, app_roots: &[String]) -> bool {
    app_roots
        .iter()
        .any(|root| has_entry_file(crawl, root_file(root, "package.json").as_str()))
}

fn typecov_enabled(crawl: &workspace_crawl::G3RsWorkspaceCrawl, app_roots: &[String]) -> bool {
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

fn package_content(crawl: &workspace_crawl::G3RsWorkspaceCrawl, root: &str) -> Option<String> {
    let rel_path = root_file(root, "package.json");
    entry(crawl, rel_path.as_str()).map(|entry| read_to_string(entry.path.abs_path.as_path()))
}

fn root_file(root: &str, file: &str) -> String {
    if root == "." {
        file.to_owned()
    } else {
        format!("{root}/{file}")
    }
}

fn trust_risks(crawl: &workspace_crawl::G3RsWorkspaceCrawl) -> Vec<String> {
    [".husky/pre-commit", "lefthook.yml", ".lintstagedrc"]
        .into_iter()
        .filter(|path| has_entry_file(crawl, path))
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
