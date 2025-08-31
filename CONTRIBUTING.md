# Contributing to RelayHub

Thanks for your interest in contributing! This document outlines how to propose changes and collaborate effectively.

## Code of Conduct

Read [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)

## Development Model
RelayHub is split into:
- **core/** – Rust library: networking, crypto, transfer logic
- **ui/** – Flutter app (desktop + Android)
- **plugins/** – extension playground (docs TBD)

## Getting Started
1. **Fork** the repository and create a feature branch:
   ```bash
   git checkout -b feat/short-description
   ```

2. Rust toolchain:

```bash
rustup update stable && \
rustup component add rustfmt clippy
```

3. Build & test:

```bash
cd core && \
cargo fmt --all -- --check && \
cargo clippy --all-targets -- -D warnings && \
cargo test
```

## Commit Style

Use conventional titles when possible:

- feat: add X
- fix: correct Y
- refactor: improve Z
- docs: update README
- ci: add workflow
- chore: for misc

> Keep commits focused; prefer smaller PRs.

## Pull Requests

1. Fill out the PR template (TBD)

2. Link related issues

3. Include tests when adding behavior

4. Keep CI green (fmt + clippy + tests)

## Licensing

By contributing, you agree that your contributions are licensed under GPL‑3.0‑or‑later.

## Security & Responsible Disclosure

Please do not open public issues for potential vulnerabilities. Contact maintainers privately.
