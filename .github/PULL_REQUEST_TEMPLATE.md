## Summary

<!-- What changed and why? Link issues if applicable. -->

## Type

- [ ] Implementation (Rust / Python bindings)
- [ ] Documentation (`docs/user/` or README)
- [ ] Tests only
- [ ] Infrastructure / CI

## Checklist

- [ ] `cargo test --locked` passes (or `./scripts/check.sh` for full CI parity)
- [ ] Python tests pass if bindings changed (`pytest python/tests -v`)
- [ ] User-facing docs updated if behavior or CLI changed
- [ ] [CHANGELOG.md](CHANGELOG.md) updated for user-visible changes
- [ ] [docs/implementation/public-api.md](docs/implementation/public-api.md) updated if Rust public API changed

## Test plan

<!-- How did you verify the change? -->
