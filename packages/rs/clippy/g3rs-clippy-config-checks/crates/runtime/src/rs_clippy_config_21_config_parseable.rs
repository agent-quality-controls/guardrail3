use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{raw_parse_error, typed_parse_error, typed_clippy};

const ID: &str = "RS-CLIPPY-CONFIG-21";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if typed_clippy(input).is_some() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "clippy.toml parseable".to_owned(),
                format!("`{}` parsed successfully.", input.clippy_rel_path),
                Some(input.clippy_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    if let Some(parse_error) = raw_parse_error(input).or_else(|| typed_parse_error(input)) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "clippy.toml parse error".to_owned(),
            format!("Failed to parse `{}`: {parse_error}", input.clippy_rel_path),
            Some(input.clippy_rel_path.clone()),
            None,
        ));
    }
}
