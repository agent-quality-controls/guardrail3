use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_generic_parameter_caps;

const ID: &str = "RS-CODE-34";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_generic_parameter_caps(input.ast) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "too many generic parameters".to_owned(),
            format!(
                "{} `{}` has {} type/const generic parameters (cap 6; lifetimes do not count).",
                info.item_kind, info.item_name, info.type_const_param_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
}

