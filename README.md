# odcs — Rust & Python reference implementation for ODCS

[![CI](https://github.com/eddiethedean/odcs/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/odcs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/odcs)](https://crates.io/crates/odcs)
[![PyPI](https://img.shields.io/pypi/v/pyodcs)](https://pypi.org/project/pyodcs/)
[![Documentation](https://readthedocs.org/projects/odcs/badge/?version=latest)](https://odcs.readthedocs.io/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

> **Reference implementation, not the specification.** This repo validates ODCS documents. The normative [Open Data Contract Standard (ODCS)](https://github.com/bitol-io/open-data-contract-standard) is maintained by Bitol.

**odcs** validates ODCS YAML/JSON files locally and in CI.

It checks that your contract document is well-formed and conforms to ODCS v3.1.0 — schema, quality rules, SLAs, ownership, and server metadata. It does **not** run quality checks against live data.

> **Release:** **0.9.0** is published on [crates.io](https://crates.io/crates/odcs) and [PyPI](https://pypi.org/project/pyodcs/). **1.0.0** stabilization is on `main` — see [Release status](https://odcs.readthedocs.io/en/latest/project/release-status/) and [API stability policy](https://odcs.readthedocs.io/en/latest/implementation/api-stability/).

**Install → validate in 60 seconds:**

```bash
cargo install odcs   # or: pip install pyodcs
odcs version         # crateVersion 0.9.0, upstreamSpecVersion 3.1.0
pyodcs version       # same output from the Python CLI
odcs validate contract.yaml
```

New to ODCS? Read [What is ODCS?](https://odcs.readthedocs.io/en/latest/user/what-is-odcs/) first.

| | |
|---|---|
| **Upstream ODCS** | 3.1.0 |
| **Rust crate** | [`odcs`](https://crates.io/crates/odcs) |
| **Python package** | [`pyodcs`](https://pypi.org/project/pyodcs/) |
| **Documentation** | [odcs.readthedocs.io](https://odcs.readthedocs.io/) |
| **Rust API docs** | [docs.rs/odcs](https://docs.rs/odcs) |

## Quick start

Copy [`examples/minimal.odcs.yaml`](examples/minimal.odcs.yaml) or save as `contract.yaml` (`.yaml` and `.odcs.yaml` both work):

```yaml
version: "1.0.0"      # your contract revision
apiVersion: "v3.1.0"  # ODCS spec release — not the same as version
kind: "DataContract"
id: "hello-contract"
status: "draft"
schema:
  - name: customers
    logicalType: object
    properties:
      - name: customer_id
        logicalType: string
        required: true
```

```bash
odcs validate contract.yaml   # prints: valid
```

On failure, diagnostics include stable codes and paths:

```text
[error] odcs:invalid-kind: expected kind 'DataContract', got 'WrongKind'
  at: kind
```

See [Diagnostics](https://odcs.readthedocs.io/en/latest/user/diagnostics/) and [Troubleshooting](https://odcs.readthedocs.io/en/latest/user/troubleshooting/).

From code:

```python
import pyodcs

report = pyodcs.parse_and_validate(open("contract.yaml", "rb").read(), format="yaml")
assert pyodcs.is_valid(report)
```

Full walkthrough: [Getting started](https://odcs.readthedocs.io/en/latest/user/getting-started/) · [Installation](https://odcs.readthedocs.io/en/latest/user/installation/)

## Documentation

**Full docs:** [odcs.readthedocs.io](https://odcs.readthedocs.io/)

| I want to… | Read |
|------------|------|
| Learn what ODCS is | [What is ODCS?](https://odcs.readthedocs.io/en/latest/user/what-is-odcs/) |
| Get started in 5 minutes | [Getting started](https://odcs.readthedocs.io/en/latest/user/getting-started/) |
| Install Rust or Python | [Installation](https://odcs.readthedocs.io/en/latest/user/installation/) |
| Choose the right API | [API decision guide](https://odcs.readthedocs.io/en/latest/user/api-guide/) |
| Use the CLI | [CLI](https://odcs.readthedocs.io/en/latest/user/cli/) |
| Integrate in CI/CD | [CI/CD](https://odcs.readthedocs.io/en/latest/user/ci-cd/) |
| Fix validation errors | [Troubleshooting](https://odcs.readthedocs.io/en/latest/user/troubleshooting/) |
| Browse examples | [Examples](https://odcs.readthedocs.io/en/latest/examples/) |
| Local registry & cross-file refs | [examples/registry/](examples/registry/) |
| Contribute | [Contributing](https://odcs.readthedocs.io/en/latest/contributing/) |
| Upstream sync policy (maintainers) | [SPEC.md](SPEC.md) |
| Report a security issue | [SECURITY.md](SECURITY.md) |

**Status:** 0.9.0 published; 1.0 stabilization on `main` — [Release status](https://odcs.readthedocs.io/en/latest/project/release-status/) · [API stability](https://odcs.readthedocs.io/en/latest/implementation/api-stability/) · [ROADMAP.md](ROADMAP.md)

> This repository implements the standard; it is not the ODCS specification itself.

## Pipeline

```text
ODCS Document → Parser → Canonical Object Model → Validator → Diagnostics
```

Execution, pipeline composition, and transformation semantics are out of scope. See [non-goals](https://odcs.readthedocs.io/en/latest/implementation/non-goals/).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Run `./scripts/check.sh` before opening a PR. When implementation guidance conflicts with the upstream ODCS specification, **the upstream specification wins**.

## License

Apache License 2.0. See [LICENSE](LICENSE).
