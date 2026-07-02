# odcs documentation

Reference implementation of the [Open Data Contract Standard (ODCS)](https://github.com/bitol-io/open-data-contract-standard) — parse, validate, and inspect data contracts from Rust or Python.

**Normative ODCS specification:** [bitol-io/open-data-contract-standard](https://github.com/bitol-io/open-data-contract-standard)

## I want to…

| Goal | Start here |
|------|------------|
| Get started in 5 minutes | [Getting started](user/getting-started.md) |
| Install Rust or Python | [Installation](user/installation.md) |
| Use the CLI | [CLI](user/cli.md) |
| Use from Rust | [Rust](user/rust.md) |
| Use from Python | [Python](user/python.md) |
| Author a contract | [Authoring contracts](user/authoring.md) |
| Integrate in CI/CD | [CI/CD integration](user/ci-cd.md) |
| Understand error codes | [Diagnostics](user/diagnostics.md) |
| Upgrade between versions | [Migration](user/migration.md) |
| Browse examples | [Examples](upstream/examples.md) |
| Answer common questions | [FAQ](user/faq.md) |
| Contribute code | [Contributing](upstream/contributing.md) |
| Cut a release | [Releasing](maintainer/releasing.md) |

## User guides

| Document | Description |
|----------|-------------|
| [Getting started](user/getting-started.md) | Five-minute quick start |
| [Installation](user/installation.md) | Install Rust, Python, from source |
| [CLI](user/cli.md) | CLI commands, flags, exit codes |
| [Rust](user/rust.md) | Rust API and library usage |
| [Python](user/python.md) | Python API and CLI |
| [Authoring contracts](user/authoring.md) | Write a minimal ODCS contract |
| [Diagnostics](user/diagnostics.md) | Diagnostic codes and JSON shape |
| [CI/CD integration](user/ci-cd.md) | GitHub Actions and pre-commit |
| [Migration](user/migration.md) | Upgrade guide between releases |
| [FAQ](user/faq.md) | Frequently asked questions |

## Project

| Document | Description |
|----------|-------------|
| [Specification](upstream/spec.md) | Repository policy and upstream synchronization |
| [Roadmap](upstream/roadmap.md) | Reference implementation milestones |
| [Changelog](upstream/changelog.md) | Release notes |

## Implementation guides

For contributors and maintainers building the Rust crate in `src/`:

| Document | Description |
|----------|-------------|
| [Implementation overview](implementation/overview.md) | Implementation guide overview |
| [Project goal](implementation/project-goal.md) | Project scope |
| [Architecture](implementation/architecture.md) | Pipeline architecture |
| [Crate layout](implementation/crate-layout.md) | Module layout |
| [Model guide](implementation/model-guide.md) | Canonical Object Model |
| [Validation guide](implementation/validation-guide.md) | Validation phases |
| [Diagnostics guide](implementation/diagnostics-guide.md) | Diagnostics types |
| [Public API](implementation/public-api.md) | Public Rust API (maintainer reference) |
| [CLI specification](implementation/cli-spec.md) | CLI specification |
| [Implementation phases](implementation/implementation-phases.md) | Build phases |
| [Testing plan](implementation/testing-plan.md) | Test coverage |
| [Rust dependencies](implementation/rust-dependencies.md) | Dependencies |
| [Spec usage](implementation/spec-usage.md) | Using upstream ODCS during development |
| [Non-goals](implementation/non-goals.md) | Out-of-scope features |
| [Relationship to DTCS](implementation/relationship-to-dtcs.md) | Ecosystem positioning |

## Maintainer guides

| Document | Description |
|----------|-------------|
| [Releasing](maintainer/releasing.md) | Release workflow (crates.io + PyPI) |

## Related links

- [Rust API docs (docs.rs)](https://docs.rs/odcs)
- [PyPI package](https://pypi.org/project/pyodcs/)
- [Security policy](../SECURITY.md)
- [Upstream ODCS specification](https://github.com/bitol-io/open-data-contract-standard)
