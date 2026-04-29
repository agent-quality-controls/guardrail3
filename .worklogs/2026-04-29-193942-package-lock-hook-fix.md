Summary
- Fixed the pre-commit lockfile check that incorrectly assumed every staged `package.json` belongs to a root pnpm workspace.
- The hook now validates lockfiles in the changed package directory and no longer fails because the repository root has no `pnpm-lock.yaml`.

Decisions made
- Did not add a fake root `pnpm-lock.yaml`; the repo root has no `package.json` and is not a pnpm workspace root.
- Kept package changes fail-closed: package-local `package-lock.json` is checked with `npm install --package-lock-only --ignore-scripts --dry-run`, package-local `pnpm-lock.yaml` is checked with `pnpm install --frozen-lockfile`, and packages without a supported lockfile fail.
- Rejected `npm ci --dry-run` because it mutates existing `node_modules` contents.

Key files for context
- `.githooks/pre-commit`
- `.plans/2026-04-29-193810-package-lock-hook-fix.md`

Verification
- `bash -n .githooks/pre-commit`
- `npm install --package-lock-only --ignore-scripts --dry-run` in `packages/ts/g3ts-eslint-plugin-astro-i18n-policy`
- `npm install --package-lock-only --ignore-scripts --dry-run` in `packages/ts/g3ts-eslint-plugin-astro-pipeline`
- `npm install --package-lock-only --ignore-scripts --dry-run` in `packages/ts/g3ts-astro-nuasite-checks`

Next steps
- Re-run a normal commit with staged hook changes to prove the hook no longer blocks on the absent root pnpm lockfile.
