# Garde Adapter Boundary Validation

Every adapter in a hexagonal architecture is a trust boundary. Data crossing that boundary — inbound requests, outbound API responses, file reads, message queue payloads — must be validated before entering the application layer. This document describes how to enforce that structurally using [Garde](https://github.com/jprochazk/garde) and Clippy's `disallowed_types` / `disallowed_methods`.

## The Problem

Serde deserialization only checks "is this valid JSON that matches the struct shape?" It does not check:
- Required fields being non-empty
- Strings being within expected lengths
- Numbers being within expected ranges
- URLs being syntactically valid
- Arrays having minimum/maximum lengths

A malformed request or garbage API response that happens to parse as valid JSON passes straight through to the application layer. The app then produces meaningless results or hits unexpected edge cases.

## The Pattern

### 1. Annotate all boundary types with `#[derive(garde::Validate)]`

Every struct that crosses an adapter boundary gets Garde validation rules:

```rust
use garde::Validate;

#[derive(Deserialize, Validate)]
#[garde(context(()))]  // or a custom context for dynamic constraints
struct CreateArticleRequest {
    #[garde(length(chars, min = 1, max = 200))]
    title: String,

    #[garde(length(chars, min = 1, max = 500))]
    description: String,

    #[garde(url)]
    base_url: String,

    #[garde(range(min = 1))]
    h2_count: usize,

    #[garde(length(min = 1))]  // at least one category
    categories: Vec<Category>,

    #[garde(dive)]  // recursively validate nested structs
    metadata: ArticleMetadata,
}
```

For dynamic constraints (values loaded from config/JSON at runtime), use a context struct:

```rust
#[derive(Deserialize, Validate)]
#[garde(context(ValidationConfig as ctx))]
struct ArticleRequest {
    #[garde(length(chars, min = ctx.title_min, max = ctx.title_max))]
    title: String,
}

struct ValidationConfig {
    title_min: usize,
    title_max: usize,
}

// At validation time:
let config = load_config()?;
request.validate(&config)?;
```

### 2. Inbound adapters: `ValidatedJson<T>` extractor

Replace raw `axum::Json<T>` with a custom extractor that deserializes AND validates:

```rust
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + garde::Validate<Context = ()>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum::Json(value) = axum::Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::BadRequest(e.body_text()))?;

        value.validate(&())
            .map_err(|e| AppError::ValidationFailed(e.to_string()))?;

        Ok(Self(value))
    }
}
```

**The trait bound `T: garde::Validate` is the key.** Any request type used with `ValidatedJson<T>` that doesn't derive `garde::Validate` is a compile error.

For context-based validation (dynamic constraints), use `State` to pass the context:

```rust
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + garde::Validate,
    T::Context: FromRef<S>,
    S: Send + Sync,
{
    // Extract context from app state, pass to validate()
}
```

### 3. Outbound adapters: `Validated<T>` wrapper

For data coming FROM external services (API responses, file reads), use a newtype that can only be constructed through validation:

```rust
/// A value that has been validated via Garde.
/// Can only be constructed through `Validated::new()`.
pub struct Validated<T>(T);

impl<T: garde::Validate> Validated<T> {
    pub fn new(value: T, ctx: &T::Context) -> Result<Self, garde::Report> {
        value.validate(ctx)?;
        Ok(Self(value))
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for Validated<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}
```

Port traits return `Validated<T>`:

```rust
pub trait LighthouseClient {
    async fn run_audit(&self, url: &str) -> Result<Validated<LighthouseReport>>;
}

pub trait PageFetcher {
    async fn fetch(&self, urls: &[String]) -> Result<Vec<Validated<FetchedPage>>>;
}
```

The adapter MUST validate before wrapping:

```rust
impl LighthouseClient for ReqwestLighthouseClient {
    async fn run_audit(&self, url: &str) -> Result<Validated<LighthouseReport>> {
        let response: LighthouseReport = self.client.get(url).send().await?.json().await?;
        Validated::new(response, &())  // validation happens here
    }
}
```

If the adapter tries to return a raw `LighthouseReport` without `Validated` — compile error (return type mismatch). If `LighthouseReport` doesn't derive `Validate` — compile error (trait bound unsatisfied).

## Enforcement: Banning Raw Types via Clippy

The trait bounds above enforce "IF you use the wrapper, the type must implement Validate." To enforce "you MUST use the wrapper" — ban the raw alternatives in `clippy.toml`.

### Workspace-level clippy.toml

Add to the workspace `clippy.toml` (applies to all crates):

```toml
# Adapter boundary enforcement — force validated wrappers
disallowed-types = [
    # ... existing bans (HashMap, Mutex, etc.) ...

    # Ban raw Axum JSON extractor — use ValidatedJson<T> instead
    { path = "axum::extract::Json", reason = "BANNED: Use ValidatedJson<T> instead. ValidatedJson requires your request type to #[derive(garde::Validate)], which enforces field-level validation (length, range, url, dive for nested types). Add reasonable garde rules to each field — e.g. #[garde(length(chars, min = 1, max = 500))] for strings, #[garde(range(min = 0))] for numbers, #[garde(url)] for URLs, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty collections. Use #[garde(skip)] only with a // reason comment for fields that genuinely need no validation." },
    { path = "axum::Json", reason = "BANNED: Use ValidatedJson<T> instead. ValidatedJson requires your request type to #[derive(garde::Validate)], which enforces field-level validation (length, range, url, dive for nested types). Add reasonable garde rules to each field — e.g. #[garde(length(chars, min = 1, max = 500))] for strings, #[garde(range(min = 0))] for numbers, #[garde(url)] for URLs, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty collections. Use #[garde(skip)] only with a // reason comment for fields that genuinely need no validation." },
]

disallowed-methods = [
    # ... existing bans (env::var, fs::write, etc.) ...

    # Ban raw serde deserialization — use Validated<T>::new() for external data
    { path = "serde_json::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field — #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new(). Exception: #[allow(clippy::disallowed_methods)] with a // reason comment is OK in build.rs, tests, and constraint loading only." },
    { path = "serde_json::from_slice", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field — #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new(). Exception: #[allow(clippy::disallowed_methods)] with a // reason comment is OK in build.rs, tests, and constraint loading only." },
    { path = "serde_json::from_value", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field — #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new(). Exception: #[allow(clippy::disallowed_methods)] with a // reason comment is OK in build.rs, tests, and constraint loading only." },
    { path = "serde_json::from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field — #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new(). Exception: #[allow(clippy::disallowed_methods)] with a // reason comment is OK in build.rs, tests, and constraint loading only." },
]
```

### Legitimate exceptions

Some uses of raw deserialization are legitimate:

- **`build.rs`** — embedding constraint JSON at compile time
- **Tests** — loading golden fixtures
- **Constraint loading at startup** — deserializing the constraints file itself (you can't validate constraints with constraints)

These get `#[allow(clippy::disallowed_methods)]` with a mandatory reason comment (already enforced by pre-commit hook):

```rust
#[allow(clippy::disallowed_methods)] // constraint loading at startup — no constraints to validate against
let constraints: ContentConstraints = serde_json::from_str(json)?;
```

## The Enforcement Chain

Each step produces an actionable error message that tells the agent exactly what to do next:

```
Agent writes new adapter
  ↓
Uses raw axum::Json<T>?
  → clippy error: "BANNED: Use ValidatedJson<T> instead. ValidatedJson requires
    your request type to #[derive(garde::Validate)], which enforces field-level
    validation. Add reasonable garde rules to each field — e.g.
    #[garde(length(chars, min = 1, max = 500))] for strings, #[garde(range(min = 0))]
    for numbers, #[garde(url)] for URLs, #[garde(dive)] for nested structs..."
  → Agent switches to ValidatedJson<T>

  ↓
Uses raw serde_json::from_str?
  → clippy error: "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add
    #[derive(garde::Validate)] to your type. (2) Add garde rules to each field.
    (3) Deserialize into raw type, then wrap with Validated::new()..."
  → Agent switches to Validated<T>::new()

  ↓
Uses ValidatedJson<NewType> but NewType missing #[derive(Validate)]?
  → compiler error: "the trait bound `NewType: garde::Validate` is not satisfied"
  → Agent adds #[derive(garde::Validate)]

  ↓
Adds #[derive(Validate)] but no field rules?
  → garde compile error: field must have a #[garde(...)] attribute
  → Agent adds validation rules to each field (or #[garde(skip)] with reason)

  ↓
Adds validation rules → adapter is guarded ✓
```

Every step is a compile-time or lint-time failure with a specific instruction on how to fix it. The agent never needs to read this doc — the error messages guide it through the entire process. No runtime surprises. No hooks needed for this enforcement (though the pre-commit hook catches it too via `cargo clippy`).

## Per-Crate Overrides

Clippy uses the first `clippy.toml` found walking from crate dir to workspace root. If a crate needs different rules, place a `clippy.toml` in that crate's directory. **It replaces the workspace file entirely** — copy all workspace rules and add/modify as needed.

In practice, the workspace-level ban works for most cases because:
- Only the `api` crate imports `axum::Json` — banning it workspace-wide has no effect on other crates
- `serde_json::from_*` is used in build scripts and tests — these get explicit `#[allow]` with reasons
- Inner crates (domain, app) don't do deserialization anyway

## Checklist for New Adapters

When adding a new adapter (inbound or outbound):

1. Define the boundary types (request/response structs)
2. Add `#[derive(serde::Deserialize, garde::Validate)]` to each type
3. Add `#[garde(...)]` rules to every field (or `#[garde(skip)]` with reason)
4. For nested types: use `#[garde(dive)]` to validate recursively
5. Inbound: use `ValidatedJson<T>` extractor (not `axum::Json<T>`)
6. Outbound: return `Validated<T>` from port trait methods
7. If dynamic constraints needed: define a context struct, use `#[garde(context(...))]`

## Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
garde = { version = "0.21", features = ["derive"] }
```

## References

- [Garde documentation](https://docs.rs/garde/latest/garde/)
- [Garde GitHub](https://github.com/jprochazk/garde)
- [Clippy disallowed_types](https://rust-lang.github.io/rust-clippy/master/index.html#disallowed_types)
- [Clippy disallowed_methods](https://rust-lang.github.io/rust-clippy/master/index.html#disallowed_methods)
