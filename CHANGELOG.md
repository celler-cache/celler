# Changelog

All notable changes to Celler will be documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `CELLER_TOKEN` environment variable as an alternative to `celler login`, useful for CI and scripted use.
- `celler push` now prints the total NAR size.

### Changed

- Binaries renamed: `attic` → `celler`, `atticd` → `cellerd`.
- NixOS module renamed: `services.atticd` → `services.cellerd`.
- Environment variables renamed: `ATTIC_SERVER_*` → `CELLER_SERVER_*`.
- Replaced C++ FFI to `libnixstore` with the pure-Rust `nix-daemon` crate.
- Updated `sea-orm` to 2.0: Postgres connections should now specify a username, because the default user is no longer the user account name.

### Removed

- WASM build target.
- Static package builds.

### Fixed

- S3 storage: improved tolerance for transient errors.
- Improved error logging for `push` and `watch-store`.

[Unreleased]: https://github.com/blitz/celler/compare/12cbeca141f46e1ade76728bce8adc447f2166c6...HEAD
