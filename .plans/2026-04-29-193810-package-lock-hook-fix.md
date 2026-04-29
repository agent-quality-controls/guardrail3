Goal
- Fix the pre-commit package lock check so commits with standalone npm package changes do not fail because the repository root has no pnpm workspace lockfile.

Approach
- Update `.githooks/pre-commit` in the lockfile integrity section.
- Keep root pnpm validation only when the repository actually has a pnpm root marker: `pnpm-lock.yaml`, `pnpm-workspace.yaml`, or root `package.json`.
- For each staged `package.json`, validate the package manager lockfile in that package directory:
  - if `package-lock.json` exists, run `npm ci --ignore-scripts --dry-run` in that directory.
  - if `pnpm-lock.yaml` exists, run `pnpm install --frozen-lockfile` in that directory.
  - if no known lockfile exists, fail with a direct error naming the package directory.
- Preserve fail-closed behavior for real package drift.

Key decisions
- Do not add a fake root `pnpm-lock.yaml`; there is no root npm/pnpm package manifest.
- Do not suppress lockfile checks; make them target the package that changed.
- Do not change unrelated TypeScript formatting/lint sections in this hook.

Files to modify
- `.githooks/pre-commit`
