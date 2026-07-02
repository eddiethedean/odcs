# odcs documentation

Reference implementation of the [Open Data Contract Standard (ODCS)](https://github.com/bitol-io/open-data-contract-standard) — parse, validate, and inspect data contracts from Rust or Python.

## I want to…

| Goal | Start here |
|------|------------|
| Get started in 5 minutes | [Getting started](user/getting-started.md) |
| Install Rust or Python | [Installation](user/installation.md) |
| Use the CLI | [CLI](user/cli.md) |
| Use from Python | [Python](user/python.md) |
| Understand error codes | [Diagnostics](user/diagnostics.md) |
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
| [Python](user/python.md) | Python API and CLI |
| [Diagnostics](user/diagnostics.md) | Diagnostic codes and JSON shape |
| [FAQ](user/faq.md) | Frequently asked questions |

## Upstream specification

| Document | Description |
|----------|-------------|
| [Specification](upstream/spec.md) | Upstream specification policy and synchronization workflow |
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
| [Public API](implementation/public-api.md) | Public Rust API |
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
- [Upstream ODCS specification](https://github.com/bitol-io/open-data-contract-standard)
