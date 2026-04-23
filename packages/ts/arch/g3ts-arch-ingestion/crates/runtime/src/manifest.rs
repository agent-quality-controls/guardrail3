use g3_workspace_crawl::{G3RsWorkspaceCrawl as G3WorkspaceCrawl, root_file};
use g3ts_arch_types::{
    G3TsArchDeclaredEntryPoint, G3TsArchEntryPointSource, G3TsArchManifestSnapshot,
    G3TsArchManifestState,
};
use package_json_parser::{from_path_document, parse_error_reason};
use serde_json::Value;

pub(crate) fn ingest_manifest_state(crawl: &G3WorkspaceCrawl) -> G3TsArchManifestState {
    let Some(entry) = root_file(crawl, "package.json") else {
        return G3TsArchManifestState::Missing;
    };

    if !entry.readable {
        return G3TsArchManifestState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected root manifest unreadable".to_owned(),
        };
    }

    let document = match from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return G3TsArchManifestState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        return G3TsArchManifestState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let declared_entrypoints = declared_entrypoints(&document.raw);

    G3TsArchManifestState::Parsed {
        snapshot: G3TsArchManifestSnapshot {
            rel_path: entry.path.rel_path.clone(),
            declared_entrypoints,
        },
    }
}

fn declared_entrypoints(raw: &Value) -> Vec<G3TsArchDeclaredEntryPoint> {
    let mut entrypoints = Vec::new();

    if let Some(types) = raw.get("types").and_then(Value::as_str)
        && let Some(rel_path) = normalize_source_entrypoint(types)
    {
        entrypoints.push(G3TsArchDeclaredEntryPoint {
            source: G3TsArchEntryPointSource::Types,
            rel_path,
        });
    }

    if let Some(exports_dot) = raw.get("exports").and_then(|exports| exports.get(".")) {
        collect_export_paths(exports_dot, &mut entrypoints);
    }

    entrypoints.sort_by(|left, right| {
        left.rel_path
            .cmp(&right.rel_path)
            .then(left.source.cmp(&right.source))
    });
    entrypoints.dedup_by(|left, right| left.rel_path == right.rel_path);
    entrypoints
}

fn collect_export_paths(value: &Value, entrypoints: &mut Vec<G3TsArchDeclaredEntryPoint>) {
    match value {
        Value::String(path) => {
            if let Some(rel_path) = normalize_source_entrypoint(path) {
                entrypoints.push(G3TsArchDeclaredEntryPoint {
                    source: G3TsArchEntryPointSource::ExportsDot,
                    rel_path,
                });
            }
        }
        Value::Object(map) => {
            for nested in map.values() {
                collect_export_paths(nested, entrypoints);
            }
        }
        Value::Array(items) => {
            for nested in items {
                collect_export_paths(nested, entrypoints);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

fn normalize_source_entrypoint(path: &str) -> Option<String> {
    let trimmed = path.strip_prefix("./").unwrap_or(path);
    let valid = matches!(
        trimmed,
        "src/index.ts" | "src/index.tsx" | "index.ts" | "index.tsx"
    ) || matches_noncanonical_source_entrypoint(trimmed);

    valid.then(|| trimmed.to_owned())
}

fn matches_noncanonical_source_entrypoint(path: &str) -> bool {
    let file_name = std::path::Path::new(path)
        .file_name()
        .and_then(|name| name.to_str());
    let extension = std::path::Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str());

    file_name.is_some()
        && matches!(extension, Some("ts" | "tsx"))
        && !path.ends_with(".d.ts")
        && !path.starts_with("dist/")
}
