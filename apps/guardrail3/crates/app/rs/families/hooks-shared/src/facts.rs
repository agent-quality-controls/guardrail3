use std::path::Path;

use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;
use guardrail3_outbound_traits::FileSystem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookScriptKind {
    PreCommit,
    Modular,
}

#[derive(Debug, Clone)]
pub struct HookScriptFacts {
    pub(crate) rel_path: String,
    pub(crate) kind: HookScriptKind,
    pub(crate) content: String,
}

#[derive(Debug, Default)]
pub struct SharedHookFacts {
    pub(crate) pre_commit: Option<HookScriptFacts>,
    pub(crate) modular_scripts: Vec<HookScriptFacts>,
    pub(crate) has_modular_dir: bool,
    pub(crate) local_override_scripts: Vec<String>,
    pub(crate) hooks_path: Option<String>,
    pub(crate) pre_commit_executable: Option<bool>,
    pub(crate) modular_executable: Vec<(String, bool)>,
    pub(crate) trust_risks: Vec<String>,
}

pub fn collect(fs: &dyn FileSystem, root: &Path, tree: &ProjectTree) -> SharedHookFacts {
    let pre_commit = [".githooks/pre-commit", "hooks/pre-commit"]
        .into_iter()
        .find_map(|rel_path| {
            tree.file_content(rel_path).map(|content| HookScriptFacts {
                rel_path: rel_path.to_owned(),
                kind: HookScriptKind::PreCommit,
                content: content.to_owned(),
            })
        });

    let mut modular_scripts = Vec::new();
    let has_modular_dir = tree.dir_exists(".githooks/pre-commit.d");
    if let Some(dir) = tree.dir_contents(".githooks/pre-commit.d") {
        for file_name in dir.files() {
            let rel_path = ProjectTree::join_rel(".githooks/pre-commit.d", file_name);
            let content =
                guardrail3_shared_fs::read_file_err(&tree.abs_path(&rel_path)).unwrap_or_default();
            modular_scripts.push(HookScriptFacts {
                rel_path,
                kind: HookScriptKind::Modular,
                content,
            });
        }
        modular_scripts.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    }

    let mut local_override_scripts = Vec::new();
    if let Some(dir) = tree.dir_contents(".guardrail3/overrides/pre-commit.d") {
        local_override_scripts.extend(dir.files().iter().cloned());
        local_override_scripts.sort();
    }

    let hooks_path = read_hooks_path(root);
    let pre_commit_executable = pre_commit
        .as_ref()
        .and_then(|script| executable_bit(fs, &tree.abs_path(&script.rel_path)));
    let mut modular_executable = Vec::new();
    for script in &modular_scripts {
        if let Some(is_executable) = executable_bit(fs, &tree.abs_path(&script.rel_path)) {
            modular_executable.push((script.rel_path.clone(), is_executable));
        }
    }
    let trust_risks = collect_trust_risks(fs, root, tree, hooks_path.as_deref());

    SharedHookFacts {
        pre_commit,
        modular_scripts,
        has_modular_dir,
        local_override_scripts,
        hooks_path,
        pre_commit_executable,
        modular_executable,
        trust_risks,
    }
}

fn read_hooks_path(root: &Path) -> Option<String> {
    #[allow(clippy::disallowed_methods)]
    let output = std::process::Command::new("git")
        .args(["config", "core.hooksPath"])
        .current_dir(root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn executable_bit(fs: &dyn FileSystem, path: &Path) -> Option<bool> {
    #[cfg(unix)]
    {
        let metadata = fs.metadata(path)?;
        Some(metadata.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        let _ = (fs, path);
        None
    }
}

fn collect_trust_risks(
    fs: &dyn FileSystem,
    root: &Path,
    tree: &ProjectTree,
    hooks_path: Option<&str>,
) -> Vec<String> {
    let mut risks = Vec::new();

    if tree.file_exists(".husky/pre-commit") {
        risks.push(".husky/pre-commit".to_owned());
    }
    for rel_path in [
        "lefthook.yml",
        "lefthook.yaml",
        ".lefthook.yml",
        ".lefthook.yaml",
    ] {
        if tree.file_exists(rel_path) {
            risks.push(rel_path.to_owned());
        }
    }

    let git_hook_path = root.join(".git/hooks/pre-commit");
    if fs.metadata(&git_hook_path).is_some() && hooks_path != Some(".githooks") {
        risks.push(".git/hooks/pre-commit".to_owned());
    }

    risks.sort();
    risks
}
