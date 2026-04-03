use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_public_struct_field_bags;

const ID: &str = "RS-CODE-31";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_public_struct_field_bags(input.ast) {
        let severity = if info.public_field_count >= 5 {
            Severity::Error
        } else {
            Severity::Warn
        };
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            severity,
            "public struct exposes named public fields".to_owned(),
            format!(
                "Public struct `{}` exposes {} named `pub` fields (warn below 5, error at 5+). Prefer private fields and explicit accessors or constructors.",
                info.struct_name, info.public_field_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
}

