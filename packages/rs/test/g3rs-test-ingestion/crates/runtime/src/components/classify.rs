use g3rs_test_types::{G3RsTestFileKind, G3RsTestSourceFile};

use crate::components::OwnedTestComponent;
use crate::components::support::{
    file_stem, is_fixture_path, owner_module_name_from_sidecar_path, path_is_under,
};
use crate::roots::{OwnedTestRoot, join_under_root};

pub(crate) fn classify_file_for_source(
    rel_path: &str,
    root: &OwnedTestRoot,
    components: &[OwnedTestComponent],
) -> Option<G3RsTestSourceFile> {
    if is_fixture_path(rel_path) {
        return None;
    }

    for component in components {
        if let Some(file) = classify_component_file(rel_path, component) {
            return Some(file);
        }
    }

    let root_runtime_src = join_under_root(&root.runtime_rel_dir, "src");
    if path_is_under(rel_path, &root_runtime_src) {
        return Some(G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind: G3RsTestFileKind::Other,
            owner_module_name: file_stem(rel_path).map(str::to_owned),
            component_rel_dir: None,
            assertions_package_name: None,
            content: String::new(),
        });
    }

    None
}

pub(crate) fn classify_file_for_file_tree(
    rel_path: &str,
    root: &OwnedTestRoot,
    components: &[OwnedTestComponent],
) -> Option<G3RsTestSourceFile> {
    if is_fixture_path(rel_path) {
        return None;
    }

    for component in components {
        if let Some(file) = classify_component_file(rel_path, component) {
            return Some(file);
        }
    }

    if is_test_support_path(root, rel_path) {
        return Some(G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind: G3RsTestFileKind::TestSupport,
            owner_module_name: file_stem(rel_path).map(str::to_owned),
            component_rel_dir: None,
            assertions_package_name: None,
            content: String::new(),
        });
    }

    let root_relative = root_rel_prefix(rel_path, &root.root_rel_dir)?;
    if let Some(rel_after_src) = rel_after_named_dir(root_relative, "src") {
        return Some(source_file_from_rel_after_src(
            rel_path,
            rel_after_src,
            None,
            None,
        ));
    }

    let kind = if rel_after_named_dir(root_relative, "tests").is_some() {
        G3RsTestFileKind::ExternalHarness
    } else {
        G3RsTestFileKind::Other
    };
    Some(G3RsTestSourceFile {
        rel_path: rel_path.to_owned(),
        kind,
        owner_module_name: file_stem(rel_path).map(str::to_owned),
        component_rel_dir: None,
        assertions_package_name: None,
        content: String::new(),
    })
}

pub(crate) fn root_rel_prefix<'a>(rel_path: &'a str, root_rel_dir: &str) -> Option<&'a str> {
    if root_rel_dir.is_empty() {
        Some(rel_path)
    } else {
        rel_path
            .strip_prefix(root_rel_dir)
            .and_then(|rest| rest.strip_prefix('/'))
    }
}

fn classify_component_file(
    rel_path: &str,
    component: &OwnedTestComponent,
) -> Option<G3RsTestSourceFile> {
    let runtime_src = join_under_root(&component.runtime_rel_dir, "src");
    if path_is_under(rel_path, &runtime_src) {
        let rel_after_src = rel_path
            .strip_prefix(&runtime_src)
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or("");
        return Some(source_file_from_rel_after_src(
            rel_path,
            rel_after_src,
            Some(component.rel_dir.clone()),
            component.assertions_package_name.clone(),
        ));
    }

    let runtime_tests = join_under_root(&component.runtime_rel_dir, "tests");
    if path_is_under(rel_path, &runtime_tests) && rel_path.ends_with(".rs") {
        return Some(G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind: G3RsTestFileKind::ExternalHarness,
            owner_module_name: None,
            component_rel_dir: Some(component.rel_dir.clone()),
            assertions_package_name: component.assertions_package_name.clone(),
            content: String::new(),
        });
    }

    let assertions_src = join_under_root(&component.assertions_rel_dir, "src");
    if path_is_under(rel_path, &assertions_src) {
        return Some(G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind: G3RsTestFileKind::AssertionsModule,
            owner_module_name: file_stem(rel_path).map(str::to_owned),
            component_rel_dir: Some(component.rel_dir.clone()),
            assertions_package_name: component.assertions_package_name.clone(),
            content: String::new(),
        });
    }

    None
}

fn source_file_from_rel_after_src(
    rel_path: &str,
    rel_after_src: &str,
    component_rel_dir: Option<String>,
    assertions_package_name: Option<String>,
) -> G3RsTestSourceFile {
    if rel_after_src.ends_with("_tests/mod.rs") {
        return G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind: G3RsTestFileKind::InternalSidecarMod,
            owner_module_name: rel_after_src
                .rsplit_once('/')
                .and_then(|(parent, _)| parent.rsplit('/').next())
                .and_then(|segment| segment.strip_suffix("_tests"))
                .map(str::to_owned),
            component_rel_dir,
            assertions_package_name,
            content: String::new(),
        };
    }

    if let Some(owner_module_name) = owner_module_name_from_sidecar_path(rel_after_src) {
        return G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind: G3RsTestFileKind::InternalSidecarSupport,
            owner_module_name: Some(owner_module_name),
            component_rel_dir,
            assertions_package_name,
            content: String::new(),
        };
    }

    G3RsTestSourceFile {
        rel_path: rel_path.to_owned(),
        kind: G3RsTestFileKind::Source,
        owner_module_name: file_stem(rel_path).map(str::to_owned),
        component_rel_dir,
        assertions_package_name,
        content: String::new(),
    }
}

fn rel_after_named_dir<'a>(root_relative: &'a str, dir_name: &str) -> Option<&'a str> {
    let prefix = format!("{dir_name}/");
    if let Some(rest) = root_relative.strip_prefix(&prefix) {
        return Some(rest);
    }
    let marker = format!("/{dir_name}/");
    root_relative.rsplit_once(&marker).map(|(_, rest)| rest)
}

fn is_test_support_path(root: &OwnedTestRoot, rel_path: &str) -> bool {
    [
        join_under_root(&root.root_rel_dir, "test_support/src"),
        join_under_root(&root.root_rel_dir, "crates/test_support/src"),
    ]
    .into_iter()
    .any(|test_support_src| path_is_under(rel_path, &test_support_src))
}
