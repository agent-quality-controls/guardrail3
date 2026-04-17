# hook-shell-parser

Facade crate for parsing shell hook scripts into typed executable-line facts.

The public API stays at this root crate. Internal implementation crates live
under `crates/`.

## Usage

```rust
use hook_shell_parser::parse_script;
use hook_shell_parser::types::ParsedShellScript;

let parsed: ParsedShellScript = parse_script(
    "#!/usr/bin/env bash\nguardrail3 rs validate --staged .\n",
);

assert_eq!(parsed.shebang.as_deref(), Some("#!/usr/bin/env bash"));
assert_eq!(parsed.executable_lines[0].command_name, "guardrail3");
```

## License

MIT OR Apache-2.0
