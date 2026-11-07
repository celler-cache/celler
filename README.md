# Celler

**Celler** is a self-hostable Nix Binary Cache server backed by an S3-compatible storage provider.
It has support for global deduplication and garbage collection.

Check the [book](https://celler.x86.lol) for admin, user, and developer
documentation. For spinning up your own Celler for testing, jump to the
[tutorial](https://celler.x86.lol/tutorial.html).

## Relationship to Attic

Celler is derived from [Attic](https://github.com/zhaofengli/attic). See the
[migration guide](https://celler.x86.lol/admin-guide/attic-migration.html) for
migration instructions.

We love Attic for its design, clear code and ease-of-use, but found it
lacking for production setups. Development also seems to be stalled.

See the [changelog](CHANGELOG.md) for the current list of changes compared to Attic.

We are also interested in the following features and will work on them
as time permits.

- Prometheus Metrics for requests, duration of requests, NAR file size, chunk size, etc.
- Request logging
- Integration into Single-Sign On via OpenID Connect.
- Store path pinning.

## Licensing

Celler is available under the **Apache License, Version 2.0**.
See `LICENSE` for details.

By contributing to the project, you agree to license your work under the
aforementioned license.

## Contact

Chat with us on Matrix: [#celler:chat.x86.lol](https://matrix.to/#/%23celler:chat.x86.lol)
