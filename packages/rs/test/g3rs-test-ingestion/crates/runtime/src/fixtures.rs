use g3rs_test_types::{
    G3RsTestComponentFileTreeFacts, G3RsTestComponentSourceFacts, G3RsTestFileKind,
    G3RsTestFileTreeChecksInput, G3RsTestOwnedSidecarFacts, G3RsTestSourceChecksInput,
    G3RsTestSourceFile,
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

pub mod file_tree {
    use std::collections::BTreeSet;

    use super::*;

    pub fn file(
        rel_path: &str,
        kind: G3RsTestFileKind,
        component_rel_dir: Option<&str>,
        owner_module_name: Option<&str>,
        assertions_package_name: Option<&str>,
        content: &str,
    ) -> G3RsTestSourceFile {
        G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind,
            owner_module_name: owner_module_name.map(str::to_owned),
            component_rel_dir: component_rel_dir.map(str::to_owned),
            assertions_package_name: assertions_package_name.map(str::to_owned),
            content: content.to_owned(),
        }
    }

    pub fn component(
        rel_dir: &str,
        runtime_rel_dir: &str,
        runtime_package_name: Option<&str>,
        assertions_exists: bool,
        assertions_package_name: Option<&str>,
    ) -> G3RsTestComponentFileTreeFacts {
        G3RsTestComponentFileTreeFacts {
            rel_dir: rel_dir.to_owned(),
            runtime_rel_dir: runtime_rel_dir.to_owned(),
            runtime_cargo_rel_path: if runtime_rel_dir.is_empty() {
                "Cargo.toml".to_owned()
            } else {
                format!("{runtime_rel_dir}/Cargo.toml")
            },
            runtime_package_name: runtime_package_name.map(str::to_owned),
            runtime_normal_dependencies: BTreeSet::new(),
            runtime_dev_dependencies: BTreeSet::new(),
            assertions_rel_dir: if runtime_rel_dir.is_empty() {
                "assertions".to_owned()
            } else {
                format!("{}/assertions", parent_dir(runtime_rel_dir))
            },
            assertions_cargo_rel_path: if runtime_rel_dir.is_empty() {
                "assertions/Cargo.toml".to_owned()
            } else {
                format!("{}/assertions/Cargo.toml", parent_dir(runtime_rel_dir))
            },
            assertions_exists,
            nested_assertions_cargo_rel_path: None,
            assertions_package_name: assertions_package_name.map(str::to_owned),
            assertions_dependencies: BTreeSet::new(),
            sidecars: Vec::new(),
            external_harnesses: Vec::new(),
        }
    }

    pub fn with_sidecar(
        mut component: G3RsTestComponentFileTreeFacts,
        mod_rel_path: &str,
        assertions_module_rel_path: &str,
    ) -> G3RsTestComponentFileTreeFacts {
        component.sidecars.push(G3RsTestOwnedSidecarFacts {
            mod_rel_path: mod_rel_path.to_owned(),
            assertions_module_rel_path: assertions_module_rel_path.to_owned(),
        });
        component
    }

    pub fn with_external_harness(
        mut component: G3RsTestComponentFileTreeFacts,
        rel_path: &str,
    ) -> G3RsTestComponentFileTreeFacts {
        component.external_harnesses.push(rel_path.to_owned());
        component
    }

    pub fn with_nested_assertions_manifest(
        mut component: G3RsTestComponentFileTreeFacts,
        nested_assertions_cargo_rel_path: &str,
    ) -> G3RsTestComponentFileTreeFacts {
        component.nested_assertions_cargo_rel_path =
            Some(nested_assertions_cargo_rel_path.to_owned());
        component
    }

    pub fn input(
        files: Vec<G3RsTestSourceFile>,
        components: Vec<G3RsTestComponentFileTreeFacts>,
    ) -> G3RsTestFileTreeChecksInput {
        let local_package_names = components
            .iter()
            .filter_map(|component| component.runtime_package_name.clone())
            .chain(
                components
                    .iter()
                    .filter_map(|component| component.assertions_package_name.clone()),
            )
            .collect();
        crate::file_tree_analysis::build_file_tree_checks_input(
            String::new(),
            "Cargo.toml".to_owned(),
            files,
            components,
            local_package_names,
            Vec::new(),
        )
    }

    fn parent_dir(rel_path: &str) -> &str {
        rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
    }
}
