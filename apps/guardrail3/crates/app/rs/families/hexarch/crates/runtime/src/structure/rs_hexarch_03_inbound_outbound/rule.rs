use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::DirectionalContainerHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-03";

pub fn check(input: &DirectionalContainerHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let before = results.len();
    for expected in ["inbound", "outbound"] {
        if input.dirs.iter().any(|dir| dir == expected)
            && !input.symlink_dirs.iter().any(|dir| dir == expected)
        {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!(
                "Service `{}` missing {}/{}/ directory",
                input.app_name, input.label, expected
            ),
            format!(
                "Service `{}` is missing `{}/{}/`. Create it and add a `.gitkeep` if not needed yet.",
                input.app_name, input.label, expected
            ),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }

    for dir in input.dirs {
        if ["inbound", "outbound"].contains(&dir.as_str()) {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!(
                "Service `{}` has unexpected directory {}/{}/",
                input.app_name, input.label, dir
            ),
            format!(
                "Service `{}` has `{}/{}/` which is not part of the hexarch template. Only `{{inbound, outbound}}` directories are allowed in `{}/`.",
                input.app_name, input.label, dir, input.label
            ),
            Some(format!("{}/{}", input.rel_path, dir)),
            None,
            false,
        ));
    }

    if results.len() == before {
        push_success(
            results,
            ID,
            format!(
                "Service `{}` has inbound/outbound split in {}",
                input.app_name, input.label
            ),
            format!(
                "Service `{}` keeps `{}` limited to `inbound/` and `outbound/`.",
                input.app_name, input.rel_path
            ),
            Some(input.rel_path.to_owned()),
        );
    }
}

