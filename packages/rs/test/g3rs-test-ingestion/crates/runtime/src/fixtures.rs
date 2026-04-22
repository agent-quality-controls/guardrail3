use g3rs_test_types::{
    G3RsTestComponentSourceFacts, G3RsTestFileKind, G3RsTestSourceChecksInput, G3RsTestSourceFile,
};

pub fn file(
    rel_path: &str,
    kind: G3RsTestFileKind,
    assertions_package_name: Option<&str>,
    content: &str,
) -> G3RsTestSourceFile {
    G3RsTestSourceFile {
        rel_path: rel_path.to_owned(),
        kind,
        owner_module_name: None,
        component_rel_dir: Some(String::new()),
        assertions_package_name: assertions_package_name.map(str::to_owned),
        content: content.to_owned(),
    }
}

pub fn input(
    files: Vec<G3RsTestSourceFile>,
    assertions_package_name: Option<&str>,
) -> G3RsTestSourceChecksInput {
    let (files, input_failures) = crate::source_analysis::analyze_source_files(files);
    assert!(input_failures.is_empty(), "{input_failures:#?}");
    G3RsTestSourceChecksInput {
        root_rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        files,
        components: vec![G3RsTestComponentSourceFacts {
            rel_dir: String::new(),
            runtime_rel_dir: String::new(),
            runtime_package_name: Some("demo".to_owned()),
            assertions_rel_dir: "assertions".to_owned(),
            assertions_exists: assertions_package_name.is_some(),
            assertions_package_name: assertions_package_name.map(str::to_owned),
        }],
        input_failures,
    }
}
