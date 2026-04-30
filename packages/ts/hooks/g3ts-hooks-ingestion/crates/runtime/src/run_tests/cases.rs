use g3ts_hooks_ingestion_assertions::run as assertions;

use super::helpers::{git_init, temp_root, write};

#[test]
fn source_ingestion_ignores_undispatched_modular_scripts() {
    let root = temp_root("undispatched");
    write(&root, "Cargo.toml", "[workspace]\nresolver = \"2\"\n");
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

    let crawl = g3_workspace_crawl::crawl(&root).expect("crawl fixture workspace");
    let inputs = super::super::ingest_for_source_checks(&crawl);

    assertions::assert_only_pre_commit_script(&inputs);
}

#[test]
fn source_ingestion_includes_dispatched_modular_scripts() {
    let root = temp_root("dispatched");
    write(&root, "Cargo.toml", "[workspace]\nresolver = \"2\"\n");
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

    let crawl = g3_workspace_crawl::crawl(&root).expect("crawl fixture workspace");
    let inputs = super::super::ingest_for_source_checks(&crawl);

    assertions::assert_modular_script_present(&inputs, ".githooks/pre-commit.d/10-typescript.sh");
}

#[cfg(unix)]
#[test]
fn source_ingestion_preserves_app_root_from_non_canonical_path() {
    use std::os::unix::fs::symlink;

    let root = temp_root("non-canonical");
    git_init(&root);
    write(
        &root,
        ".githooks/pre-commit",
        "#!/usr/bin/env bash\ng3ts validate --path apps/landing\npnpm --filter landing run validate\n",
    );
    write(&root, "apps/landing/package.json", "{}\n");

    let alias = root.with_file_name(format!(
        "{}-alias",
        root.file_name()
            .expect("fixture root should have a directory name")
            .to_string_lossy()
    ));
    symlink(&root, &alias).expect("create symlinked fixture repository root");
    let crawl = g3_workspace_crawl::crawl_any_root(&alias.join("apps/landing"))
        .expect("crawl non-canonical TypeScript app root");
    let inputs = super::super::ingest_for_source_checks(&crawl);

    assertions::assert_pre_commit_app_root(&inputs, "apps/landing");
}
