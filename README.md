# odcs — Rust & Python reference implementation for ODCS

[![CI](https://github.com/eddiethedean/odcs/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/odcs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/odcs)](https://crates.io/crates/odcs)
[![PyPI](https://img.shields.io/pypi/v/pyodcs)](https://pypi.org/project/pyodcs/)
[![Documentation](https://readthedocs.org/projects/odcs/badge/?version=latest)](https://odcs.readthedocs.io/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

**odcs** validates [Open Data Contract Standard (ODCS)](https://github.com/bitol-io/open-data-contract-standard) YAML/JSON files locally and in CI.

It checks that your contract document is well-formed and conforms to ODCS v3.1.0 — schema, quality rules, SLAs, ownership, and server metadata. It does **not** run quality checks against live data.

> **Release note:** This tree is **0.5.0** on `main`. Latest published releases on [crates.io](https://crates.io/crates/odcs) and [PyPI](https://pypi.org/project/pyodcs/) are **0.4.0** until `v0.5.0` is tagged. See [Release status](https://odcs.readthedocs.io/en/latest/project/release-status/).

**Install → validate in 60 seconds:**

```bash
cargo install odcs   # or: pip install pyodcs
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

Save a minimal contract as `contract.yaml`:

```yaml
version: "1.0.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "hello-contract"
status: "draft"
schema:
  - name: customers
    properties:
      - name: customer_id
        logicalType: string
        required: true
```

```bash
odcs validate contract.yaml   # prints: valid
```

From code:

```python
import pyodcs

report = pyodcs.parse_and_validate(open("contract.yaml", "rb").read(), format="yaml")
assert pyodcs.is_valid(report)
```

Full walkthrough: [Getting started](https://odcs.readthedocs.io/en/latest/user/getting-started/) · [Installation](docs/user/installation.md)

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
| Contribute | [Contributing](https://odcs.readthedocs.io/en/latest/contributing/) |
| Report a security issue | [SECURITY.md](SECURITY.md) |

**Status:** Alpha pre-1.0 — see [ROADMAP.md](ROADMAP.md) and [Release status](https://odcs.readthedocs.io/en/latest/project/release-status/).

> This repository implements the standard; it is not the ODCS specification itself.

## Pipeline

```text
ODCS Document → Parser → Canonical Object Model → Validator → Diagnostics
```

Execution, pipeline composition, and transformation semantics are out of scope. See [non-goals](https://odcs.readthedocs.io/en/latest/implementation/non-goals/).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). When implementation guidance conflicts with the upstream ODCS specification, **the upstream specification wins**.

## License

Apache License 2.0. See [LICENSE](LICENSE).
