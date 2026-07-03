# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.8.x   | Yes       |
| 0.9.x   | Yes       |
| 0.6.x   | Yes       |
| 0.5.x   | Yes       |
| 0.4.x   | Yes       |
| < 0.4   | No        |

## Reporting a vulnerability

If you discover a security vulnerability in `odcs` or `pyodcs`, please report it responsibly:

1. **Do not** open a public GitHub issue for security-sensitive reports.
2. Email the maintainer via the contact listed on the [GitHub repository](https://github.com/eddiethedean/odcs) profile, or open a [GitHub Security Advisory](https://github.com/eddiethedean/odcs/security/advisories/new) if enabled.
3. Include:
   - A description of the vulnerability
   - Steps to reproduce
   - Potential impact (e.g. denial of service, memory safety, path traversal)
   - Affected version(s)

## Scope

In scope:

- The `odcs` Rust crate and CLI
- The `pyodcs` Python package and native extension
- Parsing of untrusted ODCS documents (size limits, malformed input)

Out of scope:

- Vulnerabilities in upstream ODCS specification documents
- Issues in consumer applications that use this library

## Response expectations

- Acknowledgment within 7 days
- Status update within 30 days
- Coordinated disclosure after a fix is available

## Safe usage

- This tool validates contract *documents*; it does not execute quality checks against live data.
- Default parse size limit is 16 MiB (`MAX_PARSE_BYTES`).
- Run current supported releases in CI and production validation pipelines.

### YAML parsing limits

| Control | Limit |
|---------|-------|
| Document size | 16 MiB maximum |
| Duplicate-key detection | Block-style YAML mappings and JSON objects only |
| Not scanned | YAML flow mappings, anchors (`&`), aliases (`*`) |
| Nesting depth | No explicit cap; bounded primarily by document size |

Malicious YAML may exploit alias expansion in `serde_yaml`. Only parse contracts from trusted sources or scan in isolated CI environments.

### Registry path confinement

Local registry indexing resolves contract paths with `canonicalize()` and rejects entries that resolve outside the registry root directory (including symlink escapes).

### CLI file access

The CLI reads user-supplied paths as-is. It is not sandboxed; do not run against untrusted filesystem locations with sensitive permissions.
