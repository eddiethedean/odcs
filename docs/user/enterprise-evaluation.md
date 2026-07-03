# Enterprise evaluation brief

Summary for teams evaluating `odcs` / `pyodcs` for organizational adoption.

## Product scope

| In scope | Out of scope |
|----------|--------------|
| Parse ODCS v3.1.0 YAML/JSON | Execute quality rules against live data |
| Validate structure, semantics, JSON Schema | ETL / pipeline orchestration |
| CLI and Rust/Python libraries | Remote contract registry server |
| Local registry index (`.odcs/registry.json`) | Automatic contract migration |
| Cross-file FQN resolution | Compatibility auto-fix |
| Compatibility diff (`odcs diff`) | |

See [Non-goals](../implementation/non-goals.md).

## Maturity

| Attribute | Status |
|-----------|--------|
| Release stage | **0.9.0 published; 1.0.0 stabilization complete on `main`, pending release** |
| Current tree version | 0.9.0 on `main` |
| Latest published (crates.io / PyPI) | **0.9.0** — see [Release status](../project/release-status.md) |
| ODCS spec target | v3.1.0 (`apiVersion: "v3.1.0"`) |
| Default validation | Schema-complete for ODCS v3.1.0 (JSON Schema + Rust validators) |
| API stability policy | [api-stability.md](../implementation/api-stability.md) |

## Support model

- Open-source reference implementation maintained on GitHub
- No commercial SLA or dedicated support channel
- Security reports: [SECURITY.md](../../SECURITY.md) (supported: 0.8.x, 0.9.x)
- Issues and contributions via GitHub

## Security

- Validates **documents only**; no network calls during validation
- Default parse size limit: **16 MiB** (`MAX_PARSE_BYTES`)
- Registry indexing rejects symlink paths that resolve outside the registry root
- YAML anchors/aliases not fully duplicate-scanned — see [Diagnostics](../user/diagnostics.md#duplicate-key-limitations-050)
- Untrusted input: treat as potentially hostile; run current supported releases
- Scope and response expectations: [SECURITY.md](../../SECURITY.md)

## License

Apache License 2.0. See [LICENSE](../../LICENSE).

## Supply chain

| Artifact | Registry |
|----------|----------|
| Rust crate `odcs` | [crates.io](https://crates.io/crates/odcs) |
| Python package `pyodcs` | [PyPI](https://pypi.org/project/pyodcs/) |
| CI | GitHub Actions ([`.github/workflows/`](../../.github/workflows/)) |

Pin versions in CI for reproducibility. See [CI/CD integration](ci-cd.md).

## Parity guarantees

- `pyodcs` wraps the Rust core via PyO3 — same parse/validation semantics
- CLI exit codes and diagnostic codes aligned between `odcs` and `pyodcs`
- Upstream ODCS specification is normative when docs and implementation differ — see [Upstream sync policy](../upstream-sync-policy.md)

## Roadmap highlights

| Milestone | Status |
|-----------|--------|
| Structural validation | 0.7.0 ✓ |
| Cross-file references | 0.8.0 ✓ |
| Compatibility analysis | 0.8.0 ✓ |
| Local registry | 0.9.0 ✓ |
| 1.0 API stabilization | Complete on `main`; pending publish |

Full timeline: [Roadmap](../roadmap.md).

## Recommended evaluation path

1. [What is ODCS?](what-is-odcs.md) — confirm problem fit
2. [Getting started](getting-started.md) — validate a sample contract
3. [CI/CD integration](ci-cd.md) — prototype pipeline gate
4. [Local registry](registry.md) — monorepo cross-file validation
5. [Release status](../project/release-status.md) — align on published vs `main` version
6. [Architecture](../implementation/architecture.md) — technical depth for platform teams
7. [API stability policy](../implementation/api-stability.md) — semver expectations for 1.0

## Comparison positioning

This tool validates ODCS **documents** in CI. It complements (does not replace) data quality engines, orchestrators, or enterprise catalog products.
