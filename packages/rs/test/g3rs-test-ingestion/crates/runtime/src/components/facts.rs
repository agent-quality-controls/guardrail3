use g3rs_test_types::{G3RsTestComponentFileTreeFacts, G3RsTestComponentSourceFacts};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use crate::components::OwnedTestComponent;
use crate::components::support::{
    AssertionsLayout, collect_external_harnesses, collect_sidecars, manifest_dev_dependencies,
    manifest_normal_dependencies, rust_crate_name,
};
use crate::roots::OwnedTestRoot;

pub(crate) fn build_owned_component(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
    layout: AssertionsLayout,
) -> OwnedTestComponent {
    OwnedTestComponent {
        rel_dir: root.root_rel_dir.clone(),
        runtime_rel_dir: root.runtime_rel_dir.clone(),
        runtime_cargo_rel_path: root.cargo_rel_path.clone(),
        runtime_package_name: root
            .cargo
            .package
            .as_ref()
            .and_then(|package| package.name.as_deref())
            .map(rust_crate_name),
        runtime_normal_dependencies: manifest_normal_dependencies(&root.cargo),
        runtime_dev_dependencies: manifest_dev_dependencies(&root.cargo),
        assertions_rel_dir: layout.assertions_rel_dir.clone(),
        assertions_cargo_rel_path: layout.assertions_cargo_rel_path.clone(),
        assertions_exists: layout.assertions_manifest.is_some(),
        nested_assertions_cargo_rel_path: layout.nested_assertions_cargo_rel_path.clone(),
        assertions_package_name: layout
            .assertions_manifest
            .as_ref()
            .and_then(|manifest| manifest.package.as_ref())
            .and_then(|package| package.name.as_deref())
            .map(rust_crate_name),
        assertions_dependencies: layout
            .assertions_manifest
            .as_ref()
            .map(manifest_normal_dependencies)
            .unwrap_or_default(),
        sidecars: collect_sidecars(crawl, &root.runtime_rel_dir, &layout.assertions_rel_dir),
        external_harnesses: collect_external_harnesses(crawl, &root.runtime_rel_dir),
    }
}

pub(crate) fn public_component_facts(
    components: &[OwnedTestComponent],
) -> Vec<G3RsTestComponentSourceFacts> {
    components
        .iter()
        .map(|component| G3RsTestComponentSourceFacts {
            rel_dir: component.rel_dir.clone(),
            runtime_rel_dir: component.runtime_rel_dir.clone(),
            runtime_package_name: component.runtime_package_name.clone(),
            assertions_rel_dir: component.assertions_rel_dir.clone(),
            assertions_exists: component.assertions_exists,
            assertions_package_name: component.assertions_package_name.clone(),
        })
        .collect()
}

pub(crate) fn public_file_tree_component_facts(
    components: &[OwnedTestComponent],
) -> Vec<G3RsTestComponentFileTreeFacts> {
    components
        .iter()
        .map(|component| G3RsTestComponentFileTreeFacts {
            rel_dir: component.rel_dir.clone(),
            runtime_rel_dir: component.runtime_rel_dir.clone(),
            runtime_cargo_rel_path: component.runtime_cargo_rel_path.clone(),
            runtime_package_name: component.runtime_package_name.clone(),
            runtime_normal_dependencies: component.runtime_normal_dependencies.clone(),
            runtime_dev_dependencies: component.runtime_dev_dependencies.clone(),
            assertions_rel_dir: component.assertions_rel_dir.clone(),
            assertions_cargo_rel_path: component.assertions_cargo_rel_path.clone(),
            assertions_exists: component.assertions_exists,
            nested_assertions_cargo_rel_path: component.nested_assertions_cargo_rel_path.clone(),
            assertions_package_name: component.assertions_package_name.clone(),
            assertions_dependencies: component.assertions_dependencies.clone(),
            sidecars: component.sidecars.clone(),
            external_harnesses: component.external_harnesses.clone(),
        })
        .collect()
}
