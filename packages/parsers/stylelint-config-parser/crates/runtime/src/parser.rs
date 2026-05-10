#![allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized stylelint config parser; std::process::Command::new spawns the Node helper, and serde_json::from_value/from_slice are the core deserialization path"
)]

use std::path::Path;
use std::process::Command;

pub(super) use crate::types::{
    StylelintConfigDocument, StylelintConfigParseState, StylelintConfigSnapshot,
    StylelintProbeTarget,
};

/// JavaScript helper script that probes stylelint config resolution and emits a typed snapshot.
const NODE_HELPER: &str = r"
import path from 'node:path';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';

function fileKind(relPath) {
  if (relPath.endsWith('.mjs')) return 'Mjs';
  if (relPath.endsWith('.cjs')) return 'Cjs';
  if (relPath.endsWith('.js')) return 'Js';
  throw new Error(`unsupported stylelint config file kind: ${relPath}`);
}

function normalizeStringArray(value) {
  if (typeof value === 'string') return [value];
  if (Array.isArray(value)) return value.filter((item) => typeof item === 'string');
  return [];
}

const workspaceRoot = process.env.G3_WORKSPACE_ROOT;
const configRelPath = process.env.G3_CONFIG_REL_PATH;
const probes = JSON.parse(process.env.G3_PROBES_JSON ?? '[]');

if (!workspaceRoot || !configRelPath) {
  throw new Error('workspace root and config rel path are required');
}

const configAbsPath = path.join(workspaceRoot, configRelPath);
const configRequire = createRequire(configAbsPath);
const stylelint = configRequire('stylelint');
const module = await import(pathToFileURL(configAbsPath).href);
const rawConfig = module.default ?? module;

const payload = {
  selected_config: {
    rel_path: configRelPath,
    kind: fileKind(configRelPath),
  },
  raw_extends: normalizeStringArray(rawConfig.extends),
  raw_plugins: normalizeStringArray(rawConfig.plugins),
  probes: [],
};

for (const probe of probes) {
  const probeAbsPath = path.join(workspaceRoot, probe.rel_path);
  const config = await stylelint.resolveConfig(probeAbsPath, { configFile: configAbsPath });
  payload.probes.push({
    rel_path: probe.rel_path,
    ignored: config === undefined || config === null,
    extends: normalizeStringArray(config?.extends),
    plugins: normalizeStringArray(config?.plugins),
    rules: config?.rules ?? {},
  });
}

console.log(JSON.stringify(payload));
";

/// Resolves the stylelint config at `config_rel_path` (relative to `workspace_root`) into a typed snapshot.
///
/// # Errors
/// Returns [`crate::Error::Json`] when the helper output cannot be deserialized into the snapshot schema, or
/// [`crate::Error::Helper`] when the Node helper process itself fails.
pub fn parse(
    workspace_root: impl AsRef<Path>,
    config_rel_path: &str,
    probes: &[StylelintProbeTarget],
) -> Result<StylelintConfigSnapshot, crate::Error> {
    let document = parse_document(workspace_root, config_rel_path, probes)?;
    match document.typed {
        StylelintConfigParseState::Parsed(snapshot) => Ok(snapshot),
        StylelintConfigParseState::Invalid(reason) => Err(crate::Error::Json(reason)),
    }
}

/// Resolves the stylelint config into a [`StylelintConfigDocument`], capturing typed-parse failures as `Invalid`.
///
/// # Errors
/// Returns [`crate::Error::Helper`] when the Node helper process fails to produce raw JSON output.
pub fn parse_document(
    workspace_root: impl AsRef<Path>,
    config_rel_path: &str,
    probes: &[StylelintProbeTarget],
) -> Result<StylelintConfigDocument, crate::Error> {
    let raw = evaluate(workspace_root.as_ref(), config_rel_path, probes)?;
    let typed = match serde_json::from_value::<StylelintConfigSnapshot>(raw.clone()) {
        Ok(snapshot) => StylelintConfigParseState::Parsed(snapshot),
        Err(err) => StylelintConfigParseState::Invalid(err.to_string()),
    };
    Ok(StylelintConfigDocument { raw, typed })
}

/// Reads the stylelint config and resolves it into a typed snapshot.
///
/// # Errors
/// Returns [`crate::Error::Json`] when the helper output cannot be deserialized, or [`crate::Error::Helper`] when the Node helper fails.
pub fn from_path(
    workspace_root: impl AsRef<Path>,
    config_rel_path: &str,
    probes: &[StylelintProbeTarget],
) -> Result<StylelintConfigSnapshot, crate::Error> {
    parse(workspace_root, config_rel_path, probes)
}

/// Spawns the Node helper script that resolves the stylelint config and returns its raw JSON output.
fn evaluate(
    workspace_root: &Path,
    config_rel_path: &str,
    probes: &[StylelintProbeTarget],
) -> Result<serde_json::Value, crate::Error> {
    let probes_json = serde_json::to_string(probes)?;
    let output = Command::new("node")
        .arg("--input-type=module")
        .arg("--eval")
        .arg(NODE_HELPER)
        .env("G3_WORKSPACE_ROOT", workspace_root)
        .env("G3_CONFIG_REL_PATH", config_rel_path)
        .env("G3_PROBES_JSON", probes_json)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        return Err(crate::Error::Helper(stderr));
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}
