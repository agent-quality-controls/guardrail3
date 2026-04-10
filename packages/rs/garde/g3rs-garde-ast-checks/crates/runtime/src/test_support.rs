use std::path::Path;

use g3rs_garde_ast_checks_types::{G3RsAstFile, G3RsGardeAstChecksInput};
use guardrail3_check_types::G3CheckResult;
use tempfile::TempDir;

pub(crate) struct Fixture {
    _tempdir: TempDir,
    input: G3RsGardeAstChecksInput,
}

impl Fixture {
    pub(crate) fn run(&self) -> Vec<G3CheckResult> {
        crate::run::check(&self.input)
    }

    #[cfg(unix)]
    pub(crate) fn make_source_unreadable(&self, rel_path: &str) {
        use std::os::unix::fs::PermissionsExt;

        let path = self
            .input
            .source_files
            .iter()
            .find(|file| file.rel_path == rel_path)
            .expect("fixture source file should exist")
            .abs_path
            .clone();
        let mut permissions = std::fs::metadata(&path)
            .expect("fixture source metadata should exist")
            .permissions();
        permissions.set_mode(0o000);
        std::fs::set_permissions(&path, permissions)
            .expect("should make fixture source unreadable");
    }

    #[cfg(unix)]
    pub(crate) fn make_guardrail_unreadable(&self) {
        use std::os::unix::fs::PermissionsExt;

        let path = self.input.guardrail_toml.abs_path.clone();
        let mut permissions = std::fs::metadata(&path)
            .expect("fixture guardrail metadata should exist")
            .permissions();
        permissions.set_mode(0o000);
        std::fs::set_permissions(&path, permissions)
            .expect("should make fixture guardrail unreadable");
    }
}

pub(crate) fn fixture(source_files: &[(&str, &str)], guardrail_toml: &str) -> Fixture {
    let tempdir = tempfile::tempdir().expect("failed to create garde AST fixture tempdir");
    let guardrail_rel_path = "guardrail3.toml";
    let guardrail_abs_path = tempdir.path().join(guardrail_rel_path);
    std::fs::write(&guardrail_abs_path, guardrail_toml)
        .expect("failed to write fixture guardrail3.toml");

    let mut ast_files = Vec::new();
    for (rel_path, content) in source_files {
        let abs_path = tempdir.path().join(rel_path);
        write_file(&abs_path, content);
        ast_files.push(G3RsAstFile {
            rel_path: (*rel_path).to_owned(),
            abs_path,
        });
    }

    ast_files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));

    Fixture {
        _tempdir: tempdir,
        input: G3RsGardeAstChecksInput {
            garde_dependency_present: true,
            source_files: ast_files,
            guardrail_toml: G3RsAstFile {
                rel_path: guardrail_rel_path.to_owned(),
                abs_path: guardrail_abs_path,
            },
        },
    }
}

pub(crate) fn default_guardrail_toml() -> &'static str {
    "[profile]\nname = \"service\"\n"
}

fn write_file(path: &Path, content: &str) {
    std::fs::create_dir_all(path.parent().expect("fixture file must have parent directory"))
        .expect("failed to create fixture directory");
    std::fs::write(path, content).expect("failed to write fixture file");
}
