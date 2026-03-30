use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

#[cfg(test)]
use super::inputs::AncestorToolchainInput;

const ID: &str = "RS-TOOLCHAIN-05";

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    if input.legacy_toolchain_rel.is_some() {
        return;
    }

    let Some(local_rel) = input.toolchain_toml_rel else {
        return;
    };
    let Some(ancestor) = input.ancestor_toolchain.as_ref() else {
        return;
    };

    if ancestor.is_legacy {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "ancestor legacy toolchain shadows local policy root".to_owned(),
            format!(
                "Ancestor `rust-toolchain` can shadow local toolchain policy at `{local_rel}`. Remove or migrate the ancestor file so routed-root toolchain behavior stays stable."
            ),
            Some(ancestor.rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }

    if let Some(parse_error) = ancestor.parse_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "ancestor toolchain parse error risks shadow drift".to_owned(),
            format!(
                "Ancestor `rust-toolchain.toml` at `{}` is invalid: {parse_error}. Commands run above `{local_rel}` may resolve a different toolchain surface.",
                ancestor.rel_path,
            ),
            Some(ancestor.rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }

    let (Some(local), Some(ancestor_parsed)) = (input.parsed, ancestor.parsed) else {
        return;
    };

    if local != ancestor_parsed {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "ancestor toolchain drifts from local policy root".to_owned(),
            format!(
                "Ancestor `rust-toolchain.toml` differs from local toolchain policy at `{local_rel}`. Running from the ancestor directory can use a different toolchain contract."
            ),
            Some(ancestor.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) struct AncestorFixture {
    rel_path: &'static str,
    is_legacy: bool,
    parsed: Option<&'static toml::Value>,
    parse_error: Option<&'static str>,
}

#[cfg(test)]
impl AncestorFixture {
    pub(crate) fn modern(rel_path: &'static str, source: &str) -> Self {
        let parsed = toml::from_str::<toml::Value>(source)
            .expect("ancestor modern toolchain fixture should parse");
        Self {
            rel_path,
            is_legacy: false,
            parsed: Some(Box::leak(Box::new(parsed))),
            parse_error: None,
        }
    }

    pub(crate) const fn legacy(rel_path: &'static str) -> Self {
        Self {
            rel_path,
            is_legacy: true,
            parsed: None,
            parse_error: None,
        }
    }

    pub(crate) const fn malformed(rel_path: &'static str, parse_error: &'static str) -> Self {
        Self {
            rel_path,
            is_legacy: false,
            parsed: None,
            parse_error: Some(parse_error),
        }
    }

    fn as_input(self) -> AncestorToolchainInput<'static> {
        AncestorToolchainInput {
            rel_path: self.rel_path,
            is_legacy: self.is_legacy,
            parsed: self.parsed,
            parse_error: self.parse_error,
        }
    }
}

#[cfg(test)]
pub(crate) fn test_input_with_ancestor(
    toolchain_toml_rel: &'static str,
    toolchain_source: &str,
    ancestor_toolchain: Option<AncestorFixture>,
) -> ToolchainRootInput<'static> {
    let parsed = toml::from_str::<toml::Value>(toolchain_source)
        .expect("local toolchain fixture should parse");

    ToolchainRootInput {
        rel_dir: "",
        cargo_rel_path: "Cargo.toml",
        cargo_toml_rel: Some("Cargo.toml"),
        toolchain_toml_rel: Some(toolchain_toml_rel),
        legacy_toolchain_rel: None,
        parsed: Some(Box::leak(Box::new(parsed))),
        parse_error: None,
        cargo_rust_version: Some("1.85"),
        cargo_rust_version_invalid: false,
        cargo_parse_error: None,
        ancestor_toolchain: ancestor_toolchain.map(AncestorFixture::as_input),
        descendant_toolchains: Vec::new(),
    }
}

#[cfg(test)]
#[path = "rs_toolchain_05_ancestor_shadow_drift_tests/mod.rs"]
mod rs_toolchain_05_ancestor_shadow_drift_tests;
