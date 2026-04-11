# g3rs Current Architecture

**Status:** active working template

This file is the current architecture note for the new `g3rs` pipeline.

It is meant to replace stale scattered assumptions with one simple template.

## Pipeline

```text
workspace root
  -> g3rs-workspace-crawl
  -> family ingestion package
  -> family checks package
  -> tiny rule functions
```

More explicitly:

```text
workspace root
  -> G3RsWorkspaceCrawl
  -> ingest_for_config_checks / ingest_for_source_checks / ingest_for_file_tree_checks
  -> family-level checks input
  -> checks runtime/support mapping
  -> rule-specific tiny inputs
  -> pure rules
```

## Layer Responsibilities

### 1. Workspace crawl

`g3rs-workspace-crawl` owns only neutral workspace snapshot behavior.

It knows:

- which paths exist
- which paths are ignored
- basic file kind / readability facts
- simple lookup by relative path

It does not know:

- family ownership
- rule semantics
- config policy
- AST semantics
- file-tree legality

### 2. Family ingestion package

Each family ingestion package is the mapper from workspace crawl to a checks-package input.

It owns:

- selecting the files its family needs
- reading the files it needs
- calling parser crates when parser output is part of the checks input
- family-level normalization needed to build the public checks input
- choosing scope for each checks lane

Each ingestion package exposes exactly three entry points:

- `ingest_for_config_checks`
- `ingest_for_source_checks`
- `ingest_for_file_tree_checks`

Each of those returns the input type for the matching checks package.

Examples:

- config ingestion may return one input for one config file
- deps config ingestion may return many per-crate inputs
- source ingestion may return one input per file, per crate, per root, or per package
- file-tree ingestion may return one input per structural scope

### 3. Family checks package runtime

The checks package runtime is allowed to do a second mapping pass.

It does **not** rediscover the workspace.

It only works from the input that ingestion handed to it.

It owns:

- parse-once work that is local to that checks input
- cross-file AST mapping inside that input scope
- support-layer indexes and derived facts
- fan-out into tiny rule-specific inputs

This applies to all checks lanes, not only AST.

In simple config families, this second pass may be tiny.
In AST families, it may be substantial.

### 4. Rule files

Rules are the end of the pipeline.

Each rule:

- is one production file
- is pure
- takes the smallest possible typed input
- does not read files
- does not parse files
- does not discover siblings
- does not know about the workspace crawl

## Checks Lanes

### Config lane

Typical shape:

- ingestion selects config files and parses them
- checks runtime may do small family-local normalization
- rules operate on minimal config facts

Config rules may be:

- single-file
- pair-wise
- small aggregate

But they should only receive the smallest surface they actually need.

### source lane

AST is **not** always one file.

The correct AST scope is the smallest one that matches the rule.

Valid AST scopes:

- one file
- one crate
- one root
- one assertions package
- another family-owned bounded scope

source ingestion chooses that scope.

source checks runtime then:

- parses once inside that scope
- builds cross-file maps if needed
- emits tiny rule-local facts

### File-tree lane

File-tree rules own structural and path-relationship questions.

Typical file-tree concerns:

- file existence
- path classification
- membership / placement
- workspace boundary questions
- local target structure

File-tree checks should not be smuggled into config rules just because the config mentions a path.

Example:

- `topology` rules about nested workspaces, exact workspace membership, member
  path escapes, and illegal placement of workspace-local family files are
  file-tree legality checks
- they are not `arch`
- they are not config/source checks

## Scope Rule

The important rule is:

**Use the smallest family-owned scope that matches the rule.**

Bad:

- whole repo AST by default
- rules re-walking all files
- checks runtime reaching back into crawl state it was not given

Good:

- one file for local syntax rules
- one root for garde boundary analysis
- one crate for hexarch public API shape
- one assertions package for test proof catalogs

## Current Gaps / Drift

This is the target architecture, but current code is not fully there yet.

Known drift:

- `g3rs-garde-source-checks` still reads source files itself from explicit file paths
- that works, but it is looser than the clean target boundary
- the file-tree lane is still mostly stubbed across the new package tree
- some current packages still reflect transitional contracts from the legacy app split

So this note is both:

- the current working template
- the cleanup target for packages that still carry transitional behavior

## Build Template For New Families

When building a new family lane:

1. decide the lane
   - config
   - AST
   - file-tree

2. decide the correct scope
   - file
   - crate
   - root
   - package

3. design one public checks input type for that scope

4. make ingestion own:
   - selection
   - reading
   - parser calls
   - boundary normalization up to the checks input

5. make checks runtime own:
   - parse-once work inside its input
   - support-layer mapping
   - rule fan-out

6. keep each rule tiny and pure

## Short Version

- crawl is neutral
- ingestion is the family mapper from crawl to checks input
- checks runtime is the lane-local mapper from checks input to tiny rule inputs
- rules are tiny pure functions
- AST may be single-file or multi-file
- scope must be the smallest bounded scope that matches the rule
