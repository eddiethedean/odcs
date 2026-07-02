# Validation Guide

Validation should be deterministic and phase-based.

Suggested phases:

1. Document validation
2. Canonical Object Model validation
3. Structural validation
4. Schema validation
5. Quality validation
6. Reference validation
7. Extension validation

Return a `ValidationReport`.

Do not panic on invalid contracts.
