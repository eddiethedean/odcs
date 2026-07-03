# Local contract registry

Use a **local registry** when multiple ODCS contracts live in one repository and consumers reference providers by fully-qualified name (FQN) — for example `provider-contract/customers/customer_id`.

The registry builds an index at `<registry-root>/.odcs/registry.json`. Validation with `--registry` resolves FQN relationship endpoints without listing every dependency with `--dep`.

## When to use a registry

| Scenario | Approach |
|----------|----------|
| Single contract, no cross-file references | `odcs validate contract.yaml` |
| One consumer, one known provider | `odcs validate consumer.yaml --dep provider.yaml` |
| Several contracts in a directory | `odcs registry index ./contracts/` then `odcs validate … --registry ./contracts/` |
| Monorepo CI gate | Index once, validate each changed contract with `--registry` |

!!! warning "Single-file validate does not load dependencies"
    `odcs validate consumer.yaml` alone does **not** resolve FQN endpoints against other contracts. Validation may still print `valid` while cross-contract references are unchecked. See [CI/CD integration](ci-cd.md#cross-file-and-fqn-relationships).

## Quick start

From the repository root:

```bash
odcs registry index examples/registry/
odcs validate examples/registry/consumer.yaml --registry examples/registry/
```

Expected output: `valid` (exit code `0`).

See [examples/registry/](../../examples/registry/) for provider and consumer files.

## Workflow

1. **Index** — scan the registry root recursively and write `.odcs/registry.json`.
2. **Validate** — pass `--registry <root>` so FQN references resolve against indexed contracts.
3. **Re-index** — run `registry index` again after adding, removing, or changing contracts under the root. There is no auto-reindex on validate.

```bash
odcs registry index ./contracts/
odcs validate consumer.yaml --registry ./contracts/
```

### Lookup and list

```bash
odcs registry lookup ./contracts/ provider-contract
odcs registry lookup ./contracts/ provider-contract --version 1.0.0
odcs registry list ./contracts/
```

Add `--json` for structured output. Lookup without `--version` returns the highest semver entry for the id. Exit code `1` when no entry is found.

## Python

```python
import pyodcs

pyodcs.registry_index_and_save("./contracts/")
report = pyodcs.parse_and_validate_paths(
    "consumer.yaml",
    registry="./contracts/",
)
assert pyodcs.is_valid(report)

entry = pyodcs.registry_lookup("./contracts/", "provider-contract")
entries = pyodcs.registry_list("./contracts/")
```

CLI parity: `pyodcs registry index`, `pyodcs registry lookup`, and `pyodcs registry list` mirror the Rust `odcs` commands.

## CI recipe

```bash
odcs registry index ./contracts/
for f in contracts/*.yaml; do
  odcs validate "$f" --registry ./contracts/ --json > /dev/null
done
```

Do **not** run `odcs registry index` in parallel against the same directory — concurrent jobs can race even though writes are atomic.

Set `ODCS_VERBOSE=1` for per-file index progress on stderr (does not affect stdout JSON).

See [CI/CD integration](ci-cd.md) for GitHub Actions examples and untrusted-input guidance.

## Limits and security

| Limit | Value |
|-------|-------|
| Maximum indexed file count | 10,000 contract files per scan |
| Maximum index file size | 16 MiB (`.odcs/registry.json`) |
| Maximum parse size per contract | 16 MiB (`MAX_PARSE_BYTES`) |

- Indexed paths whose canonical form escapes the registry root are rejected.
- Symlink cycles during directory scan are rejected.
- Treat pull-request YAML as potentially hostile — see [SECURITY.md](../../SECURITY.md).

Implementation details: [Local registry (implementation)](../implementation/registry.md) · [Cross-file references](../implementation/cross-file-references.md)

## Troubleshooting

| Symptom | Likely cause | Fix |
|---------|--------------|-----|
| FQN references not validated | Validated without `--registry` or `--dep` | Pass `--registry` or explicit `--dep` paths |
| `registry entry not found` on lookup | Index missing or stale | Run `odcs registry index <dir>` |
| Index fails with duplicate entry | Two files share the same `id` and `version` | Use unique `(id, version)` pairs or separate registry roots |
| Index fails on symlink | Path resolves outside registry root | Remove or replace the symlink |
| CI passes locally but fails in CI | Unpinned tool version | Pin `cargo install odcs --version 0.9.0 --locked` |

See also [Troubleshooting](troubleshooting.md) and [FAQ](faq.md).
