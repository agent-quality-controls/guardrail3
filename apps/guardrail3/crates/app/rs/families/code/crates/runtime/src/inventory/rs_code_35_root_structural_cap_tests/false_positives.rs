use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_35_root_structural_cap::assert_no_hits;
use test_support::{create_temp_dir, write_file};

#[test]
fn stays_quiet_at_exact_thresholds() {
    let tmp = create_temp_dir("rs-code-35-threshold");
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(tmp.path(), "src/lib.rs", "");
    for index in 0..11 {
        write_file(tmp.path(), &format!("src/dir{index}/mod.rs"), "");
    }
    for index in 0..19 {
        write_file(tmp.path(), &format!("src/file{index}.rs"), "");
    }
    write_file(tmp.path(), "src/a/b/c/d/e/mod.rs", "");

    assert_no_hits(&run_family(tmp.path()));
}
