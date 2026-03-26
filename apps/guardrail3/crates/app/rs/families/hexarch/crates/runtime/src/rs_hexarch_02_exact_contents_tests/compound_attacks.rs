use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;
use super::{copy_fixture, remove_dir, write_file};
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

#[test]
fn missing_dir_plus_unexpected_dir_hits_both_branches_per_owned_root() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        write_file(tmp.path(), &format!("{dir}/utils/.gitkeep"), "");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        8,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        None,
        None,
        None,
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        4,
        None,
        None,
        &["domain/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        4,
        None,
        None,
        &["utils"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn missing_dir_plus_loose_file_hits_both_branches_per_owned_root() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/ports"));
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        8,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        None,
        None,
        None,
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        4,
        None,
        None,
        &["ports/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        4,
        None,
        None,
        &["loose files"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn renamed_required_dir_yields_missing_and_unexpected_per_owned_root() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        write_file(tmp.path(), &format!("{dir}/domains/.gitkeep"), "");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        8,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        None,
        None,
        None,
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        4,
        None,
        None,
        &["missing", "domain/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        4,
        None,
        None,
        &["unexpected", "domains"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn gitkeep_alongside_bad_files_ignores_gitkeep_but_still_hits_loose_files() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        write_file(tmp.path(), &format!("{dir}/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
        write_file(tmp.path(), &format!("{dir}/README.md"), "# stray\n");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        Some(&["loose files"]),
        None,
        None,
    );
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        None,
        None,
        None,
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        4,
        None,
        None,
        &["loose files"],
        &[],
        &["mod.rs", "README.md"],
        &[".gitkeep"],
    );
}

#[test]
fn deep_unexpected_dir_tree_blames_only_the_top_level_unexpected_dir() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        write_file(
            tmp.path(),
            &format!("{dir}/utils/helpers/deep/lib.rs"),
            "// buried",
        );
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        Some(&["utils"]),
        Some(&["helpers", "deep"]),
        None,
    );
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

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [devctl, "apps/devctl/crates/utils"],
        None,
        None,
        None,
        None,
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        1,
        Some(devctl),
        None,
        &["missing", "app/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        1,
        Some(devctl),
        None,
        &["missing", "domain/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        1,
        Some("apps/devctl/crates/utils"),
        None,
        &["unexpected", "utils"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        1,
        Some(devctl),
        None,
        &["loose files"],
        &[],
        &["app", "mod.rs"],
        &[".gitkeep"],
    );
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
        remove_dir(tmp.path(), &format!("{}/{}", inner_hex(), name));
    }
    write_file(tmp.path(), &format!("{}/.gitkeep", inner_hex()), "");

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        15,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        Some(&["missing"]),
        Some(&["loose files"]),
        None,
        None,
    );
}
