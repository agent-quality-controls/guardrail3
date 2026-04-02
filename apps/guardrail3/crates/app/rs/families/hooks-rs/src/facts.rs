use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

#[derive(Debug, Clone)]
pub struct RustHookFacts {
    pub(crate) pre_commit_rel_path: Option<String>,
    pub(crate) pre_commit_content: Option<String>,
}

pub fn collect(tree: &ProjectTree) -> RustHookFacts {
    for rel_path in [".githooks/pre-commit", "hooks/pre-commit"] {
        if let Some(content) = tree.file_content(rel_path) {
            return RustHookFacts {
                pre_commit_rel_path: Some(rel_path.to_owned()),
                pre_commit_content: Some(content.to_owned()),
            };
        }
    }

    RustHookFacts {
        pre_commit_rel_path: None,
        pre_commit_content: None,
    }
}
