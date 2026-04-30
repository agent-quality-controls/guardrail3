use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_seo_types::{
    G3TsAstroCallSnapshot, G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState,
    G3TsAstroIntegrationSnapshot, G3TsAstroOutputMode, G3TsAstroStaticObjectProperty,
    G3TsAstroStaticValue, G3TsAstroTrailingSlashPolicy,
};

pub(crate) fn ingest_astro_config_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroConfigSurfaceState {
    let Some(entry) = crate::roots::select_astro_config(crawl, app_root_rel_path) else {
        return G3TsAstroConfigSurfaceState::Missing {
            rel_path: if app_root_rel_path == "." {
                "astro.config.*".to_owned()
            } else {
                format!("{app_root_rel_path}/astro.config.*")
            },
        };
    };

    if !entry.readable {
        return G3TsAstroConfigSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected Astro config unreadable".to_owned(),
        };
    }

    let document =
        match astro_config_parser::parse_document(&crawl.root_abs_path, &entry.path.rel_path) {
            Ok(document) => document,
            Err(error) => {
                return G3TsAstroConfigSurfaceState::ParseError {
                    rel_path: entry.path.rel_path.clone(),
                    reason: error.to_string(),
                };
            }
        };

    if let Some(reason) = astro_config_parser::parse_error_reason(&document) {
        return G3TsAstroConfigSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let Some(typed) = astro_config_parser::typed(&document) else {
        return G3TsAstroConfigSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "Astro config parsed without typed config data".to_owned(),
        };
    };

    G3TsAstroConfigSurfaceState::Parsed {
        snapshot: G3TsAstroConfigSurfaceSnapshot {
            rel_path: typed.selected_config.rel_path.clone(),
            site: typed.site.clone(),
            output: typed.output.map(astro_output_mode),
            out_dir: typed.out_dir.clone(),
            trailing_slash: typed.trailing_slash.map(astro_trailing_slash_policy),
            integrations: typed.integrations.iter().map(astro_integration).collect(),
            adapter: typed.adapter.as_ref().map(astro_adapter_as_integration),
        },
    }
}

fn astro_output_mode(value: astro_config_parser::types::AstroOutputMode) -> G3TsAstroOutputMode {
    match value {
        astro_config_parser::types::AstroOutputMode::Static => G3TsAstroOutputMode::Static,
        astro_config_parser::types::AstroOutputMode::Server => G3TsAstroOutputMode::Server,
    }
}

fn astro_trailing_slash_policy(
    value: astro_config_parser::types::AstroTrailingSlashPolicy,
) -> G3TsAstroTrailingSlashPolicy {
    match value {
        astro_config_parser::types::AstroTrailingSlashPolicy::Always => {
            G3TsAstroTrailingSlashPolicy::Always
        }
        astro_config_parser::types::AstroTrailingSlashPolicy::Never => {
            G3TsAstroTrailingSlashPolicy::Never
        }
        astro_config_parser::types::AstroTrailingSlashPolicy::Ignore => {
            G3TsAstroTrailingSlashPolicy::Ignore
        }
    }
}

fn astro_integration(
    value: &astro_config_parser::types::AstroIntegrationSnapshot,
) -> G3TsAstroIntegrationSnapshot {
    G3TsAstroIntegrationSnapshot {
        source_module: value.source_module.clone(),
        name: value.name.clone(),
        imported_name: value.imported_name.clone(),
        call: value.call.as_ref().map(astro_call),
    }
}

fn astro_adapter_as_integration(
    value: &astro_config_parser::types::AstroAdapterSnapshot,
) -> G3TsAstroIntegrationSnapshot {
    G3TsAstroIntegrationSnapshot {
        source_module: value.source_module.clone(),
        name: value.name.clone(),
        imported_name: value.imported_name.clone(),
        call: value.call.as_ref().map(astro_call),
    }
}

fn astro_call(value: &astro_config_parser::types::AstroCallSnapshot) -> G3TsAstroCallSnapshot {
    G3TsAstroCallSnapshot {
        first_arg: value.first_arg.as_ref().map(astro_static_value),
    }
}

fn astro_static_value(
    value: &astro_config_parser::types::AstroStaticValue,
) -> G3TsAstroStaticValue {
    match value {
        astro_config_parser::types::AstroStaticValue::Bool(value) => {
            G3TsAstroStaticValue::Bool(*value)
        }
        astro_config_parser::types::AstroStaticValue::Number(value) => {
            G3TsAstroStaticValue::Number(*value)
        }
        astro_config_parser::types::AstroStaticValue::String(value) => {
            G3TsAstroStaticValue::String(value.clone())
        }
        astro_config_parser::types::AstroStaticValue::Null => G3TsAstroStaticValue::Null,
        astro_config_parser::types::AstroStaticValue::Array(values) => {
            G3TsAstroStaticValue::Array(values.iter().map(astro_static_value).collect())
        }
        astro_config_parser::types::AstroStaticValue::Object(properties) => {
            G3TsAstroStaticValue::Object(
                properties
                    .iter()
                    .map(|property| G3TsAstroStaticObjectProperty {
                        key: property.key.clone(),
                        value: astro_static_value(&property.value),
                    })
                    .collect(),
            )
        }
        astro_config_parser::types::AstroStaticValue::ImportedIdentifier {
            local_name,
            source_module,
            imported_name,
        } => G3TsAstroStaticValue::ImportedIdentifier {
            local_name: local_name.clone(),
            source_module: source_module.clone(),
            imported_name: imported_name.clone(),
        },
        astro_config_parser::types::AstroStaticValue::UnsupportedExpression { reason } => {
            G3TsAstroStaticValue::UnsupportedExpression {
                reason: reason.clone(),
            }
        }
    }
}
