# cliff-toml-parser-runtime-assertions

Shared proof helpers for `cliff-toml-parser-runtime` sidecar tests.

The runtime sidecars call these helpers instead of asserting on parsed values
directly, so every parser proof goes through one reusable shared assertions
surface.
