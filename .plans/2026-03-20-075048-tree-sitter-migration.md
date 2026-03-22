# Tree-sitter-javascript Migration for ESLint Config Parsing

**Date:** 2026-03-20 07:50
**Task:** Plan the migration from `.contains()` string matching to tree-sitter-javascript AST parsing for all ESLint config checks.

---

## 1. What tree-sitter-javascript gives us

tree-sitter-javascript parses `.mjs` files into a full concrete syntax tree. For an ESLint flat config like:

```js
export default tseslint.config(
  ...tseslint.configs.strictTypeChecked,
  { rules: { "no-console": "error", complexity: ["error", { max: 25 }] } },
  { files: ["**/*.test.ts"], rules: { "max-lines": "off" } }
);
```

The AST gives us these node types:

### Import statements
- `import_statement` nodes with `source` child (string literal) and `import_clause` (named/default bindings)
- Example: `import boundaries from "eslint-plugin-boundaries"` -> `import_statement { import_clause: { identifier: "boundaries" }, source: "eslint-plugin-boundaries" }`
- **YES: We can extract all plugin imports and their local binding names.**

### Rule names and values
- Rules live inside `property` nodes within `object` nodes:
  - `pair` node: `key` (string or identifier) + `value` (string, array, or object)
  - `"no-console": "error"` -> `pair { key: string("no-console"), value: string("error") }`
  - `complexity: ["error", { max: 25 }]` -> `pair { key: identifier("complexity"), value: array [string("error"), object { pair { key: identifier("max"), value: number(25) } }] }`
- **YES: We can extract rule names, their severity (error/warn/off), and numeric option values.**

### Spread expressions
- `...tseslint.configs.strictTypeChecked` -> `spread_element { member_expression { object: member_expression { object: "tseslint", property: "configs" }, property: "strictTypeChecked" } }`
- **YES: We can detect preset spreads by walking `spread_element` nodes and reading the dotted member expression chain.**

### String literals inside object properties
- All string values are `string` nodes with their quotes included in the text. We can strip quotes to get the raw value.
- **YES: We can extract any string literal value from any position in the AST.**

### Config block identification (main vs test override vs .mjs override)
- Each argument to `tseslint.config(...)` is a separate config block. Some are spread elements (presets), some are object expressions (rule blocks).
- Object blocks with a `files` property can be identified as override blocks. The `files` array values tell us the scope:
  - `["**/*.test.ts", "**/*.spec.ts"]` -> test override
  - `["**/*.mjs"]` -> .mjs override
  - `["**/*.ts", "**/*.tsx"]` -> main TS rules
- **YES: We can determine which config block a rule belongs to by walking up to the enclosing object and checking its `files` property.**

### What tree-sitter-javascript does NOT give us
- **Runtime semantics.** Spread operators like `...BANNED_PATHS` reference a variable defined elsewhere in the file. tree-sitter gives us the AST but doesn't evaluate expressions. We can resolve simple cases (variable defined as array literal in the same file) but not computed values.
- **Plugin preset contents.** `...tseslint.configs.strictTypeChecked` spreads in ~53 rules at runtime. We can detect the spread exists but can't know which rules it contains without hardcoding that knowledge.
- Neither of these is a problem for our checks -- we only need to verify what the config file explicitly declares, not what it resolves to at runtime.

---

## 2. Right architecture: Parse once, query many times (Option A)

**Option (a) is correct.** Parse the `.mjs` file once into a structured `EslintConfig`, then have all check functions query the struct.

### Why not per-check AST walks (option b)?
- 35+ check functions would each re-parse the same file and walk the same tree. Wasteful.
- The checks are all asking variations of the same question: "is rule X present with value Y?" A pre-extracted HashMap answers this in O(1).
- The structured approach also enables new checks that the string approach couldn't support: "is this rule in the test override block?" or "what's the numeric value of `max` in `complexity`?"

### Proposed data model

```rust
/// Severity of an ESLint rule as configured.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EslintSeverity {
    Off,
    Warn,
    Error,
}

/// A single ESLint rule's configuration.
#[derive(Debug, Clone)]
pub struct EslintRuleConfig {
    /// "error", "warn", "off" (or numeric 0/1/2)
    pub severity: EslintSeverity,
    /// Numeric option values found in the rule config (e.g., max: 25)
    pub numeric_options: BTreeMap<String, u64>,
    /// The line number where this rule appears (1-based)
    pub line: usize,
    /// Raw value text for non-standard checks
    pub raw_value: String,
}

/// A config block — one object argument to tseslint.config(...)
#[derive(Debug, Clone)]
pub struct ConfigBlock {
    /// File patterns this block applies to (empty = global)
    pub files: Vec<String>,
    /// File patterns this block ignores
    pub ignores: Vec<String>,
    /// Rules defined in this block: rule_name -> config
    pub rules: BTreeMap<String, EslintRuleConfig>,
    /// Line number where this block starts
    pub line: usize,
}

/// Parsed ESLint flat config.
#[derive(Debug, Clone)]
pub struct EslintConfig {
    /// Plugin imports: package_name -> local_binding_name
    /// e.g., "eslint-plugin-boundaries" -> "boundaries"
    pub imports: BTreeMap<String, String>,

    /// Preset spreads found (e.g., "tseslint.configs.strictTypeChecked")
    pub presets: Vec<String>,

    /// All config blocks in order
    pub blocks: Vec<ConfigBlock>,

    /// Flattened view: ALL rules across all blocks (last-wins for duplicates)
    /// This is the primary query target for simple presence/value checks.
    pub all_rules: BTreeMap<String, EslintRuleConfig>,

    /// Content-related markers found (for checks like T6 boundary enforcement,
    /// T50 route wrappers, T51 process.env ban) that aren't simple rule lookups
    pub markers: ConfigMarkers,
}

/// Non-rule config features that checks need to verify.
#[derive(Debug, Clone, Default)]
pub struct ConfigMarkers {
    /// Whether eslint-plugin-boundaries is imported
    pub has_boundaries_plugin: bool,
    /// Whether boundaries/element-types rule exists
    pub has_element_types: bool,
    /// Whether boundaries/entry-point rule exists
    pub has_entry_point: bool,
    /// Whether boundaries/external rule exists
    pub has_external_deps: bool,
    /// Whether process.env appears in a no-restricted-syntax selector
    pub has_process_env_ban: bool,
    /// Whether withBody/withRoute appears in restricted syntax
    pub has_route_wrappers: bool,
    /// Whether RegExp appears in no-restricted-globals/no-restricted-syntax
    pub has_regexp_ban: bool,
    /// Whether test file override blocks exist
    pub has_test_overrides: bool,
    /// Lines with "off" or "warn" severity (for T7 inventory)
    pub relaxed_rules: Vec<(String, EslintSeverity, usize)>, // (rule_name, severity, line)
    /// File override blocks (for T8 inventory)
    pub file_overrides: Vec<(Vec<String>, usize)>, // (file_patterns, line)
}
```

### Parser function

```rust
/// Parse an eslint.config.mjs file into structured config data.
///
/// Uses tree-sitter-javascript to parse the AST, then extracts:
/// - Import statements (plugin names + bindings)
/// - Preset spreads (...tseslint.configs.X)
/// - Config blocks with their file patterns and rules
///
/// Returns None if parsing fails.
pub fn parse_eslint_config(content: &str) -> Option<EslintConfig>
```

### Where it lives

New file: `apps/guardrail3/src/app/ts/validate/eslint_parser.rs`

This follows the existing pattern where `ast_helpers.rs` provides tree-sitter infrastructure and domain-specific parsers live in their own files.

---

## 3. Helper functions needed

### Core parser (new file: `eslint_parser.rs`)

```rust
// Public API
pub fn parse_eslint_config(content: &str) -> Option<EslintConfig>

// Internal helpers
fn parse_javascript(source: &str) -> Option<Tree>        // tree-sitter-javascript parser
fn extract_imports(root: &Node, source: &[u8]) -> BTreeMap<String, String>
fn extract_config_call(root: &Node, source: &[u8]) -> Option<Node>  // find tseslint.config(...) or export default [...]
fn extract_config_blocks(call_node: &Node, source: &[u8]) -> Vec<ConfigBlock>
fn extract_presets(call_node: &Node, source: &[u8]) -> Vec<String>
fn extract_rules_from_object(obj: &Node, source: &[u8]) -> BTreeMap<String, EslintRuleConfig>
fn extract_file_patterns(obj: &Node, source: &[u8]) -> Vec<String>
fn parse_rule_value(value_node: &Node, source: &[u8]) -> EslintRuleConfig
fn parse_severity(s: &str) -> Option<EslintSeverity>     // "error"|"warn"|"off"|0|1|2
fn extract_numeric_options(node: &Node, source: &[u8]) -> BTreeMap<String, u64>
fn build_markers(config: &mut EslintConfig)               // derive markers from parsed rules + imports
fn node_text<'a>(node: &Node, source: &'a [u8]) -> &'a str
fn unquote(s: &str) -> &str                               // strip surrounding quotes from string literals
```

### New dependency

Add to `apps/guardrail3/Cargo.toml`:
```toml
tree-sitter-javascript = "0.23"  # same version series as tree-sitter-typescript
```

The `tree-sitter` core crate is already at 0.26, and `tree-sitter-javascript` 0.23 should be compatible (same as `tree-sitter-typescript` 0.23).

---

## 4. Which checks change and how

### Files that change

#### `eslint_rule_infra.rs` — **GUTTED**

Current: `check_eslint_rule` and `check_eslint_rule_presence` use `content.contains(rule_name)`. `check_rule_value` does line-by-line number extraction.

New: All three functions take `&EslintConfig` instead of `&str`. Rule presence becomes `config.all_rules.contains_key(rule_name)`. Value checking becomes `config.all_rules.get(rule_name).map(|r| r.numeric_options.get("max"))`.

Functions that change:
- `check_eslint_rule(config: &EslintConfig, ...)` — lookup in `config.all_rules`
- `check_eslint_rule_presence(config: &EslintConfig, ...)` — lookup in `config.all_rules`
- `check_rule_value` — replaced by direct struct field access
- `extract_number_from_line` — **DELETED** (no longer needed)
- `eslint_rule_explanation` — **UNCHANGED** (pure data, no parsing)

#### `eslint_check.rs` — **REWRITTEN to use EslintConfig**

Every function changes from `(content: &str, ...)` to `(config: &EslintConfig, ...)`.

| Function | Current | New |
|---|---|---|
| `check_eslint_config` | reads file, passes `&str` to sub-checks | reads file, calls `parse_eslint_config`, passes `&EslintConfig` to sub-checks |
| `check_eslint_value_rules` (T2-T5) | `check_eslint_rule(content, ...)` | `check_eslint_rule(config, ...)` |
| `check_boundary_enforcement` (T6) | `content.contains("boundaries")` | `config.markers.has_boundaries_plugin` |
| `check_eslint_presets` (T-ESLP-13/14) | `content.contains("strictTypeChecked")` | `config.presets.iter().any(\|p\| p.contains("strictTypeChecked"))` |
| `check_regex_ban` (T-ESLP-15) | `content.contains("RegExp") && content.contains("no-restricted")` | `config.markers.has_regexp_ban` |
| `check_relaxed_rules` (T7) | line-by-line `trimmed.contains("\"off\"")` | `config.markers.relaxed_rules.iter()` |
| `check_file_overrides` (T8) | line-by-line `trimmed.contains("files:")` | `config.markers.file_overrides.iter()` |
| `check_rule_presence_t40_t48` (T40-T48) | `check_eslint_rule_presence(content, ...)` | `check_eslint_rule_presence(config, ...)` |
| `check_all_eslint_rules` (T60-T83) | `check_eslint_rule_presence(content, ...)` | `check_eslint_rule_presence(config, ...)` |
| `check_test_relaxations` (T49) | line-by-line `test` + `files` | `config.blocks.iter().filter(\|b\| b.files.iter().any(\|f\| f.contains("test") \|\| f.contains("spec")))` |
| `check_route_wrappers` (T50) | `content.contains("withBody")` | `config.markers.has_route_wrappers` |
| `check_process_env_ban` (T51) | `content.contains("process.env")` | `config.markers.has_process_env_ban` |

#### `eslint_audit.rs` — **REWRITTEN to use EslintConfig**

| Function | Current | New |
|---|---|---|
| `check` | reads file, passes `&str` | reads file, calls `parse_eslint_config`, passes `&EslintConfig` |
| `check_zone_definitions` (T36) | `content.contains("element-types")` | `config.markers.has_element_types` |
| `check_import_direction` (T37) | `content.contains("boundaries/element-types")` | `config.all_rules.contains_key("boundaries/element-types")` |
| `check_entry_point` (T38) | `content.contains("boundaries/entry-point")` | `config.all_rules.contains_key("boundaries/entry-point")` |
| `check_external_deps` (T39) | `content.contains("boundaries/external")` | `config.all_rules.contains_key("boundaries/external")` |

**Optimization:** `eslint_audit.rs` currently re-reads the file and re-checks existence. After migration, `check_eslint_config` in `eslint_check.rs` should parse once and pass the `EslintConfig` to both `eslint_check` functions AND `eslint_audit` functions. The audit module's `check()` function should accept `&EslintConfig` instead of `&dyn FileSystem + &Path`.

#### `eslint_plugin_checks.rs` — **REWRITTEN to use EslintConfig**

| Function | Current | New |
|---|---|---|
| `find_missing_rules` | `content.contains(**rule)` | `config.all_rules.contains_key(*rule)` |
| `check_config_import` | `markers.iter().all(\|m\| content.contains(m))` | check `config.imports` for package name + `config.presets` for config pattern |
| `check_unicorn_import` (T-ESLP-01) | `content.contains("unicorn")` + `content.contains("flat/recommended")` | `config.imports.contains_key("eslint-plugin-unicorn")` + preset check |
| `check_core_plugins` (T-ESLP-02..11) | all via `find_missing_rules` on `&str` | all via `find_missing_rules` on `&EslintConfig` |
| `check_content_plugins` (T-ESLP-07/08/12) | `content.contains(...)` | struct field lookups |
| `check_test_relaxations` (T-ESLP-11) | line-by-line test pattern matching | check `config.blocks` for test override blocks with the expected relaxation rules |
| `check_tailwind_ban` (T-ESLP-12) | `content.contains("tailwind-ban")` | `config.imports` or `config.all_rules` lookup |
| naming-convention selector check | `content.contains("selector")` | check rule's raw value or numeric_options for selector config |
| jsx-no-leaked-render validStrategies check | `content.contains("validStrategies")` | check rule's raw value for validStrategies |

#### `jscpd_check.rs` — **ONE FUNCTION CHANGES**

| Function | Current | New |
|---|---|---|
| `check_content_import_restriction` (T60) | `content.contains("content/")` on ESLint config | `config.all_rules.get("no-restricted-imports")` and check its raw value for content/ patterns |

**Note:** This function is in `jscpd_check.rs` but checks ESLint config. It should probably move to `eslint_check.rs` during this migration, but that's a separate concern.

### Signature change cascade

The key signature change is:

```
// OLD
fn check_eslint_rule(content: &str, eslint_path: &Path, id, rule_name, expected_value, severity, results)
fn check_eslint_rule_presence(content: &str, eslint_path: &Path, id, rule_name, severity, results)

// NEW
fn check_eslint_rule(config: &EslintConfig, eslint_path: &Path, id, rule_name, expected_value, severity, results)
fn check_eslint_rule_presence(config: &EslintConfig, eslint_path: &Path, id, rule_name, severity, results)
```

All callers update from passing `&content` to `&config`. The orchestrator in `eslint_check.rs::check_eslint_config` parses once and passes the struct down.

### New call flow

```
check_eslint_config(fs, eslint_configs, root, results)
  |
  +-- for each eslint_path:
  |     content = fs.read_file(eslint_path)
  |     config = parse_eslint_config(&content)?     // NEW: parse once
  |     |
  |     +-- check_eslint_value_rules(&config, ...)   // was: (&content, ...)
  |     +-- check_boundary_enforcement(&config, ...) // was: (&content, ...)
  |     +-- check_eslint_presets(&config, ...)
  |     +-- check_regex_ban(&config, ...)
  |     +-- check_relaxed_rules(&config, ...)
  |     +-- check_file_overrides(&config, ...)
  |     +-- check_rule_presence_t40_t48(&config, ...)
  |     +-- check_all_eslint_rules(&config, ...)
  |     +-- check_test_relaxations(&config, ...)
  |     +-- check_route_wrappers(&config, ...)
  |     +-- check_process_env_ban(&config, ...)
  |     |
  |     +-- eslint_audit checks:
  |     |   +-- check_zone_definitions(&config, ...)
  |     |   +-- check_import_direction(&config, ...)
  |     |   +-- check_entry_point(&config, ...)
  |     |   +-- check_external_deps(&config, ...)
  |     |
  |     +-- eslint_plugin checks:
  |         +-- check_core_plugins(&config, ...)
  |         +-- check_content_plugins(&config, ...)
```

---

## 5. TS import boundaries (`ts_arch_checks.rs`) — should it switch to tree-sitter?

### Current state

`check_file_imports` uses line-by-line string matching:
1. Skip lines starting with `//`, `*`, `/*` (comment heuristic)
2. `extract_import_path(trimmed)` which does `line.find("from '")` etc.
3. Resolve relative paths, determine source/target layers, check boundaries

### Problems with current approach
1. **Comment skipping misses multi-line comments.** A block comment `/* ... import from '../adapters/...' ... */` would be processed as a real import.
2. **Dynamic imports missed.** `await import('../adapters/db')` is not caught.
3. **Re-exports missed.** `export { foo } from '../adapters/db'` uses `from` but isn't an `import` statement per se (though `extract_import_path` would catch it since it looks for `from '`).
4. **Template literal false positives.** A template string containing `from '...'` would match.

### Recommendation: YES, migrate to tree-sitter

The file already has tree-sitter-typescript available (used by `ast_helpers.rs` and `ts_code_analysis.rs` in the same module). The migration is straightforward:

```rust
// NEW: tree-sitter-based import extraction
pub fn extract_imports_from_ast(tree: &Tree, source: &str) -> Vec<ImportInfo> {
    // Walk tree, find all import_statement and export_statement nodes
    // Extract source (the string literal after "from")
    // Return (line_number, import_path) pairs
}

pub struct ImportInfo {
    pub line: usize,        // 1-based
    pub path: String,       // the import path string
    pub is_dynamic: bool,   // await import(...) vs static import
}
```

Then `check_file_imports` becomes:

```rust
pub fn check_file_imports(file_path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let is_tsx = file_path.extension().is_some_and(|e| e == "tsx");
    let Some(tree) = parse_ts_file(content, is_tsx) else { return; };
    let imports = extract_imports_from_ast(&tree, content);
    // ... same layer checking logic, but on ImportInfo instead of raw lines
}
```

This eliminates all four problems above. The layer-resolution logic (`layer_from_import`, `layer_from_path`, `resolve_relative`) stays unchanged -- it operates on import path strings, which we'd still extract (just from AST nodes instead of line parsing).

### Should this be part of the same migration?

**Separate task.** The ESLint config migration is a cohesive unit (one parser, one struct, many consumers). The import boundary migration is independent (different file, different tree-sitter language, different concerns). Do ESLint first, imports second.

---

## 6. Implementation phases

### Phase 1: Parser + dependency (no behavior change)
1. Add `tree-sitter-javascript` to `Cargo.toml`
2. Create `eslint_parser.rs` with `parse_eslint_config` + all internal helpers
3. Add comprehensive unit tests for the parser against real eslint.config.mjs content
4. **Zero changes to existing check files** -- the parser is standalone

### Phase 2: Migrate eslint_rule_infra.rs
1. Change `check_eslint_rule` and `check_eslint_rule_presence` signatures from `&str` to `&EslintConfig`
2. Rewrite internals to use struct lookups
3. Delete `check_rule_value` and `extract_number_from_line`
4. Update all callers (compilation will force this)

### Phase 3: Migrate eslint_check.rs
1. Add `parse_eslint_config` call in `check_eslint_config`
2. Update all sub-check functions to take `&EslintConfig`
3. Update `check_relaxed_rules`, `check_file_overrides`, `check_test_relaxations` to use struct data instead of line-by-line

### Phase 4: Migrate eslint_audit.rs
1. Remove the standalone `check()` function that re-reads the file
2. Make audit check functions accept `&EslintConfig`
3. Wire into the single parse in `eslint_check.rs`

### Phase 5: Migrate eslint_plugin_checks.rs
1. Change `find_missing_rules` to take `&EslintConfig`
2. Update all plugin check functions
3. Fix `check_config_import` to use `config.imports`

### Phase 6: Adversarial tests
1. ESLint config with rule names in comments (should NOT be detected)
2. ESLint config with rule names in string literals that aren't rule definitions
3. ESLint config with rules in test override blocks (should be correctly attributed)
4. Malformed/partial configs (should return None or partial results, not crash)
5. Empty config file
6. Config that uses `require()` instead of `import` (CJS compat)

### Phase 7 (separate): TS import boundary migration
1. Add `extract_imports_from_ast` to `ast_helpers.rs` or `ts_code_analysis.rs`
2. Rewrite `check_file_imports` in `ts_arch_checks.rs`
3. Delete `extract_import_path` and `extract_between_after`
4. Add adversarial tests (imports in comments, template literals, dynamic imports)

---

## 7. Risk analysis

### Low risk
- **tree-sitter-javascript compatibility.** Same version series (0.23) as tree-sitter-typescript, same tree-sitter core (0.26). Should work out of the box.
- **Parser correctness for rule extraction.** ESLint flat config has a predictable structure. The AST makes it unambiguous.

### Medium risk
- **Variable references in config.** If a config does `const myRules = { ... }; export default tseslint.config({ rules: myRules })`, the parser would need to resolve the variable. Current string matching doesn't have this problem (it matches anywhere in the file). The parser needs to handle this case by either:
  - Walking the full file for object literals assigned to variables, then resolving references in the config call
  - OR falling back to a simpler approach: extract ALL `pair` nodes from ALL object literals in the file, not just those inside `tseslint.config()`

  **Recommendation:** Start with the strict approach (only rules inside `tseslint.config(...)` arguments). If real configs use variable indirection, add variable resolution later. The real eslint.config.mjs from the websmasher project inlines everything, so this is likely not an immediate issue.

### High risk (but contained)
- **Spread elements that reference variables.** `...BANNED_PATHS` in the config expands a variable. The parser can't evaluate this. For the purpose of guardrail checks, this is fine -- we're checking rule names, not banned path lists. The spread contents (banned paths) are not something guardrail3 validates rule-by-rule.

---

## 8. Files to modify

| File | Change type | Size estimate |
|---|---|---|
| `apps/guardrail3/Cargo.toml` | Add dependency | 1 line |
| `apps/guardrail3/src/app/ts/validate/eslint_parser.rs` | **NEW** | ~300-400 lines |
| `apps/guardrail3/src/app/ts/validate/mod.rs` | Add `mod eslint_parser` | 1 line |
| `apps/guardrail3/src/app/ts/validate/eslint_rule_infra.rs` | Rewrite internals | ~150 lines changed |
| `apps/guardrail3/src/app/ts/validate/eslint_check.rs` | Rewrite to use struct | ~200 lines changed |
| `apps/guardrail3/src/app/ts/validate/eslint_audit.rs` | Rewrite to use struct | ~100 lines changed |
| `apps/guardrail3/src/app/ts/validate/eslint_plugin_checks.rs` | Rewrite to use struct | ~200 lines changed |
| `apps/guardrail3/src/app/ts/validate/jscpd_check.rs` | 1 function change | ~10 lines changed |
| Test files for all above | Update/expand | ~300 lines |

Total: ~1 new file, ~7 modified files, ~1200-1400 lines changed.
