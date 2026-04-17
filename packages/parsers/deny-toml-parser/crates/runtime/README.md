# deny-toml-parser-runtime

Internal runtime crate for `deny-toml-parser`.

It owns the filesystem boundary, parse entrypoints, and sidecar tests.
External callers should depend on the root facade crate instead of this crate.
