use super::super::super::test_support::{
    INNER_HEX, copy_fixture, errors_by_id, remove_dir, run_family, write_file,
};

#[test]
fn missing_dir_plus_unexpected_dir_hits_both_branches_per_owned_root() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        write_file(tmp.path(), &format!("{dir}/utils/.gitkeep"), "");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(errors.len(), 8, "{errors:#?}");
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("domain/"))
            .count(),
        4,
        "{errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("utils"))
            .count(),
        4,
        "{errors:#?}"
    );
}

#[test]
fn missing_dir_plus_loose_file_hits_both_branches_per_owned_root() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ] {
        remove_dir(tmp.path(), &format!("{dir}/ports"));
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(errors.len(), 8, "{errors:#?}");
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("ports/"))
            .count(),
        4,
        "{errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("loose files"))
            .count(),
        4,
        "{errors:#?}"
    );
}

#[test]
fn renamed_required_dir_yields_missing_and_unexpected_per_owned_root() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        write_file(tmp.path(), &format!("{dir}/domains/.gitkeep"), "");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(errors.len(), 8, "{errors:#?}");
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("missing") && error.title.contains("domain/"))
            .count(),
        4,
        "{errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("unexpected") && error.title.contains("domains"))
            .count(),
        4,
        "{errors:#?}"
    );
}

#[test]
fn gitkeep_alongside_bad_files_ignores_gitkeep_but_still_hits_loose_files() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ] {
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
        write_file(tmp.path(), &format!("{dir}/README.md"), "# stray\n");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(errors.len(), 4, "{errors:#?}");
    for error in &errors {
        assert!(error.title.contains("loose files"), "{error:#?}");
        assert!(error.message.contains("mod.rs"), "{error:#?}");
        assert!(error.message.contains("README.md"), "{error:#?}");
        let listed_files = error
            .message
            .split("Only ")
            .next()
            .unwrap_or(&error.message);
        assert!(!listed_files.contains(".gitkeep"), "{error:#?}");
    }
}

#[test]
fn deep_unexpected_dir_tree_blames_only_the_top_level_unexpected_dir() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ] {
        write_file(
            tmp.path(),
            &format!("{dir}/utils/helpers/deep/lib.rs"),
            "// buried",
        );
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(errors.len(), 4, "{errors:#?}");
    for error in &errors {
        assert!(error.title.contains("utils"), "{error:#?}");
        assert!(!error.title.contains("helpers"), "{error:#?}");
        assert!(!error.title.contains("deep"), "{error:#?}");
    }
}

#[test]
fn maximally_complex_single_root_accumulates_missing_unexpected_and_loose_without_cross_root_noise()
{
    let tmp = copy_fixture();
    let devctl = "apps/devctl/crates";
    remove_dir(tmp.path(), &format!("{devctl}/app"));
    std::os::unix::fs::symlink("/dev/null", tmp.path().join(format!("{devctl}/app")))
        .expect("symlink");
    remove_dir(tmp.path(), &format!("{devctl}/domain"));
    write_file(tmp.path(), &format!("{devctl}/utils/.gitkeep"), "");
    write_file(tmp.path(), &format!("{devctl}/mod.rs"), "// stray");
    write_file(tmp.path(), &format!("{devctl}/.gitkeep"), "");

    let results = run_family(tmp.path());
    let rule_02 = errors_by_id(&results, "RS-HEXARCH-02");
    let devctl_rule_02: Vec<_> = rule_02
        .iter()
        .filter(|error| {
            error
                .file
                .as_deref()
                .is_some_and(|file| file == devctl || file == "apps/devctl/crates/utils")
        })
        .collect();

    assert_eq!(rule_02.len(), 4, "{rule_02:#?}");
    assert_eq!(devctl_rule_02.len(), 4, "{devctl_rule_02:#?}");
    assert!(
        devctl_rule_02
            .iter()
            .any(|error| error.title.contains("missing") && error.title.contains("app/")),
        "{devctl_rule_02:#?}"
    );
    assert!(
        devctl_rule_02
            .iter()
            .any(|error| error.title.contains("missing") && error.title.contains("domain/")),
        "{devctl_rule_02:#?}"
    );
    assert!(
        devctl_rule_02
            .iter()
            .any(|error| error.title.contains("unexpected") && error.title.contains("utils")),
        "{devctl_rule_02:#?}"
    );
    let loose = devctl_rule_02
        .iter()
        .find(|error| error.title.contains("loose files"))
        .expect("loose files result");
    assert!(loose.message.contains("app"), "{loose:#?}");
    assert!(loose.message.contains("mod.rs"), "{loose:#?}");
    let listed_files = loose
        .message
        .split("Only ")
        .next()
        .unwrap_or(&loose.message);
    assert!(!listed_files.contains(".gitkeep"), "{loose:#?}");
}

#[test]
fn all_four_required_dirs_missing_with_gitkeep_still_emit_only_missing_dir_results() {
    let tmp = copy_fixture();
    for dir in ["apps/devctl/crates", "apps/worker/crates"] {
        for name in ["adapters", "app", "domain", "ports"] {
            remove_dir(tmp.path(), &format!("{dir}/{name}"));
        }
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
    }

    for name in ["app", "domain", "ports"] {
        remove_dir(tmp.path(), &format!("apps/backend/crates/{name}"));
    }
    write_file(tmp.path(), "apps/backend/crates/.gitkeep", "");

    for name in ["adapters", "app", "domain", "ports"] {
        remove_dir(tmp.path(), &format!("{INNER_HEX}/{name}"));
    }
    write_file(tmp.path(), &format!("{INNER_HEX}/.gitkeep"), "");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(errors.len(), 15, "{errors:#?}");
    assert!(
        errors.iter().all(|error| error.title.contains("missing")),
        "{errors:#?}"
    );
    assert!(
        errors
            .iter()
            .all(|error| !error.title.contains("loose files")),
        "{errors:#?}"
    );
}

#[test]
fn nested_root_compound_attack_stays_nested_and_preserves_exact_category_split() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{INNER_HEX}/domain"));
    write_file(tmp.path(), &format!("{INNER_HEX}/utils/.gitkeep"), "");
    write_file(
        tmp.path(),
        &format!("{INNER_HEX}/Cargo.toml"),
        "[package]\nname = \"bad\"\n",
    );

    let results = run_family(tmp.path());
    let nested_rule_02: Vec<_> = errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| {
            error
                .file
                .as_deref()
                .is_some_and(|file| file == INNER_HEX || file == format!("{INNER_HEX}/utils"))
        })
        .collect();

    assert_eq!(nested_rule_02.len(), 3, "{nested_rule_02:#?}");
    assert!(
        nested_rule_02
            .iter()
            .any(|error| error.title.contains("adapters/inbound/mcp/crates/domain/")),
        "{nested_rule_02:#?}"
    );
    assert!(
        nested_rule_02
            .iter()
            .any(|error| error.title.contains("adapters/inbound/mcp/crates/utils/")),
        "{nested_rule_02:#?}"
    );
    let loose = nested_rule_02
        .iter()
        .find(|error| error.title.contains("loose files"))
        .expect("nested loose files result");
    assert!(loose.message.contains("Cargo.toml"), "{loose:#?}");
    assert!(
        loose.message.contains("adapters/inbound/mcp/crates"),
        "{loose:#?}"
    );
}
