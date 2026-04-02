use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::LayerName;
use crate::inputs::PackageLibarchInput;

const ID: &str = "RS-LIBARCH-11";

pub fn check(input: &PackageLibarchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.layered_rules_active() {
        return;
    }
    let Some(api) = package.layer_member(LayerName::Api) else {
        return;
    };

    if let Some(error) = &package.facade_source_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "root facade source must prove it exports api".to_owned(),
            format!(
                "Package `{}` cannot verify root-facade exports. {error}",
                package.package_rel_dir
            ),
            package.lib_rel_path.clone(),
            None,
            false,
        ));
        return;
    }

    let exports_api = package
        .facade_exports
        .iter()
        .any(|export| export.crate_name == api.lib_crate_name);
    let exports_core = package.layer_member(LayerName::Core).is_some_and(|core| {
        package
            .facade_exports
            .iter()
            .any(|export| export.crate_name == core.lib_crate_name)
    });

    if !exports_api || exports_core {
        let mut parts = Vec::new();
        if !exports_api {
            parts.push(format!(
                "missing public re-export from api crate `{}`",
                api.lib_crate_name
            ));
        }
        if exports_core {
            parts.push("root facade re-exports directly from core".to_owned());
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "root facade must export from api".to_owned(),
            format!(
                "Package `{}` violates root facade export policy: {}.",
                package.package_rel_dir,
                parts.join("; ")
            ),
            package.lib_rel_path.clone(),
            None,
            false,
        ));
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "root facade exports api".to_owned(),
            format!(
                "Package `{}` exports its public surface from the api crate.",
                package.package_rel_dir
            ),
            package.lib_rel_path.clone(),
            None,
            false,
        )
        .as_inventory(),
    );
}



// reason: test-only sidecar module wiring
