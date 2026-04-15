use g3rs_release_config_checks_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

const ID: &str = "RS-RELEASE-CONFIG-00";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if krate.publish_declared {
        return;
    }

    results.push(error(
        ID,
        format!("{}: publish must be explicit", krate.name),
        format!(
            "Crate `{}` does not set `[package].publish`. Add `publish = true` if this crate publishes or `publish = false` if it does not.",
            krate.name
        ),
        &krate.cargo_rel_path,
    ));
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn errors_when_publish_is_missing() {
        let input = crate::test_support::config_input_for_crate(
            r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
"#,
            None,
        );
        let mut results = Vec::new();

        check(&input.crates[0], &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id(), "RS-RELEASE-CONFIG-00");
        assert_eq!(results[0].title(), "demo: publish must be explicit");
    }

    #[test]
    fn stands_down_when_publish_is_false() {
        let input = crate::test_support::config_input_for_crate(
            r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
publish = false
"#,
            None,
        );
        let mut results = Vec::new();

        check(&input.crates[0], &mut results);

        assert!(results.is_empty());
    }

    #[test]
    fn stands_down_when_publish_is_inherited() {
        let input = crate::test_support::config_input_for_crate(
            r#"
[package]
name = "demo"
version.workspace = true
edition = "2024"
publish.workspace = true
"#,
            Some(
                r#"
[workspace.package]
version = "0.1.0"
publish = false
"#,
            ),
        );
        let mut results = Vec::new();

        check(&input.crates[0], &mut results);

        assert!(results.is_empty());
    }
}
