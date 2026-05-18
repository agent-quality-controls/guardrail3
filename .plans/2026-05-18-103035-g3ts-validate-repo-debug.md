# G3TS Validate-Repo Debug Plan

## Goal

Make Slopless pass current G3TS repository validation after Guardrail3 fixes clarify the hook contract and documentation.

This plan records what `g3ts validate-repo` currently reports against Slopless and what remains unclear in Guardrail3. Do not implement the Slopless hook changes until the Guardrail3 issues below are resolved.

## Observed Slopless State

- Repo: `/Users/tartakovsky/Projects/agent-quality-controls/slopless`
- Branch checked: `development`
- Current validation script:
  - `npm run validate`
  - Runs `g3ts validate --path . --rules-only`
  - Does not run `g3ts validate-repo --path .`

## Commands Read

```bash
g3ts --help
g3ts validate --help
g3ts validate-repo --help
g3ts validate-repo --path .
g3ts validate --path . --family hooks --rules-only
```

## Current G3TS CLI Surface

- `g3ts validate --path <PATH>`
  - Validates one TypeScript package root.
  - Supports family filters.
  - Supports `--staged`.
  - Supports `--rules-only`.

- `g3ts validate-repo --path <PATH>`
  - Validates repository-level invariants.
  - Help says it covers hooks, tools, topology, and marker pairs.
  - Help does not define the required hook script content.

## Current Slopless Failures

`g3ts validate-repo --path .` reports:

```text
== hooks ==
[Error] g3ts-hooks/pre-commit-exists - pre-commit hook is missing
  TypeScript projects must have a selected pre-commit hook. Configure `git config core.hooksPath .githooks` and create `.githooks/pre-commit`.
[Error] g3ts-hooks/hooks-path-configured - git hooks path is not .githooks
  Git must use the repo-owned hook directory: run `git config core.hooksPath .githooks`. Other hook locations can bypass G3TS without changing repo files.
```

Current local Git state:

- `git config --get core.hooksPath` returns nothing.
- `.githooks/pre-commit` does not exist.
- `.githooks` does not exist.

## Required Slopless Fix After Guardrail3 Is Ready

- Add `.githooks/pre-commit`.
- Make `.githooks/pre-commit` executable.
- Configure local Git:

```bash
git config core.hooksPath .githooks
```

- Add repo-level G3TS validation to the normal validation path.
- Keep package-level validation in the normal validation path.

## Hook Contract Found In Guardrail3 Source

The hook cannot be a simple `npm run validate` wrapper.

Guardrail3 source currently requires `.githooks/pre-commit` to:

- Invoke `g3ts validate-repo`.
- Discover staged files with:

```bash
git diff --cached --name-only --diff-filter=ACM
```

- Discover adopted TypeScript units by walking upward from each staged file.
- Treat a directory as an adopted TypeScript unit only when it has both:
  - `package.json`
  - `guardrail3-ts.toml`
- Deduplicate discovered units.
- Invoke:

```bash
g3ts validate --path <unit> --staged
```

- Silently skip staged files with no owning adopted TypeScript unit.
- Avoid direct TypeScript toolchain commands in the hook.
- Scan staged files for merge conflict markers.
- Run:

```bash
gitleaks protect --staged
```

- Enforce a staged-file size cap with `git cat-file -s` or equivalent.
- Enforce lockfile integrity when `package.json` is staged.
- Enforce drizzle migration consistency by checking `drizzle/` and `db/schema/` paths.

## Guardrail3 Issues To Fix First

- `g3ts validate-repo --help` does not explain the required hook contract.
- `g3ts validate-repo --help` does not mention the staged-file routing algorithm.
- `g3ts validate-repo --help` does not mention the required marker pair:
  - `package.json`
  - `guardrail3-ts.toml`
- The available docs are stale or mismatched:
  - `docs/cli.md` documents `g3rs`, not `g3ts`.
  - `docs/cli.md` says `g3rs validate-repo` is deleted, while current `g3ts validate-repo` exists.
  - `GUARDRAIL3_GUIDE.md` documents old `guardrail3 ts hooks-install`, but current `g3ts` has no install or init command.
- There is no current `g3ts init` or `g3ts hooks-install` command to generate the required hook.
- It is unclear whether G3TS expects one monolithic `.githooks/pre-commit` or a managed `.githooks/pre-commit.d/g3ts` chain.
- It is unclear whether `gitleaks` must be declared as a repo dependency, installed globally, or only present on developer machines.
- It is unclear why every TypeScript repo must include drizzle migration consistency logic when the repo has no drizzle migrations.
- The first `validate-repo` run reports only missing hook setup. More hook-source failures will appear only after a basic hook exists.

## Proposed Guardrail3 Fixes

- Add current G3TS hook contract docs.
- Add a current G3TS repo setup command or generator.
- Make `validate-repo` output link to the setup command or exact required files.
- Decide whether drizzle migration checks are universal or conditional.
- Decide how required external tools such as `gitleaks` are declared and installed.
- Decide whether repo validation belongs in:
  - local pre-commit only,
  - `npm run validate`,
  - CI,
  - or all three.

## Proposed Slopless Implementation After Guardrail3 Fixes

- Use the canonical generated hook if Guardrail3 gains one.
- If no generator exists, write a hook that matches the documented G3TS contract exactly.
- Add `g3ts validate-repo --path .` to the Slopless validation path.
- Keep `g3ts validate --path . --rules-only` or replace it with the canonical full package validation once Guardrail3 docs clarify expected usage.
- Run:

```bash
g3ts validate-repo --path .
g3ts validate --path . --family hooks --rules-only
npm run validate
```

## Do Not Do Yet

- Do not add Slopless `.githooks/pre-commit` before Guardrail3 has a stable documented hook contract.
- Do not add a hand-written hook that only satisfies current string checks.
- Do not add drizzle-specific logic to Slopless until Guardrail3 decides whether that check is conditional.
- Do not add `gitleaks` wiring until Guardrail3 defines how required hook tools are installed or declared.
