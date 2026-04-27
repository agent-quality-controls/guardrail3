use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_workspace_crawl_assertions::run as assertions;
use tempfile::tempdir;

/// Initialize a git repo at the given path so the ignore crate's WalkBuilder
/// can find .gitignore files and compute ignore state.
fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}

#[test]
fn marks_gitignored_files_as_included_via_recovery() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".gitignore"), "Cargo.lock\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
    write(root.join("Cargo.lock"), "# lock\n");
    write(root.join("src/lib.rs"), "");

    let crawl = crate::run::crawl(root).expect("crawl should succeed");

    // Cargo.lock is ignored but recoverable — should appear as Ignored
    assertions::assert_has_rel_path(&crawl.entries, "Cargo.lock");
    assertions::assert_crawl_entry(
        &crawl,
        "Cargo.lock",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Ignored,
        true,
    );

    // Cargo.toml is not ignored
    assertions::assert_crawl_entry(
        &crawl,
        "Cargo.toml",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

#[test]
fn ignored_non_recoverable_files_do_not_appear() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".gitignore"), "*.log\n");
    write(root.join("debug.log"), "some log\n");
    write(root.join("src/lib.rs"), "");

    let crawl = crate::run::crawl(root).expect("crawl should succeed");

    // debug.log is ignored and not on the recovery list — absent
    assertions::assert_crawl_entry_absent(&crawl, "debug.log");
}

#[test]
fn nested_gitignore_is_respected() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/lib.rs"), "");
    write(root.join("src/.gitignore"), "*.tmp\n");
    write(root.join("src/temp.tmp"), "junk");
    write(
        root.join("root.tmp"),
        "also tmp but not ignored by nested rule",
    );

    let crawl = crate::run::crawl(root).expect("crawl should succeed");

    // src/temp.tmp is ignored by src/.gitignore — not recoverable, absent
    assertions::assert_crawl_entry_absent(&crawl, "src/temp.tmp");

    // root.tmp is NOT ignored (the nested .gitignore only applies to src/)
    assertions::assert_crawl_entry(
        &crawl,
        "root.tmp",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

#[test]
fn ancestor_gitignore_is_respected() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let repo_root = temp_dir.path();
    git_init(repo_root);

    // Parent .gitignore at repo root ignores *.generated
    write(repo_root.join(".gitignore"), "*.generated\n");

    // Workspace is a subdirectory of the repo
    let workspace = repo_root.join("packages/demo");
    fs::create_dir_all(&workspace).expect("create workspace dir");
    write(workspace.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
    write(workspace.join("src/lib.rs"), "");
    write(workspace.join("output.generated"), "generated file");

    let crawl = crate::run::crawl(&workspace).expect("crawl should succeed");

    // output.generated is ignored by ancestor .gitignore — not recoverable, absent
    assertions::assert_crawl_entry_absent(&crawl, "output.generated");

    // Cargo.toml is included
    assertions::assert_crawl_entry(
        &crawl,
        "Cargo.toml",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

#[test]
fn negation_pattern_unignores_file() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".gitignore"), "*.log\n!important.log\n");
    write(root.join("debug.log"), "ignored log");
    write(root.join("important.log"), "keep this");

    let crawl = crate::run::crawl(root).expect("crawl should succeed");

    // important.log is unignored by negation
    assertions::assert_crawl_entry(
        &crawl,
        "important.log",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Included,
        true,
    );

    // debug.log is ignored and not recoverable — absent
    assertions::assert_crawl_entry_absent(&crawl, "debug.log");
}

#[test]
fn hidden_dotfiles_are_included_normally() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join(".rustfmt.toml"), "edition = \"2024\"\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");

    let crawl = crate::run::crawl(root).expect("crawl should succeed");

    assertions::assert_crawl_entry(
        &crawl,
        ".clippy.toml",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Included,
        true,
    );
    assertions::assert_crawl_entry(
        &crawl,
        ".rustfmt.toml",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

#[test]
fn banned_directories_are_excluded_from_recovery() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".gitignore"), "target/\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
    // Cargo.toml inside target/ should NOT be recovered
    fs::create_dir_all(root.join("target")).expect("create target dir");
    write(
        root.join("target/Cargo.toml"),
        "[package]\nname = \"build-artifact\"\n",
    );

    let crawl = crate::run::crawl(root).expect("crawl should succeed");

    assertions::assert_crawl_entry_absent(&crawl, "target/Cargo.toml");
    assertions::assert_crawl_entry_absent(&crawl, "target");
}

#[test]
fn recovery_finds_ignored_config_in_non_banned_directory() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    // .cargo/ is ignored but not banned
    write(root.join(".gitignore"), ".cargo/\n");
    fs::create_dir_all(root.join(".cargo")).expect("create .cargo dir");
    write(root.join(".cargo/config.toml"), "[build]\njobs = 4\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");

    let crawl = crate::run::crawl(root).expect("crawl should succeed");

    // .cargo/config.toml is ignored but recoverable
    assertions::assert_has_rel_path(&crawl.entries, ".cargo/config.toml");
    assertions::assert_crawl_entry(
        &crawl,
        ".cargo/config.toml",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Ignored,
        true,
    );
}

#[test]
fn recovery_finds_ignored_generated_state_directory_sentinels() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join(".gitignore"),
        ".next/\n.velite/\n.contentlayer/\n",
    );
    fs::create_dir_all(root.join(".next/server/app")).expect("create .next output");
    fs::create_dir_all(root.join(".velite")).expect("create .velite output");
    fs::create_dir_all(root.join(".contentlayer/generated")).expect("create .contentlayer output");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");

    let crawl = crate::run::crawl(root).expect("crawl should succeed");

    assertions::assert_crawl_entry(
        &crawl,
        ".next",
        crate::G3RsWorkspaceEntryKind::Directory,
        crate::G3RsWorkspaceIgnoreState::Ignored,
        true,
    );
    assertions::assert_crawl_entry(
        &crawl,
        ".velite",
        crate::G3RsWorkspaceEntryKind::Directory,
        crate::G3RsWorkspaceIgnoreState::Ignored,
        true,
    );
    assertions::assert_crawl_entry(
        &crawl,
        ".contentlayer",
        crate::G3RsWorkspaceEntryKind::Directory,
        crate::G3RsWorkspaceIgnoreState::Ignored,
        true,
    );
}

#[test]
fn recovery_uses_guardrail3_rs_toml_and_not_dead_guardrail3_toml() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join(".gitignore"),
        "guardrail3-rs.toml\nguardrail3.toml\n",
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"service\"\n");
    write(
        root.join("guardrail3.toml"),
        "[profile]\nname = \"service\"\n",
    );

    let crawl = crate::run::crawl(root).expect("crawl should succeed");

    assertions::assert_crawl_entry(
        &crawl,
        "guardrail3-rs.toml",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Ignored,
        true,
    );
    assertions::assert_crawl_entry_absent(&crawl, "guardrail3.toml");
}

#[test]
fn golden_baseline_no_gitignore() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
    write(root.join("src/lib.rs"), "pub fn demo() {}\n");
    write(root.join("README.md"), "# demo\n");

    let crawl =
        crate::run::crawl(root).expect("crawl should succeed for workspace with no .gitignore");

    for entry in &crawl.entries {
        assert_eq!(
            entry.ignore_state,
            crate::G3RsWorkspaceIgnoreState::Included,
            "every entry should be Included when no .gitignore exists, but {rel} was not",
            rel = entry.path.rel_path,
        );
    }
    // Verify all expected files are present
    assertions::assert_crawl_entry_exists(&crawl, "Cargo.toml");
    assertions::assert_crawl_entry_exists(&crawl, "src/lib.rs");
    assertions::assert_crawl_entry_exists(&crawl, "README.md");
}

#[test]
fn directory_only_gitignore_pattern() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".gitignore"), "build/\n");
    write(root.join("build/output.txt"), "artifact");
    write(root.join("build-notes.txt"), "keep this");
    write(root.join("src/lib.rs"), "");

    let crawl = crate::run::crawl(root)
        .expect("crawl should succeed with directory-only gitignore pattern");

    // build/ directory and its contents should not appear (ignored, not recoverable)
    assertions::assert_crawl_entry_absent(&crawl, "build");
    assertions::assert_crawl_entry_absent(&crawl, "build/output.txt");

    // build-notes.txt should be included — the trailing slash means the pattern
    // only matches directories named "build", not files with "build" prefix
    assertions::assert_crawl_entry(
        &crawl,
        "build-notes.txt",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

#[test]
fn non_git_workspace_includes_everything() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    // Deliberately no git_init — testing behavior without a git repository

    write(root.join(".gitignore"), "*.log\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
    write(root.join("src/lib.rs"), "");
    write(root.join("debug.log"), "some log");

    let crawl = crate::run::crawl(root).expect("crawl should succeed in non-git workspace");

    // Without .git, the ignore crate's require_git default (true) means
    // .gitignore rules may or may not be applied depending on whether a
    // .git exists above the temp dir. Verify core files are present.
    assertions::assert_crawl_entry_exists(&crawl, ".gitignore");
    assertions::assert_crawl_entry_exists(&crawl, "Cargo.toml");
    assertions::assert_crawl_entry_exists(&crawl, "src/lib.rs");
    // debug.log may or may not be ignored depending on whether a .git dir
    // exists above the tempdir. We don't assert on it to avoid test flakiness.
}

#[test]
fn claude_worktrees_banned_from_recovery() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".gitignore"), ".claude/\n");
    write(
        root.join(".claude/worktrees/Cargo.toml"),
        "[package]\nname = \"worktree\"\n",
    );
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");

    let crawl =
        crate::run::crawl(root).expect("crawl should succeed with .claude/worktrees banned");

    // .claude/worktrees/ is a banned root — Cargo.toml inside it should NOT
    // be recovered even though Cargo.toml is on the recovery list
    assertions::assert_crawl_entry_absent(&crawl, ".claude/worktrees/Cargo.toml");
    assertions::assert_crawl_entry_absent(&crawl, ".claude");
}

#[test]
fn banned_dirs_excluded_from_phase1_without_gitignore() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    // node_modules is NOT in .gitignore, but is a banned dir name.
    // Phase 1's filter_entry should exclude it even without gitignore.
    fs::create_dir_all(root.join("node_modules")).expect("create node_modules dir");
    write(
        root.join("node_modules/package.json"),
        "{\"name\": \"dep\"}\n",
    );
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");

    let crawl =
        crate::run::crawl(root).expect("crawl should succeed with un-gitignored banned directory");

    // node_modules/ should not appear as Included despite not being gitignored
    assertions::assert_crawl_entry_absent(&crawl, "node_modules");
    assertions::assert_crawl_entry_absent(&crawl, "node_modules/package.json");
    // Cargo.toml at root should still be included
    assertions::assert_crawl_entry_exists(&crawl, "Cargo.toml");
}
