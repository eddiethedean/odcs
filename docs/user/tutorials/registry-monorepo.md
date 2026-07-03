# Multi-contract monorepo with registry

This tutorial walks through indexing a directory of contracts and validating a consumer that references a provider by fully-qualified name (FQN).

## Prerequisites

- `odcs` or `pyodcs` **0.9.0+** installed
- Repository checkout (or copy the [registry example](../../examples/registry/) files)

## Layout

```text
contracts/
  provider.yaml    # id: provider-contract
  consumer.yaml    # FQN relationship to provider
```

The repository includes a working copy at [examples/registry/](../../examples/registry/).

## Step 1 — Inspect the consumer

Open `examples/registry/consumer.yaml`. The relationship endpoint uses an FQN such as `provider-contract/customers/customer_id` instead of a local schema path.

## Step 2 — Validate without dependencies (optional)

```bash
odcs validate examples/registry/consumer.yaml
```

This may print `valid`, but FQN endpoints are **not** resolved against other files. Cross-contract reference checks are skipped when no dependencies are loaded.

## Step 3 — Build the registry index

```bash
odcs registry index examples/registry/
```

You should see indexed entries on stdout and a summary on stderr:

```text
provider-contract 1.0.0 (...)
indexed 2 contract(s)
```

This writes `examples/registry/.odcs/registry.json`.

## Step 4 — Validate with the registry

```bash
odcs validate examples/registry/consumer.yaml --registry examples/registry/
```

Expected output:

```text
valid
```

Exit code `0`. FQN references now resolve against indexed provider contracts.

## Step 5 — Alternative: explicit dependency

When you have a single known provider, skip the index:

```bash
odcs validate examples/registry/consumer.yaml \
  --dep examples/registry/provider.yaml
```

## Step 6 — Python equivalent

```python
import pyodcs

pyodcs.registry_index_and_save("examples/registry/")
report = pyodcs.parse_and_validate_paths(
    "examples/registry/consumer.yaml",
    registry="examples/registry/",
)
assert pyodcs.is_valid(report)
```

## Step 7 — Wire into CI

```bash
odcs registry index ./contracts/
for f in contracts/*.yaml; do
  odcs validate "$f" --registry ./contracts/
done
```

Pin the tool version for reproducibility:

```bash
cargo install odcs --version 0.9.1 --locked
```

## What to read next

| Goal | Document |
|------|----------|
| Registry concepts and limits | [Local registry](../registry.md) |
| GitHub Actions recipe | [CI/CD integration](../ci-cd.md) |
| FQN format and load order | [Cross-file references](../../implementation/cross-file-references.md) |
| Troubleshooting | [Troubleshooting](../troubleshooting.md) |
