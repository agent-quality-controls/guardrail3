#![allow(
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain and document role"
)]

use std::cmp::Ordering;
use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageJsonDocument {
    pub raw: Value,
    pub typed: PackageJsonParseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageJsonParseState {
    Parsed(Box<PackageJsonSnapshot>),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PackageJsonSnapshot {
    pub private_field: Option<bool>,
    pub package_manager: Option<String>,
    pub engines_node: Option<String>,
    pub engines_pnpm: Option<String>,
    pub scripts: BTreeMap<String, String>,
    pub pnpm_override_keys: Vec<String>,
    pub pnpm_only_built_dependencies: Vec<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub peer_dependencies: Vec<String>,
    pub dependency_specs: Vec<PackageDependencySpec>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageJsonBoolFieldState<'a> {
    Missing,
    Value(bool),
    WrongType(&'a Value),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageDependencySpec {
    pub name: String,
    pub raw_spec: String,
    pub section: PackageDependencySection,
    pub parsed: PackageDependencySpecParseState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageDependencySection {
    Dependencies,
    DevDependencies,
    OptionalDependencies,
    PeerDependencies,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageDependencySpecParseState {
    Exact {
        version: SemverVersion,
    },
    Range {
        minimum: Option<SemverVersion>,
        allows_below_minimum_unknown: bool,
    },
    Workspace {
        raw: String,
    },
    File {
        raw: String,
    },
    Link {
        raw: String,
    },
    Catalog {
        raw: String,
    },
    Unsupported {
        raw: String,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemverVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

impl PartialOrd for SemverVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SemverVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch)
            .cmp(&(other.major, other.minor, other.patch))
            .then_with(|| match (&self.pre, &other.pre) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(self_pre), Some(other_pre)) => compare_prerelease(self_pre, other_pre),
            })
    }
}

fn compare_prerelease(left: &str, right: &str) -> Ordering {
    let mut left_parts = left.split('.');
    let mut right_parts = right.split('.');

    loop {
        match (left_parts.next(), right_parts.next()) {
            (Some(left_part), Some(right_part)) => {
                let ordering = compare_prerelease_identifier(left_part, right_part);
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (None, None) => return Ordering::Equal,
        }
    }
}

fn compare_prerelease_identifier(left: &str, right: &str) -> Ordering {
    match (numeric_identifier(left), numeric_identifier(right)) {
        (Some(left_number), Some(right_number)) => compare_numeric_identifier(left_number, right_number),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => left.cmp(right),
    }
}

fn numeric_identifier(identifier: &str) -> Option<&str> {
    identifier
        .bytes()
        .all(|byte| byte.is_ascii_digit())
        .then_some(identifier)
}

fn compare_numeric_identifier(left: &str, right: &str) -> Ordering {
    let normalized_left = trim_numeric_identifier_zeroes(left);
    let normalized_right = trim_numeric_identifier_zeroes(right);
    normalized_left
        .len()
        .cmp(&normalized_right.len())
        .then_with(|| normalized_left.cmp(normalized_right))
}

fn trim_numeric_identifier_zeroes(identifier: &str) -> &str {
    let trimmed = identifier.trim_start_matches('0');
    if trimmed.is_empty() { "0" } else { trimmed }
}
