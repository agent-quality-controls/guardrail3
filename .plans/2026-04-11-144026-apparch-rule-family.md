# Plan: apparch rule family

## Goal

New guardrail3 architecture check family for application services. Enforces a 3-layer + direction-split architecture. Layer placement is mechanically determinable from code properties.

## Architecture

```
types/              # structs, enums, traits, errors. Depends on nothing.
logic/              # all business logic: pure rules, workflows, orchestration. Depends on types only.
io/
  inbound/          # entry points + composition root (HTTP, CLI, gRPC). Depends on types, logic, io/outbound.
  outbound/         # I/O implementations (DB, APIs, fs). Depends on types only.
```

### Design rationale

Started from hexagonal architecture (domain/ports/app/adapters) which has three ambiguous interior layers requiring semantic judgment for placement. Collapsed through several iterations:

- **ports merged into types**: trait contracts are type-level definitions. Splitting them from the structs they reference added a layer with no mechanical distinction. Trait placement is self-enforcing: if io/outbound needs to implement it, it must be in types/ (only layer io/outbound can see).
- **domain's types merged into types, domain's logic merged into logic**: hexarch's domain contained both data definitions and business rules. These are mechanically different (definitions vs computation) and belong in separate layers.
- **orchestration merged into logic**: the distinction between "pure business rule" and "workflow that coordinates via trait objects" is semantic, not mechanical. Both depend only on types/. Both are pure. The `dyn Trait` parameter doesn't change the dependency - the trait is defined in types/. Splitting them recreates the domain/app ambiguity from hexarch.
- **io/inbound depends on io/outbound**: io/inbound is the composition root. It creates concrete io/outbound implementations, wraps them as trait objects, and passes them into logic/. Without this, wiring lives outside the architecture in an unregulated escape hatch.

### Trade-off acknowledged

`logic/` will contain ~80% of application code: validation rules, business computations, workflows, orchestration. Internal structure within logic/ (by feature, by abstraction level) is conventional, not enforced. The architecture prevents cross-layer spaghetti (logic can't touch I/O), but not intra-layer spaghetti within logic/.

This is accepted because every mechanical split of logic/'s interior we explored recreated hexarch's semantic ambiguity.

### Layer detection

Path-based, same mechanism as hexarch. A crate's layer is determined by directory segment:

| Segment in path   | Layer              |
|--------------------|--------------------|
| `types/`           | Types              |
| `logic/`           | Logic              |
| `io/inbound/`      | IoInbound          |
| `io/outbound/`     | IoOutbound         |
| none of the above  | None (unclassified)|

Unclassified crates (e.g. standalone binary entry points) are not checked by dependency rules.

### Dependency matrix

```
              types  logic  io/in  io/out
types           -     NO     NO     NO
logic          YES     -     NO     NO
io/inbound     YES   YES      -    YES
io/outbound    YES    NO     NO      -
```

No cycles. types/ is the center, depends on nothing. logic/ depends on types/ only. io/outbound depends on types/ only. io/inbound depends on everything (it's the outermost layer and the composition root).

### Placement test (for agents and humans)

- Is it a data definition, trait contract, or error type? -> `types/`
- Does it compute, decide, validate, or orchestrate using types? -> `logic/`
- Does it implement traits from types/ with real I/O? -> `io/outbound/`
- Does it receive external input, wire implementations, and trigger logic? -> `io/inbound/`

## Rules

### Dependency direction (Cargo.toml analysis)

Check `[dependencies]` in each crate's Cargo.toml against the layer of every workspace-internal dependency crate.

| ID             | Layer           | Violation                                                     | Severity |
|----------------|-----------------|---------------------------------------------------------------|----------|
| RS-APPARCH-01  | types/          | Depends on logic or io crate                                  | Error    |
| RS-APPARCH-02  | logic/          | Depends on io crate                                           | Error    |
| RS-APPARCH-03  | io/outbound     | Depends on logic or io/inbound crate                          | Error    |

Inventory (Info) finding emitted per crate when all dependencies conform.

Note: no rule for io/inbound - it can depend on anything. It's the outermost layer.

### Source constraints (AST analysis)

| ID             | Layer           | Constraint                                                     | Severity |
|----------------|-----------------|----------------------------------------------------------------|----------|
| RS-APPARCH-04  | io/*            | Must not define public traits                                  | Error    |

Traits are contracts. Contracts live in types/. io/ implements them, doesn't define them.

### Potential future rules

- RS-APPARCH-05: types/ public free functions limited to constructors/conversions (prevent logic creep into types). Needs clear mechanical definition of "constructor."
- RS-APPARCH-06: io/ crates must have at least one I/O dependency in Cargo.toml (prevent pure code hiding in io/).
- RS-APPARCH-07: logic/ crates must not have I/O dependencies in Cargo.toml (enforce purity). Requires a known-I/O-crate list or heuristic.

## Packages

Following existing hexarch package structure:

```
packages/rs/apparch/
  g3rs-apparch-types/                    # Layer enum, crate facts structs, input types
  g3rs-apparch-ingestion/                # Workspace crawl -> per-crate facts + dependency edges
    crates/
      types/                             # Ingestion input/output/error types
      runtime/                           # Ingestion logic
  g3rs-apparch-dep-checks/               # RS-APPARCH-01 through 03 (dependency direction)
    crates/
      types/                             # Check input types
      runtime/                           # Check logic
  g3rs-apparch-source-checks/            # RS-APPARCH-04 (source constraints)
    crates/
      types/                             # Check input types
      runtime/                           # Check logic
```

Family integration:

```
apps/guardrail3/crates/app/rs/families/apparch/
  crates/
    runtime/                             # Wires ingestion -> checks, emits results
    assertions/                          # Integration test assertions
  test_support/                          # Shared test fixtures
```

## Ingestion requirements

### Reusable from hexarch/workspace-crawl
- Workspace root discovery (Cargo.toml with [workspace])
- Workspace member resolution (glob patterns, excludes)
- Crate entrypoint discovery (lib.rs, main.rs, custom paths)
- Module tree walking via syn AST
- Public surface metrics (trait count, free fn count, inherent method count)

### New for apparch
- **Dependency edge extraction**: for each member crate, parse Cargo.toml [dependencies] and resolve which workspace-internal crates it depends on. Map each dependency to its layer. Needed for RS-APPARCH-01 through 03.
- **Layer detection**: same mechanism as hexarch but with 4 layers (Types, Logic, IoInbound, IoOutbound) and different path segments.

The arch family already has dependency edge extraction (`facts/dependency_edges.rs`). Evaluate whether it can be reused.

## Key decisions

1. **Dependency checks via Cargo.toml, not `use` statements.** Cargo.toml is the source of truth for crate dependencies. `use` statements can alias, re-export, and glob-import in ways that make them unreliable for layer detection.

2. **No orchestration layer.** Orchestration is semantically identical to business logic from a dependency perspective. Splitting them recreates hexarch's domain/app ambiguity. All pure computation lives in logic/.

3. **io/inbound is the composition root.** It creates io/outbound implementations, wraps them as trait objects, and passes them into logic/. No code lives outside the architecture.

4. **Only 4 enforceable rules.** 3 dependency direction rules + 1 source constraint. Minimal rule set that enforces the critical boundaries (pure vs I/O, contract ownership). Internal organization within layers is conventional.

5. **Family is opt-in, coexists with hexarch.** A workspace uses apparch OR hexarch, never both. Selection mechanism TBD (likely Cargo.toml metadata or guardrail3 config).
