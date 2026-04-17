use std::collections::BTreeSet;

use deny_toml_parser::types::{
    AdvisoryIgnoreEntry, BanAllowEntry, BanDenyDetail, BanDenyEntry, BanFeatureEntry,
    BanSkipDetail, BanSkipEntry, LicenseException,
};

pub(crate) fn join_set(values: &BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join(", ")
}

pub(crate) fn ban_name(entry: &BanDenyEntry) -> Option<String> {
    match entry {
        BanDenyEntry::Simple(name) => normalized_name(name),
        BanDenyEntry::Detailed(detail) => deny_detail_name(detail),
    }
}

pub(crate) fn allow_name(entry: &BanAllowEntry) -> Option<String> {
    match entry {
        BanAllowEntry::Simple(name) => normalized_name(name),
        BanAllowEntry::Detailed(detail) => detail
            .name
            .as_deref()
            .and_then(normalized_name)
            .or_else(|| {
                detail.crate_name.as_deref().and_then(|crate_name| {
                    normalized_name(crate_name.split('@').next().unwrap_or(crate_name))
                })
            }),
    }
}

pub(crate) fn wrappers(entry: &BanDenyEntry) -> BTreeSet<String> {
    match entry {
        BanDenyEntry::Simple(_) => BTreeSet::new(),
        BanDenyEntry::Detailed(detail) => detail.wrappers.iter().cloned().collect(),
    }
}

pub(crate) fn deny_entry_name(entry: &BanDenyEntry) -> Option<String> {
    match entry {
        BanDenyEntry::Simple(name) => Some(name.clone()),
        BanDenyEntry::Detailed(detail) => deny_detail_name(detail),
    }
}

pub(crate) fn skip_entry_name(entry: &BanSkipEntry) -> Option<String> {
    match entry {
        BanSkipEntry::Simple(name) => Some(name.clone()),
        BanSkipEntry::Detailed(detail) => skip_detail_name(detail),
    }
}

pub(crate) fn skip_entry_reason(entry: &BanSkipEntry) -> Option<&str> {
    match entry {
        BanSkipEntry::Simple(_) => None,
        BanSkipEntry::Detailed(detail) => detail.reason.as_deref(),
    }
}

pub(crate) fn normalized_skip_identity(entry: &BanSkipEntry) -> Option<String> {
    match entry {
        BanSkipEntry::Simple(name) => {
            let name = name.trim();
            (!name.is_empty()).then(|| name.to_owned())
        }
        BanSkipEntry::Detailed(detail) => {
            let Some(name) = skip_detail_name(detail) else {
                return None;
            };
            let version = skip_detail_version(detail)
                .map(str::trim)
                .filter(|value| !value.is_empty());
            Some(match version {
                Some(version) => format!("{name}@{version}"),
                None => name,
            })
        }
    }
}

pub(crate) fn feature_entry_name(entry: &BanFeatureEntry) -> Option<String> {
    entry.name.as_deref().map(str::to_owned).or_else(|| {
        entry.crate_name.as_deref().map(|crate_name| {
            crate_name
                .split('@')
                .next()
                .unwrap_or(crate_name)
                .to_owned()
        })
    })
}

pub(crate) fn license_exception_name(entry: &LicenseException) -> Option<String> {
    entry
        .name
        .as_deref()
        .map(str::to_owned)
        .or_else(|| entry.crate_name.as_deref().map(str::to_owned))
}

pub(crate) fn advisory_ignore_identity(entry: &AdvisoryIgnoreEntry) -> Option<String> {
    match entry {
        AdvisoryIgnoreEntry::Simple(id) => {
            let id = id.trim();
            (!id.is_empty()).then(|| id.to_owned())
        }
        AdvisoryIgnoreEntry::Detailed(detail) => {
            if let Some(id) = detail
                .id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
            {
                return Some(id.to_owned());
            }
            if let Some(crate_name) = detail
                .crate_name
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
            {
                return Some(crate_name.to_owned());
            }
            let name = detail
                .name
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())?;
            let version = detail
                .version
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty());
            Some(match version {
                Some(version) => format!("{name}@{version}"),
                None => name.to_owned(),
            })
        }
    }
}

pub(crate) fn advisory_ignore_reason(entry: &AdvisoryIgnoreEntry) -> Option<&str> {
    match entry {
        AdvisoryIgnoreEntry::Simple(_) => None,
        AdvisoryIgnoreEntry::Detailed(detail) => detail.reason.as_deref(),
    }
}

fn deny_detail_name(detail: &BanDenyDetail) -> Option<String> {
    detail
        .name
        .as_deref()
        .and_then(normalized_name)
        .or_else(|| {
            detail.crate_name.as_deref().and_then(|crate_name| {
                normalized_name(crate_name.split('@').next().unwrap_or(crate_name))
            })
        })
}

fn normalized_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

fn skip_detail_name(detail: &BanSkipDetail) -> Option<String> {
    detail.name.as_deref().map(str::to_owned).or_else(|| {
        detail.crate_name.as_deref().map(|crate_name| {
            crate_name
                .split('@')
                .next()
                .unwrap_or(crate_name)
                .to_owned()
        })
    })
}

fn skip_detail_version(detail: &BanSkipDetail) -> Option<&str> {
    detail.version.as_deref()
}
