use guardrail3::app::rs::validate::release_checks::check;
use guardrail3::domain::report::Severity;

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn pub12_emits_inventory() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_pub12");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join("src"));
    let _ = std::fs::write(
        tmp.join("Cargo.toml"),
        "[package]\nname = \"x\"\nversion = \"0.1.0\"\ndescription = \"d\"\nlicense = \"MIT\"\nrepository = \"https://x\"",
    );
    let _ = std::fs::write(tmp.join("src/lib.rs"), "");

    let project = guardrail3::app::discover::ProjectInfo {
        has_rust: true,
        has_typescript: false,
        cargo_workspace_root: Some(tmp.clone()),
        workspace_members: vec!["x".to_owned()],
        workspace_member_dirs: vec![".".to_owned()],
        package_json_path: None,
    };
    let tc = guardrail3::adapters::outbound::tool_runner::RealToolChecker;
    let results = check(&fs, &tc, &tmp, &project, false);
    assert!(
        results
            .iter()
            .any(|c| c.id == "R-PUB-12" && c.severity == Severity::Info),
        "Should emit inventory: {results:?}"
    );

    let _ = std::fs::remove_dir_all(&tmp);
}
