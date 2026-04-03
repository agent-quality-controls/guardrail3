use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{GardeSkipInfo, find_garde_skips_with_types, same_line_has_comment};

const ID: &str = "RS-CODE-05";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_garde_skips_with_types(input.ast) {
        if info.is_exempt {
            continue;
        }
        let has_comment = same_line_has_comment(input.content, info.line);
        if has_comment {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "garde(skip) without comment".to_owned(),
            format!(
                "`#[garde(skip)]` on non-exempt {} requires documentation. Add a `// reason:` comment explaining why validation is skipped.",
                target_label(&info)
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
}

fn target_label(info: &GardeSkipInfo) -> String {
    if info.is_type_level {
        format!("type `{}`", info.field_name)
    } else {
        format!("field `{}: {}`", info.field_name, info.field_type)
    }
}




// reason: test-only sidecar module wiring
