use crate::domain::project_tree::ProjectTree;

use super::discover::{cargo_toml_rels, is_test_path, rust_file_rels};

#[derive(Debug, Clone)]
pub struct RustCodeFileFacts {
    pub rel_path: String,
    pub is_test: bool,
}

#[derive(Debug, Clone)]
pub struct UnsafeCodeLintFacts {
    pub cargo_rel_path: String,
    pub lint_level: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CodeFacts {
    pub files: Vec<RustCodeFileFacts>,
    pub unsafe_code_lints: Vec<UnsafeCodeLintFacts>,
}

pub fn collect(tree: &ProjectTree) -> CodeFacts {
    let files = rust_file_rels(tree)
        .into_iter()
        .map(|rel_path| RustCodeFileFacts {
            is_test: is_test_path(&rel_path),
            rel_path,
        })
        .collect();

    let unsafe_code_lints = cargo_toml_rels(tree)
        .into_iter()
        .filter_map(|cargo_rel_path| {
            let parsed = tree
                .file_content(&cargo_rel_path)
                .and_then(|content| toml::from_str::<toml::Value>(content).ok())?;
            if parsed.get("workspace").is_none() {
                return None;
            }
            let lint_level = parsed
                .get("workspace")
                .and_then(|workspace| workspace.get("lints"))
                .and_then(|lints| lints.get("rust"))
                .and_then(|rust| rust.get("unsafe_code"))
                .and_then(toml::Value::as_str)
                .map(str::to_owned);
            Some(UnsafeCodeLintFacts {
                cargo_rel_path,
                lint_level,
            })
        })
        .collect();

    CodeFacts {
        files,
        unsafe_code_lints,
    }
}
