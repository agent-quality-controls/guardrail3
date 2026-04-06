use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{crate_name, error, info, is_publishable};

/// Check ID for license presence.
const ID: &str = "RS-RELEASE-CONFIG-02";

/// Verify that a publishable crate has either `license` or `license-file`.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_publishable(&input.cargo) {
        return;
    }

    let name = crate_name(&input.cargo, &input.cargo_rel_path);
    let file = &input.cargo_rel_path;

    let pkg = input.cargo.package.as_ref();
    let has_license = pkg.and_then(|p| p.license.as_ref()).is_some();
    let has_license_file = pkg.and_then(|p| p.license_file.as_ref()).is_some();

    if has_license || has_license_file {
        results.push(info(ID, format!("{name}: license present"), String::new(), file));
    } else {
        results.push(error(
            ID,
            format!("{name}: missing license"),
            "Publishable crates must have a license or license-file field in [package].".to_owned(),
            file,
        ));
    }
}
