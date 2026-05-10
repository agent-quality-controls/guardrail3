use cargo_toml_parser::parse;
use g3rs_code_types::{G3RsCodeConfigFile, G3RsCodeConfigFileKind};
use guardrail3_check_types::G3CheckResult;

pub(super) fn run_check(files: &[G3RsCodeConfigFile]) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for file in files {
        super::super::check(file, &mut results);
    }
    results
}

pub(super) fn cargo_file(rel_path: &str, content: &str) -> G3RsCodeConfigFile {
    G3RsCodeConfigFile {
        rel_path: rel_path.to_owned(),
        kind: G3RsCodeConfigFileKind::CargoToml {
            cargo: parse(content).expect("test cargo fixture should parse"),
        },
    }
}
