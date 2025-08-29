# RelayHub

*painless cross-device text & file relay over local networks*

[![Rust CI](https://img.shields.io/github/actions/workflow/status/ninja-left/relay-hub/rust-ci.yaml?branch=main)](https://github.com/ninja-left/relay-hub/actions)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Latest Release](https://img.shields.io/github/v/release/ninja-left/relay-hub?display_name=tag)](https://github.com/ninja-left/relay-hub/releases)

## Intro

RelayHub is a cross-platform tool to relay **text** and **files** between devices on the same network. It uses a **Rust core** for core and a **Flutter UI** for desktop and Android. The architecture is **plugin-friendly** from the ground up.

> **License:** GPL‑3.0-or-later

---

## Roadmap

### v0.1
- Rust core crate compiles with stubs (connection/transfer APIs)
- GitHub Actions for Rust (fmt, clippy, build, test)

### v0.2
- Flutter UI skeleton (tabs: Send, Receive, Settings) for desktop & Android
- Flutter CI builds (APK + desktop binaries)

### v0.3
- Manual IP connect for text relay over LAN (TLS to be added next)
- Basic receive loop

### v0.4
- Self-signed certificates (TLS)
- File transfer basics with progress updates

### v0.5
- QR for IP + public key sharing
- LAN discovery (broadcast/scan)

### v0.6+
- Plugin system: optional compression, clipboard sync, Bluetooth, notifications, etc.

---

## Building (Temporary)

```bash
cd core
cargo build
cargo test
```

> Requires Rust (stable). CI runs on Linux/macOS/Windows.

---

## Contributing

See [CONTRIBUTING.md].

---

## Security

Do not open issues for sensitive security reports. Email [ninja.notleft@proton.me](mailto:ninja.notleft@proton.me) or use your preferred PGP method.

[CONTRIBUTING.md]: ./CONTRIBUTING.md
