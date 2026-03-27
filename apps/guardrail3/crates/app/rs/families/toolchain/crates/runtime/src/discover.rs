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

    let cargo_toml_rel = tree
        .file_exists("Cargo.toml")
        .then(|| "Cargo.toml".to_owned());
    let (cargo_rust_version, cargo_rust_version_invalid, cargo_parse_error) = match cargo_toml_rel
        .as_deref()
        .and_then(|rel| tree.file_content(rel))
    {
        Some(content) => match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => {
                let rust_version = extract_rust_version(&parsed);
                (rust_version.value, rust_version.invalid, None)
            }
            Err(err) => (None, false, Some(err.to_string())),
        },
        None => (None, false, None),
    };

    ToolchainFacts {
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_toml_rel,
        cargo_rust_version,
        cargo_rust_version_invalid,
        cargo_parse_error,
    }
}

struct RustVersionField {
    value: Option<String>,
    invalid: bool,
}

fn extract_rust_version(parsed: &toml::Value) -> RustVersionField {
    let workspace_rust_version = parsed
        .get("workspace")
        .and_then(|value| value.get("package"))
        .and_then(|value| value.get("rust-version"));

    if let Some(value) = workspace_rust_version {
        return RustVersionField {
            value: value.as_str().map(str::to_owned),
            invalid: !value.is_str(),
        };
    }

    let package_rust_version = parsed
        .get("package")
        .and_then(|value| value.get("rust-version"));

    if let Some(value) = package_rust_version {
        return RustVersionField {
            value: value.as_str().map(str::to_owned),
            invalid: !value.is_str(),
        };
    }

    RustVersionField {
        value: None,
        invalid: false,
    }
}
