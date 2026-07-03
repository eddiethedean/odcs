# Cross-file reference resolution

Design for Phase 13 (0.8.0): resolve fully-qualified relationship endpoints across multiple loaded contracts.

## Index key

Contracts are indexed by root **`id`** (stable contract identifier). Filename stems are not used for resolution.

Duplicate `id` values within a loaded set produce a load-time validation error (`odcs:invalid-schema`).

## FQN grammar

Reuse the existing regex in `src/validation/references.rs` (`fully_qualified_reference_regex`).

Canonical resolved form:

```text
{contractId}/{schemaObject}/{property}
```

Optional URL prefix (e.g. `https://example.com/contracts/foo.yaml#`) is accepted for format validation but ignored during resolution — only path segments after optional `#` or leading `/` are used.

Shorthand references (`table.column`) continue to resolve within the **primary** contract only.

## Load order

When building a `ContractSet`:

1. Primary contract (CLI path argument or library primary path)
2. Explicit `--dep` paths in command order
3. Files from `--include` directory: non-recursive scan of `*.yaml`, `*.yml`, `*.json`, sorted lexicographically by filename

The primary contract is always validated; dependencies are indexed for reference resolution only (their own validation diagnostics are merged into the set report).

## Single-document behavior

When no dependencies are loaded, validation behavior is unchanged from 0.7.0. FQN strings that match the regex but cannot be resolved emit `odcs:unresolved-reference` with remediation to include the referenced contract.

## CLI

```bash
odcs validate contract.yaml --dep other.yaml --include ./contracts/
```

- `--dep <path>` — repeatable; explicit dependency files
- `--include <dir>` — non-recursive directory scan

## Out of scope (0.8.0)

- Remote URL fetching
- Registry-backed resolution (Phase 15)
- Workspace manifest files
- Recursive directory scanning
