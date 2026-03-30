use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

/// i18n libraries to detect in package.json dependencies.
const I18N_LIBRARIES: &[&str] = &["next-intl", "react-intl", "i18next", "react-i18next"];

/// Type alias for locale key maps.
type KeyMap = std::collections::BTreeMap<String, std::collections::BTreeSet<String>>;

/// Check i18n completeness for content-type apps.
/// Auto-skips if no i18n library is detected.
#[allow(clippy::disallowed_methods)] // reason: serde_json for package.json inspection
#[allow(clippy::too_many_lines)] // reason: i18n check has multiple sequential detection steps
pub fn check_i18n(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    // Step 1: Detect i18n library in dependencies
    let pkg_path = path.join("package.json");
    let Some(content) = fs.read_file(&pkg_path) else {
        return;
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) else {
        return;
    };

    let deps = json.get("dependencies").and_then(|d| d.as_object());
    let dev_deps = json.get("devDependencies").and_then(|d| d.as_object());

    let i18n_lib = I18N_LIBRARIES.iter().find(|lib| {
        deps.is_some_and(|d| d.contains_key(**lib))
            || dev_deps.is_some_and(|d| d.contains_key(**lib))
    });

    let Some(lib_name) = i18n_lib else {
        // No i18n library detected — skip silently
        return;
    };

    results.push(
        CheckResult::from_parts(
            "T-TOOL-12".to_owned(),
            Severity::Info,
            format!("i18n library detected: {lib_name}"),
            format!("{lib_name} found in dependencies. Checking i18n completeness."),
            Some(pkg_path.display().to_string()),
            None,
            false,
        )
        .as_inventory(),
    );

    // Step 2: Check for messages directory
    let messages_dirs = &["messages", "locales", "translations", "i18n"];
    let mut found_dir: Option<std::path::PathBuf> = None;

    for dir_name in messages_dirs {
        let dir = path.join(dir_name);
        if fs.metadata(&dir).is_some_and(|m| m.is_dir()) {
            found_dir = Some(dir);
            break;
        }
        // Also check in apps/*/
        for entry in fs.list_dir(&path.join("apps")) {
            let app_dir = entry.path().join(dir_name);
            if fs.metadata(&app_dir).is_some_and(|m| m.is_dir()) {
                found_dir = Some(app_dir);
                break;
            }
        }
        if found_dir.is_some() {
            break;
        }
    }

    let Some(msg_dir) = found_dir else {
        results.push(CheckResult::from_parts(
    "T-TOOL-12".to_owned(),
    Severity::Warn,
    "No i18n message directory found".to_owned(),
    format!(
                "{lib_name} is installed but no messages/locales/translations directory found. \
                 Create a messages/ directory with per-locale JSON files (e.g., en.json, fr.json)."
            ),
    Some(path.display().to_string()),
    None,
    false,
        ));
        return;
    };

    // Step 3: Check multiple locale files exist
    let json_files: Vec<String> = fs
        .list_dir(&msg_dir)
        .iter()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if Path::new(&name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    if json_files.len() < 2 {
        results.push(CheckResult {
            id: "T-TOOL-12".to_owned(),
            severity: Severity::Warn,
            title: "Only one locale file found".to_owned(),
            message: format!(
                "Found {} JSON file(s) in {}. For i18n to work correctly, \
                 you need at least 2 locale files (e.g., en.json and fr.json).",
                json_files.len(),
                msg_dir.display()
            ),
            file: Some(msg_dir.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    // Step 4: Compare top-level keys across locale files
    let mut all_keys: KeyMap = KeyMap::new();

    for file_name in &json_files {
        let locale_path = msg_dir.join(file_name);
        if let Some(locale_content) = fs.read_file(&locale_path) {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&locale_content) {
                if let Some(obj) = parsed.as_object() {
                    let keys: std::collections::BTreeSet<String> = obj.keys().cloned().collect();
                    let _ = all_keys.insert(file_name.clone(), keys);
                }
            }
        }
    }

    if all_keys.len() < 2 {
        return; // Couldn't parse enough files
    }

    // Find the union of all keys
    let mut union_keys: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for keys in all_keys.values() {
        union_keys = union_keys.union(keys).cloned().collect();
    }

    // Check each file against the union
    let mut missing_report: Vec<String> = Vec::new();
    for (file_name, keys) in &all_keys {
        let missing: Vec<&String> = union_keys.difference(keys).collect();
        if !missing.is_empty() {
            let missing_list: Vec<&str> = missing.iter().map(|s| s.as_str()).collect();
            missing_report.push(format!("{file_name} missing: {}", missing_list.join(", ")));
        }
    }

    if missing_report.is_empty() {
        results.push(
            CheckResult::from_parts(
                "T-TOOL-12".to_owned(),
                Severity::Info,
                "i18n locale files consistent".to_owned(),
                format!(
                    "All {} locale files have the same {} top-level keys.",
                    all_keys.len(),
                    union_keys.len()
                ),
                Some(msg_dir.display().to_string()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "T-TOOL-12".to_owned(),
            severity: Severity::Error,
            title: "i18n locale files have missing keys".to_owned(),
            message: format!(
                "Translation key mismatch across locale files:\n{}",
                missing_report.join("\n")
            ),
            file: Some(msg_dir.display().to_string()),
            line: None,
            inventory: false,
        ));
    }
