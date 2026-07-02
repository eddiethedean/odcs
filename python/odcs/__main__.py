"""ODCS command-line entry point (planned)."""

import sys


def main() -> None:
    print(
        "Python bindings are not yet available. Use the Rust CLI: cargo run --",
        *sys.argv[1:],
        file=sys.stderr,
    )
    sys.exit(1)
