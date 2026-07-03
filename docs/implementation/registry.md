# Local contract registry

Design for Phase 15 (0.9.0): index and lookup ODCS contracts on the local filesystem for cross-file FQN resolution.

## Index location

`odcs registry index <dir>` writes:

```text
<dir>/.odcs/registry.json
```

Entry `path` values are stored **relative to `<dir>`** and resolved to absolute paths at load time.

## Index key

Each entry is keyed by `(id, version)` where:

- `id` — root contract `id`
- `version` — root contract revision (`version` field, e.g. `1.0.0`)

Duplicate `(id, version)` pairs fail at index time.

## Lookup semantics

| API | Behavior |
|-----|----------|
| `lookup(id, version)` | Exact match on `(id, version)` |
| `lookup(id)` | Highest valid semver among entries with that `id`; if none parse as semver, lexicographic max on `version` |

## Scan policy

`registry index` recursively walks `<dir>` for `*.yaml`, `*.yml`, `*.json`.

Limits:

| Limit | Value |
|-------|-------|
| Contract files per scan | 10,000 (`MAX_REGISTRY_CONTRACT_FILES`) |
| Index file size | 16 MiB (`MAX_REGISTRY_INDEX_BYTES`) |
| Directory cycles | Rejected (symlink loops) |

Index writes use atomic temp-file + rename. **Do not run parallel index jobs** against the same registry root.

Set `ODCS_VERBOSE=1` for per-file progress on stderr during indexing.

Skipped directories:

- Hidden directories (name starts with `.`), including `.git` and `.odcs`

## Entry fields

| Field | Source |
|-------|--------|
| `id`, `version`, `apiVersion` | Parsed contract root |
| `tags` | Root `tags` when present |
| `path` | Relative path from registry root |
| `contentHash` | SHA-256 hex of raw file bytes |
| `indexedAt` | ISO 8601 timestamp (optional metadata) |

Parse failures during indexing emit diagnostics and skip the file.

## Integration with validation

```bash
odcs validate consumer.yaml --registry ./contracts/
```

Requires a prior `odcs registry index ./contracts/`. Loads `./contracts/.odcs/registry.json` and adds indexed contract paths as dependencies (see [cross-file-references.md](cross-file-references.md) for load order).

## Out of scope (0.9.0)

- HTTP remote registry
- Authentication or publish/subscribe
- Auto-reindex on validate (run `registry index` explicitly)
- Workspace manifest files
