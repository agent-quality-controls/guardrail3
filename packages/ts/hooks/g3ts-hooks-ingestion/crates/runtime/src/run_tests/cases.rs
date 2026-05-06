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
fn source_ingestion_includes_g3ts_verifier_script() {
    let root = temp_root("g3ts-verifier");
    write(&root, "Cargo.toml", "[workspace]\nresolver = \"2\"\n");
    write(&root, "package.json", "{}\n");
    write(
        &root,
        ".githooks/pre-commit",
        "#!/usr/bin/env bash\nscripts/g3ts/verify --mode pre-commit --scope .\n",
    );
    write(
        &root,
        "scripts/g3ts/verify",
        "g3ts validate --path \"$SCOPE\"\n",
    );

    let crawl = g3_workspace_crawl::crawl(&root).expect("crawl fixture workspace");
    let inputs = super::super::ingest_for_source_checks(&crawl);

    assertions::assert_verifier_script_present(&inputs);
}

#[test]
fn source_ingestion_does_not_read_neighbor_verifiers() {
    let root = temp_root("neighbor-verifiers");
    write(&root, "Cargo.toml", "[workspace]\nresolver = \"2\"\n");
    write(&root, "package.json", "{}\n");
    write(
        &root,
        ".githooks/pre-commit",
        "#!/usr/bin/env bash\nscripts/g3ts/verify --mode pre-commit --scope .\n",
    );
    write(&root, "scripts/g3ts/verify", "g3ts validate --path \"$SCOPE\"\n");
    write(&root, "scripts/g3rs/verify", "g3rs validate --path \"$SCOPE\"\n");
    write(&root, "scripts/guardrails/verify", "echo shared\n");

    let crawl = g3_workspace_crawl::crawl(&root).expect("crawl fixture workspace");
    let inputs = super::super::ingest_for_source_checks(&crawl);

    assertions::assert_verifier_script_present(&inputs);
    assert!(
        inputs
            .iter()
            .all(|input| input.rel_path() != "scripts/g3rs/verify"
                && !input.rel_path().starts_with("scripts/guardrails/")),
        "G3TS ingestion must not read neighboring verifier scripts: {inputs:#?}"
    );
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
