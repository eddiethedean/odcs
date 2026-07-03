# Registry and cross-file example

Demonstrates **local contract registry** (0.9.0+): index contracts in a directory, then validate a consumer that references a provider by fully-qualified name (FQN) without passing `--dep`.

## Files

| File | Role |
|------|------|
| [provider.yaml](provider.yaml) | Provider contract (`id: provider-contract`) |
| [consumer.yaml](consumer.yaml) | Consumer with FQN relationship to provider |

## Steps

From the repository root:

```bash
# 1. Build the registry index
odcs registry index examples/registry/

# 2. Validate the consumer (provider resolved via registry)
odcs validate examples/registry/consumer.yaml --registry examples/registry/
```

Expected output: `valid` (exit code `0`).

## Lookup

```bash
odcs registry lookup examples/registry/ provider-contract
odcs registry list examples/registry/
```

## Python

```python
import pyodcs

pyodcs.registry_index_and_save("examples/registry/")
report = pyodcs.parse_and_validate_paths(
    "examples/registry/consumer.yaml",
    registry="examples/registry/",
)
assert pyodcs.is_valid(report)
```

## Without `--registry` or `--dep`

Single-file `odcs validate` does not load dependency contracts, so FQN relationship endpoints are not resolved against other files (validation may still pass). To enforce cross-contract references, pass `--registry` or `--dep`:

```bash
# Explicit dependency (no index required)
odcs validate examples/registry/consumer.yaml --dep examples/registry/provider.yaml
```

See [Cross-file references](../../docs/implementation/cross-file-references.md) and [Local registry](../../docs/implementation/registry.md).
