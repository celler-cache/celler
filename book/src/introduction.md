# Introduction

**Celler** is a self-hostable Nix Binary Cache server backed by an S3-compatible storage provider.
It has support for global deduplication and garbage collection.

> [!IMPORTANT]
> If you come here, because your server didn't start up anymore after an update, please refer to the [changelog](https://github.com/celler-cache/celler/blob/main/CHANGELOG.md) and the [Attic migration guide](admin-guide/attic-migration.md).

Celler is still an early prototype and is looking for more testers. Want to jump in? [Start your own Celler server](./tutorial.md) in 15 minutes.

```
⚙️ Pushing 5 paths to "demo" on "local" (566 already cached, 2001 in upstream)...
✅ gnvi1x7r8kl3clzx0d266wi82fgyzidv-steam-run-fhs (29.69 MiB/s)
✅ rw7bx7ak2p02ljm3z4hhpkjlr8rzg6xz-steam-fhs (30.56 MiB/s)
✅ y92f9y7qhkpcvrqhzvf6k40j6iaxddq8-0p36ammvgyr55q9w75845kw4fw1c65ln-source (19.96 MiB/s)
🕒 vscode-1.74.2        ███████████████████████████████████████  345.66 MiB (41.32 MiB/s)
🕓 zoom-5.12.9.367      ███████████████████████████              329.36 MiB (39.47 MiB/s)
```

## Goals

- **Multi-Tenancy**: Create a private cache for yourself, and one for friends and co-workers. Tenants are mutually untrusting and cannot pollute the views of other caches.
- **Global Deduplication**: Individual caches (tenants) are simply restricted views of the content-addressed NAR Store and Chunk Store. When paths are uploaded, a mapping is created to grant the local cache access to the global NAR.
- **Managed Signing**: Signing is done on-the-fly by the server when store paths are fetched. The user pushing store paths does not have access to the signing key.
- **Scalability**: Celler can be easily replicated. It's designed to be deployed to serverless platforms like fly.io but also works nicely in a single-machine setup.
- **Garbage Collection**: Unused store paths can be garbage-collected in an LRU manner.
