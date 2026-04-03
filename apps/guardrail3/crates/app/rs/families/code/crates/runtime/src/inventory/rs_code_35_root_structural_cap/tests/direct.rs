use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::inventory::rs_code_35_root_structural_cap::{
    RuleFinding, assert_findings,
};
use test_support::{create_dir_all, create_temp_dir, write_file};

#[test]
fn errors_when_root_exceeds_structural_caps() {
    let tmp = create_temp_dir("rs-code-35-too-large");
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(tmp.path(), "src/lib.rs", "");
    for index in 0..13 {
        write_file(tmp.path(), &format!("src/dir{index}/mod.rs"), "");
    }
    for index in 0..21 {
        write_file(tmp.path(), &format!("src/file{index}.rs"), "");
    }
    create_deep_chain(tmp.path(), 7);

    let results = run_family(tmp.path());

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "crate source tree exceeds structural caps",
            "Rust root `` exceeds structural caps: module depth 9 > 6, sibling source directories 14 > 12, sibling .rs files 22 > 20.",
            Some("Cargo.toml"),
            None,
            false,
        )],
    );
}

fn create_deep_chain(root: &std::path::Path, depth: usize) {
    let mut path = String::from("src");
    for index in 0..depth {
        path.push_str(&format!("/n{index}"));
        create_dir_all(&root.join(&path));
    }
    write_file(root, &format!("{path}/leaf.rs"), "");
}
