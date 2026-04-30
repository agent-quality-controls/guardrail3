use std::path::Path;
use std::process::Command;

pub(super) use crate::types::{
    EslintConfigDocument, EslintConfigParseState, EslintConfigSnapshot, EslintProbeTarget,
};

#[cfg(test)]
pub(crate) use crate::types::{
    EslintConfigFileKind, EslintProbeKind, EslintReportUnusedSetting, EslintRuleSeverity,
};

const NODE_HELPER: &str = r#"
import path from 'node:path';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';

function fileKind(relPath) {
  if (relPath.endsWith('.config.js') || relPath.endsWith('.js')) return 'Js';
  if (relPath.endsWith('.config.mjs') || relPath.endsWith('.mjs')) return 'Mjs';
  if (relPath.endsWith('.config.cjs') || relPath.endsWith('.cjs')) return 'Cjs';
  if (relPath.endsWith('.config.ts') || relPath.endsWith('.ts')) return 'Ts';
  if (relPath.endsWith('.config.mts') || relPath.endsWith('.mts')) return 'Mts';
  if (relPath.endsWith('.config.cts') || relPath.endsWith('.cts')) return 'Cts';
  throw new Error(`unsupported eslint config file kind: ${relPath}`);
}

function normalizeSeverity(value) {
  if (value === 0 || value === 'off') return 'Off';
  if (value === 1 || value === 'warn') return 'Warn';
  if (value === 2 || value === 'error') return 'Error';
  throw new Error(`unsupported rule severity: ${JSON.stringify(value)}`);
}

function normalizeRule(value) {
  if (Array.isArray(value)) {
    return {
      severity: normalizeSeverity(value[0]),
      options: value.slice(1),
    };
  }

  return {
    severity: normalizeSeverity(value),
    options: [],
  };
}

function normalizeReportUnusedSetting(value) {
  if (value === undefined || value === null) return null;
  if (value === false || value === 0 || value === 'off') return 'Off';
  if (value === true || value === 1 || value === 'warn') return 'Warn';
  if (value === 2 || value === 'error') return 'Error';
  throw new Error(`unsupported linter option severity: ${JSON.stringify(value)}`);
}

function normalizePlugins(plugins) {
  if (!plugins || typeof plugins !== 'object') {
    return [];
  }

  return Object.keys(plugins).sort();
}

function normalizePluginMetaNames(plugins) {
  if (!plugins || typeof plugins !== 'object') {
    return {};
  }

  const metaNames = {};
  for (const namespace of Object.keys(plugins).sort()) {
    const metaName = plugins[namespace]?.meta?.name;
    if (typeof metaName === 'string' && metaName.length > 0) {
      metaNames[namespace] = metaName;
    }
  }
  return metaNames;
}

function candidatePluginPackageNames(namespace, plugin) {
  const candidates = new Set();
  const metaName = plugin?.meta?.name;
  if (typeof metaName === 'string' && metaName.length > 0) {
    candidates.add(metaName);
  }
  if (/^[a-z0-9-]+$/.test(namespace)) {
    candidates.add(`eslint-plugin-${namespace}`);
  }
  return [...candidates].sort();
}

function packageModuleExportsPlugin(module, plugin) {
  const pluginCandidates = pluginObjectCandidates(plugin);
  return exportedPluginCandidates(module).some((moduleCandidate) =>
    pluginCandidates.some((pluginCandidate) => moduleCandidate === pluginCandidate)
  );
}

function packageModuleFingerprintMatchesPlugin(module, plugin) {
  const effectiveFingerprint = pluginFingerprint(plugin);
  return exportedPluginCandidates(module).some((candidate) =>
    pluginFingerprint(candidate) === effectiveFingerprint
  );
}

function exportedPluginCandidates(module) {
  return pluginObjectCandidates(module);
}

function pluginObjectCandidates(value) {
  return [value, value?.default, value?.['module.exports'], ...Object.values(value ?? {})].filter(
    (value) => value && typeof value === 'object'
  );
}

function pluginFingerprint(plugin) {
  return JSON.stringify({
    metaName: plugin?.meta?.name ?? null,
    rules: Object.fromEntries(
      Object.entries(plugin?.rules ?? {})
        .sort(([left], [right]) => left.localeCompare(right))
        .map(([ruleName, rule]) => [
          ruleName,
          {
            meta: rule?.meta ?? null,
            keys: Object.keys(rule ?? {}).sort(),
          },
        ])
    ),
    processorNames: Object.keys(plugin?.processors ?? {}).sort(),
    configNames: Object.keys(plugin?.configs ?? {}).sort(),
  });
}

async function normalizePluginPackageNames(plugins, require) {
  if (!plugins || typeof plugins !== 'object') {
    return {};
  }

  const packageNames = {};
  for (const namespace of Object.keys(plugins).sort()) {
    const plugin = plugins[namespace];
    const matches = new Set();
    for (const packageName of candidatePluginPackageNames(namespace, plugin)) {
      for (const module of await importCandidatePluginPackage(packageName, require)) {
        if (
          packageModuleExportsPlugin(module, plugin) ||
          packageModuleFingerprintMatchesPlugin(module, plugin)
        ) {
          matches.add(packageName);
        }
      }
    }
    if (matches.size > 0) {
      packageNames[namespace] = [...matches].sort();
    }
  }
  return packageNames;
}

async function importCandidatePluginPackage(packageName, require) {
  const modules = [];
  try {
    const resolved = require.resolve(packageName);
    modules.push(await import(pathToFileURL(resolved).href));
  } catch {
  }

  try {
    modules.push(await import(packageName));
  } catch {
  }

  return modules;
}

const workspaceRoot = process.env.G3_WORKSPACE_ROOT;
const configRelPath = process.env.G3_CONFIG_REL_PATH;
const probes = JSON.parse(process.env.G3_PROBES_JSON ?? '[]');

if (!workspaceRoot || !configRelPath) {
  throw new Error('workspace root and config rel path are required');
}

const configAbsPath = path.join(workspaceRoot, configRelPath);
const configRequire = createRequire(configAbsPath);
const { ESLint } = configRequire('eslint');
const eslint = new ESLint({
  cwd: workspaceRoot,
  overrideConfigFile: configAbsPath,
});

const payload = {
  selected_config: {
    rel_path: configRelPath,
    kind: fileKind(configRelPath),
  },
  probes: [],
};

for (const probe of probes) {
  const probeAbsPath = path.join(workspaceRoot, probe.rel_path);
  const ignored = await eslint.isPathIgnored(probeAbsPath);
  const config = await eslint.calculateConfigForFile(probeAbsPath);
  if (config === undefined || config === null) {
    payload.probes.push({
      probe: probe.probe,
      rel_path: probe.rel_path,
      ignored,
      plugins: [],
      plugin_meta_names: {},
      plugin_package_names: {},
      rules: {},
      project_service: null,
      linter_options_no_inline_config: null,
      linter_options_report_unused_disable_directives: null,
      linter_options_report_unused_inline_configs: null,
    });
    continue;
  }
  const linterOptions = config.linterOptions ?? {};
  const rules = {};
  for (const ruleName of Object.keys(config.rules ?? {}).sort()) {
    rules[ruleName] = normalizeRule(config.rules[ruleName]);
  }
  const projectService = config.languageOptions?.parserOptions?.projectService;
  payload.probes.push({
    probe: probe.probe,
    rel_path: probe.rel_path,
    ignored,
    plugins: normalizePlugins(config.plugins),
    plugin_meta_names: normalizePluginMetaNames(config.plugins),
    plugin_package_names: await normalizePluginPackageNames(config.plugins, configRequire),
    rules,
    project_service: typeof projectService === 'boolean' ? projectService : null,
    linter_options_no_inline_config:
      typeof linterOptions.noInlineConfig === 'boolean'
        ? linterOptions.noInlineConfig
        : null,
    linter_options_report_unused_disable_directives:
      normalizeReportUnusedSetting(linterOptions.reportUnusedDisableDirectives),
    linter_options_report_unused_inline_configs:
      normalizeReportUnusedSetting(linterOptions.reportUnusedInlineConfigs),
  });
}

console.log(JSON.stringify(payload));
"#;

pub fn parse(
    workspace_root: impl AsRef<Path>,
    config_rel_path: &str,
    probes: &[EslintProbeTarget],
) -> Result<EslintConfigSnapshot, crate::error::Error> {
    let document = parse_document(workspace_root, config_rel_path, probes)?;
    match document.typed {
        EslintConfigParseState::Parsed(snapshot) => Ok(snapshot),
        EslintConfigParseState::Invalid(reason) => Err(crate::error::Error::Json(reason)),
    }
}

pub fn parse_document(
    workspace_root: impl AsRef<Path>,
    config_rel_path: &str,
    probes: &[EslintProbeTarget],
) -> Result<EslintConfigDocument, crate::error::Error> {
    let raw = evaluate(workspace_root.as_ref(), config_rel_path, probes)?;
    let typed = match serde_json::from_value::<EslintConfigSnapshot>(raw.clone()) {
        Ok(snapshot) => EslintConfigParseState::Parsed(snapshot),
        Err(err) => EslintConfigParseState::Invalid(err.to_string()),
    };
    Ok(EslintConfigDocument { raw, typed })
}

pub fn from_path(
    workspace_root: impl AsRef<Path>,
    config_rel_path: &str,
    probes: &[EslintProbeTarget],
) -> Result<EslintConfigSnapshot, crate::error::Error> {
    parse(workspace_root, config_rel_path, probes)
}

fn evaluate(
    workspace_root: &Path,
    config_rel_path: &str,
    probes: &[EslintProbeTarget],
) -> Result<serde_json::Value, crate::error::Error> {
    let probes_json = serde_json::to_string(probes)?;
    let output = Command::new("node")
        .arg("--input-type=module")
        .arg("--eval")
        .arg(NODE_HELPER)
        .current_dir(
            workspace_root
                .join(config_rel_path)
                .parent()
                .unwrap_or(workspace_root),
        )
        .env("G3_WORKSPACE_ROOT", workspace_root)
        .env("G3_CONFIG_REL_PATH", config_rel_path)
        .env("G3_PROBES_JSON", probes_json)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        return Err(crate::error::Error::Helper(stderr));
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"] // reason: owned sidecar tests for parser entrypoints.
mod parser_tests;
