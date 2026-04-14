use std::path::Path;

use g3rs_garde_source_checks_types::{
    G3RsGardeApplicability, G3RsGardeRustPolicyInput, G3RsGardeSourceChecksInput,
    G3RsGardeWaiver, G3RsSourceFile,
};
use guardrail3_check_types::G3CheckResult;
use tempfile::TempDir;

pub(crate) struct Fixture {
    _tempdir: TempDir,
    input: G3RsGardeSourceChecksInput,
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
}

pub(crate) fn fixture(source_files: &[(&str, &str)], rust_policy_toml: &str) -> Fixture {
    let tempdir = tempfile::tempdir().expect("failed to create garde source fixture tempdir");
    let parsed = guardrail3_rs_toml_parser::parse(rust_policy_toml)
        .expect("fixture Rust policy TOML should parse");

    let mut ast_files = Vec::new();
    for (rel_path, content) in source_files {
        let abs_path = tempdir.path().join(rel_path);
        write_file(&abs_path, content);
        ast_files.push(G3RsSourceFile {
            rel_path: (*rel_path).to_owned(),
            abs_path,
        });
    }

    ast_files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));

    Fixture {
        _tempdir: tempdir,
        input: G3RsGardeSourceChecksInput {
            applicability: G3RsGardeApplicability::Active,
            garde_dependency_present: true,
            source_files: ast_files,
            rust_policy: G3RsGardeRustPolicyInput::Parsed {
                rel_path: "guardrail3-rs.toml".to_owned(),
                garde_enabled: parsed
                    .checks
                    .as_ref()
                    .and_then(|checks| checks.garde)
                    .unwrap_or(false),
                waivers: parsed
                    .waivers
                    .into_iter()
                    .map(|waiver| G3RsGardeWaiver {
                        rule: waiver.rule,
                        file: waiver.file,
                        selector: waiver.selector,
                        reason: waiver.reason,
                    })
                    .collect(),
            },
        },
    }
}

pub(crate) fn invalid_policy_fixture(source_files: &[(&str, &str)], message: &str) -> Fixture {
    let tempdir = tempfile::tempdir().expect("failed to create garde source fixture tempdir");

    let mut ast_files = Vec::new();
    for (rel_path, content) in source_files {
        let abs_path = tempdir.path().join(rel_path);
        write_file(&abs_path, content);
        ast_files.push(G3RsSourceFile {
            rel_path: (*rel_path).to_owned(),
            abs_path,
        });
    }

    ast_files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));

    Fixture {
        _tempdir: tempdir,
        input: G3RsGardeSourceChecksInput {
            applicability: G3RsGardeApplicability::Active,
            garde_dependency_present: true,
            source_files: ast_files,
            rust_policy: G3RsGardeRustPolicyInput::Invalid {
                rel_path: "guardrail3-rs.toml".to_owned(),
                message: message.to_owned(),
            },
        },
    }
}

pub(crate) fn default_guardrail_toml() -> &'static str {
    "profile = \"service\"\n"
}

fn write_file(path: &Path, content: &str) {
    std::fs::create_dir_all(path.parent().expect("fixture file must have parent directory"))
        .expect("failed to create fixture directory");
    std::fs::write(path, content).expect("failed to write fixture file");
}
