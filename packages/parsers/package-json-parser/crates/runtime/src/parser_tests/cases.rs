use package_json_parser_runtime_assertions::parser::{
    assert_bool_field_state, assert_dependency_spec_exact, assert_dependency_spec_kind,
    assert_dependency_spec_range_allows_below_unknown, assert_dependency_spec_range_minimum,
    assert_invalid_document, assert_parsed_document, assert_snapshot_fields,
};

use crate::types::{PackageDependencySection, SemverVersion};

#[test]
fn parses_root_manifest_snapshot() {
    let document = super::super::parse_document(
        r#"
        {
          "private": true,
          "packageManager": "pnpm@10.0.0",
          "engines": {
            "node": ">=24",
            "pnpm": "10"
          },
          "scripts": {
            "lint": "eslint .",
            "typecheck": "tsc --noEmit"
          },
          "pnpm": {
            "overrides": {
              "@eslint/js": "^9.0.0",
              "zod": "^4.0.0"
            },
            "onlyBuiltDependencies": ["esbuild"]
          },
          "dependencies": {
            "react": "^19.0.0"
          },
          "devDependencies": {
            "typescript": "^5.7.0"
          }
        }
        "#,
    )
    .expect("package.json document should parse");

    assert_parsed_document(&document);
    assert_bool_field_state(&document, "private", Some(true));

    assert_snapshot_fields(
        &document,
        Some("pnpm@10.0.0"),
        Some(">=24"),
        Some("10"),
        Some("eslint ."),
        &["@eslint/js", "zod"],
        &["esbuild"],
        &["react"],
        &["typescript"],
    );
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "single parser fixture proves dependency spec normalization across sections and range kinds"
)]
fn parses_dependency_version_specs() {
    let document = super::super::parse_document(
        r#"
        {
          "dependencies": {
            "exact-lib": "1.2.3",
            "prerelease-lib": "1.2.3-beta.1",
            "build-metadata-lib": "1.2.3+build.1",
            "range-lib": "^2.3.4",
            "bare-comparator-range-lib": "1.2.3 || >=2.0.0",
            "out-of-order-or-range-lib": "^19.0.0 || ^18.0.0",
            "unsafe-or-range-lib": "<9.0.0 || >=10.0.0",
            "bounded-range-lib": ">=2.3.4 <3.0.0",
            "upper-bound-only-lib": "<2.0.0",
            "exclusive-lower-bound-lib": ">0.1.1",
            "workspace-lib": "workspace:*",
            "file-lib": "file:../file-lib",
            "unsupported-lib": "npm:actual-lib@1.2.3"
          },
          "devDependencies": {
            "link-lib": "link:../link-lib",
            "catalog-lib": "catalog:"
          },
          "optionalDependencies": {
            "optional-lib": "^1.0.0"
          },
          "peerDependencies": {
            "peer-lib": "^2.0.0"
          }
        }
        "#,
    )
    .expect("package.json document should parse");

    assert_dependency_spec_exact(
        &document,
        "exact-lib",
        PackageDependencySection::Dependencies,
        SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: None,
        },
    );
    assert_dependency_spec_range_minimum(
        &document,
        "range-lib",
        PackageDependencySection::Dependencies,
        Some(SemverVersion {
            major: 2,
            minor: 3,
            patch: 4,
            pre: None,
        }),
    );
    assert_dependency_spec_range_allows_below_unknown(
        &document,
        "range-lib",
        PackageDependencySection::Dependencies,
        false,
    );
    assert_dependency_spec_range_minimum(
        &document,
        "bounded-range-lib",
        PackageDependencySection::Dependencies,
        Some(SemverVersion {
            major: 2,
            minor: 3,
            patch: 4,
            pre: None,
        }),
    );
    assert_dependency_spec_range_minimum(
        &document,
        "bare-comparator-range-lib",
        PackageDependencySection::Dependencies,
        Some(SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: None,
        }),
    );
    assert_dependency_spec_range_minimum(
        &document,
        "upper-bound-only-lib",
        PackageDependencySection::Dependencies,
        None,
    );
    assert_dependency_spec_range_allows_below_unknown(
        &document,
        "upper-bound-only-lib",
        PackageDependencySection::Dependencies,
        true,
    );
    assert_dependency_spec_range_minimum(
        &document,
        "out-of-order-or-range-lib",
        PackageDependencySection::Dependencies,
        Some(SemverVersion {
            major: 18,
            minor: 0,
            patch: 0,
            pre: None,
        }),
    );
    assert_dependency_spec_range_allows_below_unknown(
        &document,
        "out-of-order-or-range-lib",
        PackageDependencySection::Dependencies,
        false,
    );
    assert_dependency_spec_range_minimum(
        &document,
        "unsafe-or-range-lib",
        PackageDependencySection::Dependencies,
        None,
    );
    assert_dependency_spec_range_allows_below_unknown(
        &document,
        "unsafe-or-range-lib",
        PackageDependencySection::Dependencies,
        true,
    );
    assert_dependency_spec_range_minimum(
        &document,
        "exclusive-lower-bound-lib",
        PackageDependencySection::Dependencies,
        Some(SemverVersion {
            major: 0,
            minor: 1,
            patch: 2,
            pre: None,
        }),
    );
    assert_dependency_spec_exact(
        &document,
        "prerelease-lib",
        PackageDependencySection::Dependencies,
        SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Some("beta.1".to_owned()),
        },
    );
    assert!(
        SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Some("beta.1".to_owned()),
        } < SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: None,
        },
        "semver prerelease should sort below the matching release"
    );
    assert!(
        SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Some("beta.10".to_owned()),
        } > SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Some("beta.2".to_owned()),
        },
        "semver prerelease numeric identifiers should compare numerically"
    );
    assert!(
        SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Some("beta.100000000000000000000".to_owned()),
        } > SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Some("beta.99999999999999999999".to_owned()),
        },
        "semver prerelease numeric identifiers should compare numerically without integer overflow"
    );
    assert!(
        SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Some("alpha.1".to_owned()),
        } < SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Some("alpha.beta".to_owned()),
        },
        "semver prerelease numeric identifiers should sort below non-numeric identifiers"
    );
    assert_dependency_spec_exact(
        &document,
        "build-metadata-lib",
        PackageDependencySection::Dependencies,
        SemverVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: None,
        },
    );
    assert_dependency_spec_kind(
        &document,
        "workspace-lib",
        PackageDependencySection::Dependencies,
        "workspace",
    );
    assert_dependency_spec_kind(
        &document,
        "file-lib",
        PackageDependencySection::Dependencies,
        "file",
    );
    assert_dependency_spec_kind(
        &document,
        "link-lib",
        PackageDependencySection::DevDependencies,
        "link",
    );
    assert_dependency_spec_kind(
        &document,
        "catalog-lib",
        PackageDependencySection::DevDependencies,
        "catalog",
    );
    assert_dependency_spec_range_minimum(
        &document,
        "optional-lib",
        PackageDependencySection::OptionalDependencies,
        Some(SemverVersion {
            major: 1,
            minor: 0,
            patch: 0,
            pre: None,
        }),
    );
    assert_dependency_spec_range_minimum(
        &document,
        "peer-lib",
        PackageDependencySection::PeerDependencies,
        Some(SemverVersion {
            major: 2,
            minor: 0,
            patch: 0,
            pre: None,
        }),
    );
    assert_dependency_spec_kind(
        &document,
        "unsupported-lib",
        PackageDependencySection::Dependencies,
        "unsupported",
    );
}

#[test]
fn malformed_prerelease_dependency_spec_is_unsupported() {
    let document = super::super::parse_document(
        r#"
        {
          "dependencies": {
            "broken-lib": "1.2.3-"
          }
        }
        "#,
    )
    .expect("package.json document should parse");

    assert_dependency_spec_kind(
        &document,
        "broken-lib",
        PackageDependencySection::Dependencies,
        "unsupported",
    );
}

#[test]
fn rejects_non_object_root() {
    let document = super::super::parse_document(r#""not-an-object""#)
        .expect("json parser should still produce a document");

    assert_invalid_document(&document, "package.json root must be a JSON object");
}

#[test]
fn rejects_non_object_scripts() {
    let document = super::super::parse_document(
        r#"
        {
          "scripts": true
        }
        "#,
    )
    .expect("json parser should still produce a document");

    assert_invalid_document(&document, "package.json field `scripts` must be an object");
}

#[test]
fn rejects_non_string_dependency_versions() {
    let document = super::super::parse_document(
        r#"
        {
          "dependencies": {
            "react": true
          }
        }
        "#,
    )
    .expect("json parser should still produce a document");

    assert_invalid_document(
        &document,
        "package.json field `dependencies.react` must be a string",
    );
}
