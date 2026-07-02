# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.7.x   | Yes       |
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
- Planned but unimplemented modules (`registry`, `compatibility`)

## Response expectations

- Acknowledgment within 7 days
- Status update within 30 days
- Coordinated disclosure after a fix is available

## Safe usage

- This tool validates contract *documents*; it does not execute quality checks against live data.
- Default parse size limit is 16 MiB (`MAX_PARSE_BYTES`).
- Run current supported releases in CI and production validation pipelines.
