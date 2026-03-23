# TS-TSCONFIG — tsconfig.json checker (14 rules)

**Input:** tsconfig.json / tsconfig.base.json (JSON parsed)
**Current code:** `tsconfig_check.rs`

## Rules

| New ID | Old ID | Setting | Description | Status |
|--------|--------|---------|-------------|--------|
| TS-TSCONFIG-01 | T9 | file existence | tsconfig.json or tsconfig.base.json exists + valid JSON | Implemented |
| TS-TSCONFIG-02 | T9 | strict | `strict: true` | Implemented |
| TS-TSCONFIG-03 | T9 | noImplicitReturns | `noImplicitReturns: true` | Implemented |
| TS-TSCONFIG-04 | T9 | noUnusedLocals | `noUnusedLocals: true` | Implemented |
| TS-TSCONFIG-05 | T9 | noUnusedParameters | `noUnusedParameters: true` | Implemented |
| TS-TSCONFIG-06 | T9 | forceConsistentCasingInFileNames | `forceConsistentCasingInFileNames: true` | Implemented |
| TS-TSCONFIG-07 | T52 | noUncheckedIndexedAccess | Array/object index returns `T \| undefined` | Implemented |
| TS-TSCONFIG-08 | T53 | exactOptionalPropertyTypes | Distinguishes `undefined` value from missing | Implemented |
| TS-TSCONFIG-09 | T54 | isolatedModules | Each file independently transpilable | Implemented |
| TS-TSCONFIG-10 | T62 | noFallthroughCasesInSwitch | Switch fallthrough detection | Implemented |
| TS-TSCONFIG-11 | T63 | allowUnreachableCode=false | Unreachable code detection | Implemented |
| TS-TSCONFIG-12 | T64 | allowUnusedLabels=false | Unused label detection | Implemented |
| TS-TSCONFIG-13 | T65 | target=es2022 | Target ES version | Implemented |
| TS-TSCONFIG-14 | T66 | module=esnext | Module system | Implemented |
| TS-TSCONFIG-15 | T67 | moduleResolution=bundler | Module resolution strategy | Implemented |
| TS-TSCONFIG-16 | T68 | esModuleInterop | CommonJS/ES interop | Implemented |
| TS-TSCONFIG-17 | T-TSC-60 | noPropertyAccessFromIndexSignature | Forces bracket notation for index signatures | Implemented |
| TS-TSCONFIG-18 | T-TSC-61 | noImplicitOverride | Requires explicit `override` keyword | Implemented |
| TS-TSCONFIG-19 | T10 | extra compilerOptions inventory | Non-baseline options present | Implemented |

**Note:** T9 was overloaded — one old ID covering 6 different boolean settings. New IDs split them out (TS-TSCONFIG-01 through 06). Total: 19 rules (up from 5 in old plan).
