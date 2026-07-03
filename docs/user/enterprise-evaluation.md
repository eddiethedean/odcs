# Enterprise evaluation brief

Summary for teams evaluating `odcs` / `pyodcs` for organizational adoption.

## Product scope

| In scope | Out of scope |
|----------|--------------|
| Parse ODCS v3.1.0 YAML/JSON | Execute quality rules against live data |
| Validate structure, semantics, JSON Schema | ETL / pipeline orchestration |
| CLI and Rust/Python libraries | Contract registry server (planned) |
| Structured diagnostics for CI routing | Compatibility diff reporting (planned) |

See [Non-goals](../implementation/non-goals.md).

## Maturity

| Attribute | Status |
|-----------|--------|
| Release stage | **Pre-1.0 Alpha** |
| Current tree version | 0.7.0 on `main` |
| Latest published (crates.io / PyPI) | **0.7.0** — see [Release status](../project/release-status.md) |
| ODCS spec target | v3.1.0 (`apiVersion: "v3.1.0"`) |
| Default validation | Schema-complete for ODCS v3.1.0 (JSON Schema + Rust validators) |

## Support model

- Open-source reference implementation maintained on GitHub
- No commercial SLA or dedicated support channel
- Security reports: [SECURITY.md](../../SECURITY.md) (supported: 0.5.x, 0.4.x)
- Issues and contributions via GitHub

## Security

- Validates **documents only**; no network calls during validation
- Default parse size limit: **16 MiB** (`MAX_PARSE_BYTES`)
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

| Planned (not yet shipped) | Target |
|---------------------------|--------|
| `validationPhase` on diagnostics | 0.6.0 ✓ |
| Cross-field structural validation | 0.7.0 ✓ |
| Contract registry module | 0.9.0 (local filesystem) |
| Compatibility analysis | 0.8.0 |

Full timeline: [Roadmap](../roadmap.md).

## Recommended evaluation path

1. [What is ODCS?](what-is-odcs.md) — confirm problem fit
2. [Getting started](getting-started.md) — validate a sample contract
3. [CI/CD integration](ci-cd.md) — prototype pipeline gate
4. [Release status](../project/release-status.md) — align on published vs `main` version
5. [Architecture](../implementation/architecture.md) — technical depth for platform teams

## Comparison positioning

| Approach | When to use |
|----------|-------------|
| **`odcs validate`** | ODCS-aware validation with stable `odcs:*` codes and typed object model |
| **Generic JSON Schema validator** | Schema-only checks without ODCS-specific Rust validators or diagnostics |
| **Upstream ODCS tooling** | Spec authoring and ecosystem tools from bitol-io |

This implementation adds deterministic Rust validators, duplicate-key detection, and CI-friendly diagnostics on top of the pinned ODCS JSON Schema.
