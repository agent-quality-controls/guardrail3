import path from "node:path";

type RuleContextWithFilename = {
  filename?: string;
  physicalFilename?: string;
};

export function normalizeConfiguredModules(modules: string[]): Set<string> {
  return new Set(modules.map(normalizeModuleId));
}

export function importMatchesConfiguredModule(
  source: string,
  context: RuleContextWithFilename,
  configuredModules: Set<string>
): boolean {
  const normalizedSource = normalizeModuleId(source);
  if (configuredModules.has(normalizedSource)) {
    return true;
  }

  if (!source.startsWith(".")) {
    return false;
  }

  const filename = context.physicalFilename ?? context.filename;
  if (!filename || filename.startsWith("<")) {
    return false;
  }

  const resolved = normalizeModuleId(path.resolve(path.dirname(filename), source));

  return [...configuredModules].some((configured) => {
    const configuredAbsolutePath = configuredModuleAbsolutePath(filename, configured);

    return configuredAbsolutePath !== null && resolved === configuredAbsolutePath;
  });
}

function normalizeModuleId(value: string): string {
  let normalized = value.replaceAll(path.sep, "/");
  normalized = normalized.replace(/\/+/g, "/");
  normalized = normalized.replace(/^\.\//, "");
  normalized = normalized.replace(/\.(?:mjs|cjs|jsx|tsx|ts|js)$/, "");

  return normalized;
}

function configuredModuleAbsolutePath(filename: string, configured: string): string | null {
  if (path.isAbsolute(configured)) {
    return normalizeModuleId(configured);
  }

  const configuredPrefix = configured.split("/").at(0);
  if (!configuredPrefix) {
    return null;
  }

  const normalizedFilename = normalizeModuleId(filename);
  const marker = `/${configuredPrefix}/`;
  const markerIndex = normalizedFilename.lastIndexOf(marker);
  if (markerIndex === -1 || normalizedFilename === configuredPrefix) {
    return null;
  }

  const appRoot = normalizedFilename.slice(0, markerIndex);

  return normalizeModuleId(path.posix.join(appRoot, configured));
}
