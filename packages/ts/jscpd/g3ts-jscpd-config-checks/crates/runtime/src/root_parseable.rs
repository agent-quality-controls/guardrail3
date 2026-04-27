use g3ts_jscpd_types::{G3TsJscpdChecksInput, G3TsJscpdRootState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::info;

const ID: &str = "g3ts-jscpd/root-parseable";

pub(crate) fn check(input: &G3TsJscpdChecksInput, results: &mut Vec<G3CheckResult>) {
    match &input.root {
        G3TsJscpdRootState::Missing => {}
        G3TsJscpdRootState::Unreadable { rel_path, reason } => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "root .jscpd.json unreadable".to_owned(),
                format!("Failed to read `{rel_path}` while evaluating jscpd policy: {reason}"),
                Some(rel_path.clone()),
                None,
            ));
        }
        G3TsJscpdRootState::ParseError { rel_path, reason } => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "root .jscpd.json parse error".to_owned(),
                format!("Failed to parse root `.jscpd.json`: {reason}"),
                Some(rel_path.clone()),
                None,
            ));
        }
        G3TsJscpdRootState::Parsed { snapshot } => {
            results.push(info(
                ID,
                "root .jscpd.json parseable",
                format!("`{}` parsed successfully as jscpd JSON.", snapshot.rel_path),
                &snapshot.rel_path,
            ));
        }
    }
}
