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
- S3 storage now uses the [object_store](https://crates.io/crates/object_store) crate instead of the AWS SDK.
- **Breaking**: S3 credentials are no longer read from AWS profile files (`~/.aws/config`, `~/.aws/credentials`, or the files named by `AWS_CONFIG_FILE`/`AWS_SHARED_CREDENTIALS_FILE`). `AWS_PROFILE` and profile features such as SSO, `credential_process` and assume-role are not supported. Configure credentials in the server configuration or via `AWS_ACCESS_KEY_ID`/`AWS_SECRET_ACCESS_KEY`. Web identity, ECS and EC2 instance metadata credentials keep working. Check the [object_store documentation](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.from_env) to see what configuration options are available.
- Interrupted uploads to the object store no longer abort their multipart upload. Incomplete parts may remain in the bucket. Configure an [`AbortIncompleteMultipartUpload` lifecycle rule](https://docs.aws.amazon.com/AmazonS3/latest/userguide/mpu-abort-incomplete-mpu-lifecycle-config.html) on the bucket to clean them up.
- **Breaking**: S3 endpoints using plain HTTP (`http://`) are rejected by default. Set the `AWS_ALLOW_HTTP` environment variable to `true` for `cellerd` to allow them.

### Removed

- Configuration via environment variables: `ATTIC_SERVER_*`, `CELLER_SERVER_*`.
- `cellerd` does not create an initial token or a template configuration anymore.
- WASM build target.
- Static package builds.
- Docker containers.

### Fixed

- S3 storage: improved tolerance for transient errors.
- Improved error logging for `push` and `watch-store`.

[Unreleased]: https://github.com/blitz/celler/compare/12cbeca141f46e1ade76728bce8adc447f2166c6...HEAD
