use guardrail3_domain_project_tree::ProjectTree;

use super::facts::ToolchainFacts;

pub fn collect(tree: &ProjectTree) -> ToolchainFacts {
    let toolchain_toml_rel = tree
        .file_exists("rust-toolchain.toml")
        .then(|| "rust-toolchain.toml".to_owned());
    let legacy_toolchain_rel = tree
        .file_exists("rust-toolchain")
        .then(|| "rust-toolchain".to_owned());

    let (parsed, parse_error) = match toolchain_toml_rel
        .as_deref()
        .and_then(|rel| tree.file_content(rel))
    {
        Some(content) => match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => (Some(parsed), None),
            Err(err) => (None, Some(err.to_string())),
        },
        None => (None, None),
    };

    let cargo_rust_version = tree.file_content("Cargo.toml").and_then(|content| {
        toml::from_str::<toml::Value>(content)
            .ok()
            .and_then(|parsed| {
                parsed
                    .get("workspace")
                    .and_then(|value| value.get("package"))
                    .and_then(|value| value.get("rust-version"))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
                    .or_else(|| {
                        parsed
                            .get("package")
                            .and_then(|value| value.get("rust-version"))
                            .and_then(toml::Value::as_str)
                            .map(str::to_owned)
                    })
            })
    });

    ToolchainFacts {
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_rust_version,
    }
}
