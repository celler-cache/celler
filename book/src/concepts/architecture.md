# Architecture

## Caches Are Views of a Global Store

A Celler server hosts many binary caches.
Each cache looks like an ordinary Nix binary cache to Nix, but the caches don't store data themselves.
All data lives in two global stores shared by every cache on the server:

```
  Cache "alice"      Cache "team"      Cache "ci"        ← store path metadata, per cache
       │  │              │                 │
       ▼  ▼              ▼                 ▼
  ┌──────────────────────────────────────────────┐
  │              Global NAR Store                │       ← one entry per unique NAR
  └──────────────────────────────────────────────┘
              │          │          │
              ▼          ▼          ▼
  ┌──────────────────────────────────────────────┐
  │             Global Chunk Store               │       ← the actual bytes, in S3 or on disk
  └──────────────────────────────────────────────┘
```

- **Cache**: holds the metadata of the store paths pushed to it: store path, references, deriver, and so on.
  Each entry points to a NAR in the global NAR store.
- **Global NAR Store**: one entry per unique NAR (the archive format Nix uses for store paths), identified by its hash.
  A NAR is made of one or more chunks.
- **Global Chunk Store**: the stored data.
  Chunks are identified by their hash and stored in the configured storage backend.

When you push a store path that already exists on the server, possibly in another cache, Celler only adds a reference from your cache to the existing NAR.
See [Deduplication and Compression](./deduplication.md).

Because the global stores contain only content-addressed data without any metadata, caches can't affect each other.
See [Security Model](./security.md).

## Server Components

`cellerd` has two long-running components:

- **API server**: serves the binary caches to Nix, and handles uploads and cache management from the `celler` client.
  It keeps no state of its own, so you can run several API servers behind a load balancer.
- **Garbage collector**: periodically deletes data that is no longer needed.
  Only run one garbage collector per server.

Both use a database (SQLite or PostgreSQL) for metadata, and a storage backend (local directory or S3) for chunks.

By default, `cellerd` runs in `monolithic` mode, which runs both components in one process.
For bigger setups, run them separately with `--mode api-server` and `--mode garbage-collector`.
With several API servers, use PostgreSQL and S3, so that all servers share the same database and storage.

## Garbage Collection

Each cache can have a retention period (or use the server's `default-retention-period`).
Because the data is shared between caches, garbage collection happens in three steps:

1. **Cache**: Store paths that were neither pushed nor pulled within the retention period are removed from the cache.
   This only removes the reference to the NAR; no storage is freed yet.
2. **Global NAR Store**: NARs that no cache references anymore are deleted.
3. **Global Chunk Store**: Chunks that no NAR references anymore are deleted from the storage backend.
   Only this step frees storage space.

A NAR that was removed from one cache stays available in all other caches that reference it.
