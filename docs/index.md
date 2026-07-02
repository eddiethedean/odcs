# odcs documentation

Reference implementation of the [Open Data Contract Standard (ODCS)](https://github.com/bitol-io/open-data-contract-standard) — parse, validate, and inspect data contracts from Rust or Python.

!!! note "Release"
    **0.7.0** is on `main` and ready to publish. Push tag `v0.7.0` to release. See [Release status](project/release-status.md).

**Normative ODCS specification:** [bitol-io/open-data-contract-standard](https://github.com/bitol-io/open-data-contract-standard) · [Published spec site](https://bitol-io.github.io/open-data-contract-standard/)

New to ODCS? Start with [What is ODCS?](user/what-is-odcs.md).

## I want to…

| Goal | Start here |
|------|------------|
| Learn what ODCS is | [What is ODCS?](user/what-is-odcs.md) |
| Get started in 5 minutes | [Getting started](user/getting-started.md) |
| Install Rust or Python | [Installation](user/installation.md) |
| Use the CLI | [CLI](user/cli.md) |
| Use from Rust | [Rust](user/rust.md) |
| Use from Python | [Python](user/python.md) |
| Choose the right API | [API decision guide](user/api-guide.md) |
| Author a contract | [Authoring contracts](user/authoring.md) |
| Integrate in CI/CD | [CI/CD integration](user/ci-cd.md) |
| Understand error codes | [Diagnostics](user/diagnostics.md) |
| Fix a broken contract | [Fix your first invalid contract](user/tutorials/fix-invalid-contract.md) |
| Troubleshoot issues | [Troubleshooting](user/troubleshooting.md) |
| Upgrade between versions | [Migration](user/migration.md) |
| Browse examples | [Examples](examples.md) |
| Evaluate for enterprise use | [Enterprise evaluation](user/enterprise-evaluation.md) |
| Answer common questions | [FAQ](user/faq.md) |
| Contribute code | [Contributing](contributing.md) |
| Check published vs main version | [Release status](project/release-status.md) |
| Cut a release | [Releasing](maintainer/releasing.md) |

## User guides

| Document | Description |
|----------|-------------|
| [What is ODCS?](user/what-is-odcs.md) | Primer for newcomers |
| [Getting started](user/getting-started.md) | Five-minute quick start |
| [Installation](user/installation.md) | Install Rust, Python, from source |
| [CLI](user/cli.md) | CLI commands, flags, exit codes |
| [Rust](user/rust.md) | Rust API and library usage |
| [Python](user/python.md) | Python API and CLI |
| [API decision guide](user/api-guide.md) | Which function to use when |
| [Authoring contracts](user/authoring.md) | Write a minimal ODCS contract |
| [Diagnostics](user/diagnostics.md) | Diagnostic codes and JSON shape |
| [CI/CD integration](user/ci-cd.md) | GitHub Actions and pre-commit |
| [Migration](user/migration.md) | Upgrade guide between releases |
| [Troubleshooting](user/troubleshooting.md) | Symptom → fix index |
| [FAQ](user/faq.md) | Frequently asked questions |
| [Glossary](user/glossary.md) | ODCS and tool terminology |

## Project

| Document | Description |
|----------|-------------|
| [Release status](project/release-status.md) | Published vs `main` version |
| [Changelog](changelog.md) | Release notes |
| [Roadmap](roadmap.md) | Reference implementation milestones |
| [Upstream sync policy](upstream-sync-policy.md) | How this repo tracks the ODCS spec |

## Implementation guides

For contributors and maintainers building the Rust crate in `src/`:

| Document | Description |
|----------|-------------|
| [Implementation overview](implementation/overview.md) | Implementation guide overview |
| [Architecture](implementation/architecture.md) | Pipeline and module boundaries |
| [Crate layout](implementation/crate-layout.md) | Module layout |
| [Public API](implementation/public-api.md) | Public Rust API (maintainer reference) |
| [Non-goals](implementation/non-goals.md) | Out-of-scope features |

## Maintainer guides

| Document | Description |
|----------|-------------|
| [Releasing](maintainer/releasing.md) | Release workflow (crates.io + PyPI) |

## Related links

- [Rust API docs (docs.rs)](https://docs.rs/odcs)
- [PyPI package](https://pypi.org/project/pyodcs/)
- [Security policy](../SECURITY.md)
