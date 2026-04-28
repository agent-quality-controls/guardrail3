use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use g3_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};
use g3ts_hooks_types::{
    G3TsHookScriptKind, G3TsHooksConfigChecksInput, G3TsHooksFileTreeChecksInput,
    G3TsHooksScriptFileFact, G3TsHooksSelectedHookConfigFact, G3TsHooksSourceChecksInput,
};
use hook_shell_parser::parse_script;

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
pub fn ingest_for_source_checks(crawl: &G3RsWorkspaceCrawl) -> Vec<G3TsHooksSourceChecksInput> {
    let hook_root =
        git_root(crawl.root_abs_path.as_path()).unwrap_or_else(|| crawl.root_abs_path.clone());
    let hooks_path = read_hooks_path(hook_root.as_path());
    let Some(selected) =
        select_pre_commit_surface(crawl, hook_root.as_path(), hooks_path.as_deref())
    else {
        return Vec::new();
    };
    let app_package_roots = app_package_roots(crawl);
    let mut inputs = Vec::new();
    let content = read_path(selected.abs_path.as_path());
    inputs.push(G3TsHooksSourceChecksInput {
        rel_path: selected.rel_path.clone(),
        kind: G3TsHookScriptKind::PreCommit,
        parsed: parse_script(content.as_str()),
        has_modular_dir: selected.has_modular_dir,
        app_package_roots: app_package_roots.clone(),
        requirements: Vec::new(),
    });
    if selected.has_modular_dir && pre_commit_dispatches_modular_scripts(content.as_str()) {
        for script in direct_modular_entries(crawl, hook_root.as_path()) {
            let content = read_path(script.abs_path.as_path());
            inputs.push(G3TsHooksSourceChecksInput {
                rel_path: script.rel_path,
                kind: G3TsHookScriptKind::Modular,
                parsed: parse_script(content.as_str()),
                has_modular_dir: selected.has_modular_dir,
                app_package_roots: app_package_roots.clone(),
                requirements: Vec::new(),
            });
        }
    }
    inputs
}

fn pre_commit_dispatches_modular_scripts(content: &str) -> bool {
    parse_script(content).executable_lines.iter().any(|line| {
        line.is_dispatcher_syntax && dispatcher_invokes_modular_directory(&line.command_text)
    })
}

fn dispatcher_invokes_modular_directory(command_text: &str) -> bool {
    let words = hook_shell_parser::command_query::shell_words(command_text);
    let Some(command) = words.first().map(String::as_str) else {
        return false;
    };
    match command {
        "run-parts" => words
            .iter()
            .skip(1)
            .any(|word| word.trim_end_matches('/') == ".githooks/pre-commit.d"),
        "." | "source" => words
            .iter()
            .skip(1)
            .any(|word| word == ".githooks/pre-commit.d"),
        _ => false,
    }
}

#[must_use]
pub fn ingest_for_file_tree_checks(crawl: &G3RsWorkspaceCrawl) -> G3TsHooksFileTreeChecksInput {
    let active = hooks_scope_is_active(crawl);
    if !active {
        return G3TsHooksFileTreeChecksInput {
            active,
            pre_commit: None,
            has_modular_dir: false,
            modular_scripts: Vec::new(),
            local_override_scripts: Vec::new(),
            hooks_path: None,
            trust_risks: Vec::new(),
        };
    }
    let hook_root =
        git_root(crawl.root_abs_path.as_path()).unwrap_or_else(|| crawl.root_abs_path.clone());
    let hooks_path = read_hooks_path(hook_root.as_path());
    let normalized_hooks_path = normalized_hooks_path(hooks_path.as_deref()).map(ToOwned::to_owned);
    let selected = select_pre_commit_surface(crawl, hook_root.as_path(), hooks_path.as_deref());
    let has_modular_dir = hook_root.join(".githooks/pre-commit.d").is_dir()
        || has_entry_dir(crawl, ".githooks/pre-commit.d");
    G3TsHooksFileTreeChecksInput {
        active,
        pre_commit: selected.map(|surface| {
            script_file_fact_from_path(surface.rel_path, surface.abs_path.as_path())
        }),
        has_modular_dir,
        modular_scripts: if has_modular_dir {
            direct_modular_entries(crawl, hook_root.as_path())
                .into_iter()
                .map(script_file_fact_from_surface)
                .collect()
        } else {
            Vec::new()
        },
        local_override_scripts: direct_file_names(crawl, ".guardrail3/overrides/pre-commit.d/"),
        hooks_path: normalized_hooks_path,
        trust_risks: trust_risks(crawl),
    }
}

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3RsWorkspaceCrawl) -> G3TsHooksConfigChecksInput {
    ingest_for_config_checks_with_path(crawl, std::env::var_os("PATH").as_deref())
}

#[must_use]
pub fn ingest_for_config_checks_with_path(
    crawl: &G3RsWorkspaceCrawl,
    path_env: Option<&OsStr>,
) -> G3TsHooksConfigChecksInput {
    let active = hooks_scope_is_active(crawl);
    let hook_root =
        git_root(crawl.root_abs_path.as_path()).unwrap_or_else(|| crawl.root_abs_path.clone());
    let hooks_path = read_hooks_path(hook_root.as_path());
    G3TsHooksConfigChecksInput {
        active,
        selected_hook: select_pre_commit_surface(crawl, hook_root.as_path(), hooks_path.as_deref())
            .map(|surface| G3TsHooksSelectedHookConfigFact {
                rel_path: surface.rel_path,
                parsed: parse_script(read_path(surface.abs_path.as_path()).as_str()),
            }),
        installed_tools: discover_installed_tools(path_env),
        requirements: Vec::new(),
    }
}

fn hooks_scope_is_active(crawl: &G3RsWorkspaceCrawl) -> bool {
    has_entry_file(crawl, "package.json") || !app_package_roots(crawl).is_empty()
}

fn select_pre_commit_surface(
    crawl: &G3RsWorkspaceCrawl,
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
    crawl: &G3RsWorkspaceCrawl,
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

fn entry<'a>(crawl: &'a G3RsWorkspaceCrawl, rel_path: &str) -> Option<&'a G3RsWorkspaceEntry> {
    crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
}

fn has_entry_file(crawl: &G3RsWorkspaceCrawl, rel_path: &str) -> bool {
    entry(crawl, rel_path).is_some_and(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
}

fn has_entry_dir(crawl: &G3RsWorkspaceCrawl, rel_path: &str) -> bool {
    entry(crawl, rel_path).is_some_and(|entry| entry.kind == G3RsWorkspaceEntryKind::Directory)
}

fn direct_modular_entries(crawl: &G3RsWorkspaceCrawl, hook_root: &Path) -> Vec<HookScriptSurface> {
    let mut entries = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| direct_child(&entry.path.rel_path, ".githooks/pre-commit.d/"))
        .map(|entry| HookScriptSurface {
            rel_path: entry.path.rel_path.clone(),
            abs_path: entry.path.abs_path.clone(),
        })
        .collect::<Vec<_>>();
    let modular_root = hook_root.join(".githooks/pre-commit.d");
    if let Ok(read_dir) = std::fs::read_dir(modular_root) {
        for entry in read_dir.flatten() {
            let abs_path = entry.path();
            if abs_path.is_file()
                && let Some(file_name) = abs_path.file_name().and_then(OsStr::to_str)
                && !file_name.contains('/')
            {
                let rel_path = format!(".githooks/pre-commit.d/{file_name}");
                if !entries.iter().any(|entry| entry.rel_path == rel_path) {
                    entries.push(HookScriptSurface { rel_path, abs_path });
                }
            }
        }
    }
    entries.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    entries
}

fn direct_file_names(crawl: &G3RsWorkspaceCrawl, prefix: &str) -> Vec<String> {
    let mut names = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
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

fn script_file_fact_from_surface(script: HookScriptSurface) -> G3TsHooksScriptFileFact {
    script_file_fact_from_path(script.rel_path, script.abs_path.as_path())
}

fn script_file_fact_from_path(rel_path: String, abs_path: &Path) -> G3TsHooksScriptFileFact {
    let content = read_path(abs_path);
    G3TsHooksScriptFileFact {
        rel_path,
        line_count: content.lines().count(),
        byte_count: content.len(),
        executable: executable(abs_path),
    }
}

fn executable(path: &Path) -> Option<bool> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = std::fs::metadata(path).ok()?.permissions().mode();
        Some(mode & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        None
    }
}

fn read_path(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

fn read_hooks_path(root: &Path) -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["config", "--get", "core.hooksPath"])
        .current_dir(root)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn git_root(root: &Path) -> Option<PathBuf> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(root)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| PathBuf::from(String::from_utf8_lossy(&output.stdout).trim().to_owned()))
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

fn app_package_roots(crawl: &G3RsWorkspaceCrawl) -> Vec<String> {
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
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
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

fn trust_risks(crawl: &G3RsWorkspaceCrawl) -> Vec<String> {
    [".husky/pre-commit", "lefthook.yml", ".lintstagedrc"]
        .into_iter()
        .filter(|path| has_entry_file(crawl, path))
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use g3ts_hooks_types::G3TsHookScriptKind;

    fn temp_root(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("g3ts-hooks-ingestion-{test_name}-{unique}"));
        fs::create_dir_all(&path).expect("create temp fixture root");
        path
    }

    fn write(root: &Path, rel_path: &str, content: &str) {
        let path = root.join(rel_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create fixture parent");
        }
        fs::write(path, content).expect("write fixture");
    }

    #[test]
    fn source_ingestion_ignores_undispatched_modular_scripts() {
        let root = temp_root("undispatched");
        write(&root, "package.json", "{}\n");
        write(
            &root,
            ".githooks/pre-commit",
            "#!/usr/bin/env bash\necho not dispatching\n",
        );
        write(
            &root,
            ".githooks/pre-commit.d/10-typescript.sh",
            "g3ts validate --path apps/landing\npnpm --filter landing run validate\n",
        );

        let crawl = g3_workspace_crawl::crawl(&root).expect("crawl fixture");
        let inputs = super::ingest_for_source_checks(&crawl);

        assert_eq!(inputs.len(), 1, "{inputs:#?}");
        assert_eq!(inputs[0].kind, G3TsHookScriptKind::PreCommit);
        assert_eq!(inputs[0].rel_path, ".githooks/pre-commit");
    }

    #[test]
    fn source_ingestion_includes_dispatched_modular_scripts() {
        let root = temp_root("dispatched");
        write(&root, "package.json", "{}\n");
        write(
            &root,
            ".githooks/pre-commit",
            "#!/usr/bin/env bash\nrun-parts .githooks/pre-commit.d\n",
        );
        write(
            &root,
            ".githooks/pre-commit.d/10-typescript.sh",
            "g3ts validate --path apps/landing\npnpm --filter landing run validate\n",
        );

        let crawl = g3_workspace_crawl::crawl(&root).expect("crawl fixture");
        let inputs = super::ingest_for_source_checks(&crawl);

        assert_eq!(inputs.len(), 2, "{inputs:#?}");
        assert!(
            inputs
                .iter()
                .any(|input| input.kind == G3TsHookScriptKind::Modular
                    && input.rel_path == ".githooks/pre-commit.d/10-typescript.sh"),
            "{inputs:#?}"
        );
    }
}
