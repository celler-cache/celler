# Changelog

All notable changes to Celler will be documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `CELLER_TOKEN` environment variable as an alternative to `celler login`, useful for CI and scripted use.
- `celler push` now prints the total NAR size.

### Changed

- **Breaking**: Binaries renamed: `attic` → `celler`, `atticd` → `cellerd`.
- **Breaking**: NixOS module renamed: `services.atticd` → `services.cellerd`.
- Replaced C++ FFI to `libnixstore` with the pure-Rust `nix-daemon` crate.
- Updated [sea-orm](https://www.sea-ql.org/SeaORM/) to v2.x.
- `celleradm` was integrated into `celler` as `celler admin` subcommand.
- **Breaking**: The server doesn't need private keys anymore. Keys are also not BASE64-encoded anymore. Check the [admin guide](https://celler.x86.lol/admin-guide) for the new configuration format.

### Removed

- Configuration via environment variables: `ATTIC_SERVER_*`, `CELLER_SERVER_*`.
- `cellerd` does not create an initial token or a template configuration anymore.
- WASM build target.
- Static package builds.
- Docker containers.

### Fixed

- S3 storage: improved tolerance for transient errors.
- Improved error logging for `push` and `watch-store`.
- `celler login` now fails if the configuration file cannot be written, instead of only logging the error.

[Unreleased]: https://github.com/blitz/celler/compare/12cbeca141f46e1ade76728bce8adc447f2166c6...HEAD
