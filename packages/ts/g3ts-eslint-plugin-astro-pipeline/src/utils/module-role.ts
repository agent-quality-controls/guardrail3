import type { ResolvedAstroPipelineOptions } from "./options.js";
import { inferPathPolicyRoot, matchesFileGlobs } from "./path-policy.js";

export interface ModuleRole {
  isRoute: boolean;
  isEndpoint: boolean;
  isRouteOrEndpoint: boolean;
  isApprovedContentAdapter: boolean;
  isMdxContent: boolean;
  isApprovedLoader: boolean;
  isMdxRuntimeModule: boolean;
  isRouteRegistryModule: boolean;
  isAdapterModule: boolean;
  isApprovedGeneratedArtifact: boolean;
}

export function classifyModuleRole(
  filename: string,
  options: ResolvedAstroPipelineOptions,
  cwd = inferPathPolicyRoot(filename)
): ModuleRole {
  const isRoute = matchesFileGlobs(filename, options.routeGlobs, cwd);
  const isEndpoint = matchesFileGlobs(filename, options.endpointGlobs, cwd);

  return {
    isRoute,
    isEndpoint,
    isRouteOrEndpoint: isRoute || isEndpoint,
    isApprovedContentAdapter: matchesFileGlobs(
      filename,
      options.approvedContentAdapterModules,
      cwd
    ),
    isMdxContent: matchesFileGlobs(filename, options.mdxContentGlobs, cwd),
    isApprovedLoader: matchesFileGlobs(filename, options.approvedLoaderModules, cwd),
    isMdxRuntimeModule: matchesFileGlobs(filename, options.mdxRuntimeModuleGlobs, cwd),
    isRouteRegistryModule: matchesFileGlobs(
      filename,
      options.routeRegistryModuleGlobs,
      cwd
    ),
    isAdapterModule: matchesFileGlobs(filename, options.adapterModuleGlobs, cwd),
    isApprovedGeneratedArtifact: matchesFileGlobs(
      filename,
      options.approvedGeneratedArtifactRoots,
      cwd
    )
  };
}
