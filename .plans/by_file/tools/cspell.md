# cspell

## What it does
Spell checker for code and documentation.

## Config file
`cspell.json`, `.cspell.json`, `cspell.config.{json,yaml,yml,js,cjs,mjs}`, `.cspell.config.*` (11+ variants). `.cspell.json` takes priority over `cspell.json`.

## Config discovery (verified from cspell docs + research)
Walk-up from each FILE being checked. Nearest config wins. No automatic merging.

`--config <path>` bypasses walk-up entirely.
`--no-config-search` disables walk-up.
`--root` sets base directory for glob resolution (whether it limits config search upward is undocumented).

## Shadowing
YES. Subdirectory cspell.json shadows root for files in that subtree.

## `import` mechanism
Explicit — NOT auto-resolved. Subdirectory config must explicitly `import` parent to inherit:
```json
{ "import": ["../../cspell.json"], "words": ["myterm"] }
```
- Array settings (`words`, `dictionaries`): unioned on import
- Scalar settings: parent config takes precedence (importer is lower priority)
- Relative paths resolve relative to the importing config file

## How to invoke
```bash
pnpm exec cspell --no-progress --no-summary <files>
```
Can run on specific files (staged files in hook) or directories.

## Guardrail3's role
- **Generate:** Scaffold cspell.json on first run. For existing: preserve `words` array, ensure structural settings.
- **Validate:** Check cspell installed, config exists
- **Hook:** Run on staged files from root
- **Coverage map:** Show which files are covered by which cspell config (walk-up per file)
