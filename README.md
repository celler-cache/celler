# Celler

**Celler** is a self-hostable Nix Binary Cache server backed by an S3-compatible storage provider.
It has support for global deduplication and garbage collection.

It is derived from [Attic](https://github.com/zhaofengli/attic). See the [migration guide](https://celler.x86.org/admin-guide/attic-migration.html) for migration instructions.

## Rationale for Forking

We love Attic for its design, clear code and ease-of-use, but found it
lacking for production setups. Development also seems to be stalled.

See the [changelog](CHANGELOG.md) for the current list of changes compared to Attic.

We are also interested in the following features and will work on them
as time permits.

- Prometheus Metrics for requests, duration of requests, NAR file size, chunk size, etc.
- Request logging
- Integration into Single-Sign On via OpenID Connect.
- Store path pinning.

## Try it out (15 minutes)

Let's [spin up Celler](https://celler.x86.lol/tutorial.html) in just
15 minutes. And yes, it works on macOS too!

## Goals

- **Multi-Tenancy**: Create a private cache for yourself, and one for friends and co-workers. Tenants are mutually untrusting and cannot pollute the views of other caches.
- **Global Deduplication**: Individual caches (tenants) are simply restricted views of the content-addressed NAR Store and Chunk Store. When paths are uploaded, a mapping is created to grant the local cache access to the global NAR.
- **Managed Signing**: Signing is done on-the-fly by the server when store paths are fetched. The user pushing store paths does not have access to the signing key.
- **Scalabilty**: Celler can be easily replicated. It's designed to be deployed to serverless platforms like fly.io but also works nicely in a single-machine setup.
- **Garbage Collection**: Unused store paths can be garbage-collected in an LRU manner.

## Licensing

Celler is available under the **Apache License, Version 2.0**.
See `LICENSE` for details.

By contributing to the project, you agree to license your work under the aforementioned license.

## Contact

Chat with us on Matrix: [#ctrl-os:cyberus-technology.de](https://matrix.to/#/#ctrl-os:cyberus-technology.de)

For commercial support, reach out to [Cyberus Technology](https://ctrl-os.com/).
