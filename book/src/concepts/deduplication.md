# Deduplication and Compression

Nix stores lots of similar data.
Two versions of the same package often differ in only a few files, and the same package often ends up in several caches.
Celler deduplicates on two levels so it stores this data only once.

## NAR-Level Deduplication

Every NAR is identified by its hash.
If someone uploads a NAR that the server already has, in any cache, Celler doesn't store it again.
It only adds a reference from the cache to the existing NAR.

This happens on the server and is invisible to the client.
By default, the client still has to upload the full NAR to prove that it has the data.
See [Proof of Possession](./security.md#proof-of-possession).

## Chunk-Level Deduplication

If chunking is enabled, new NARs are split into chunks with the [FastCDC algorithm](https://www.usenix.org/conference/atc16/technical-sessions/presentation/xia).
FastCDC places chunk boundaries based on the content, so a small change in a file only affects the chunks around the change.
Chunks that already exist are not stored again.

When Nix downloads a chunked NAR, `cellerd` reads its chunks from the storage backend and streams them to the client as one NAR.

NARs smaller than `nar-size-threshold` are stored as a single chunk; NAR-level deduplication still applies to them.

Chunking is optional and costs performance.
See [Chunking](../admin-guide/chunking.md) for the trade-offs.

### Why Chunk Whole NARs Instead of Individual Files?

Celler splits the entire uncompressed NAR, ignoring the boundaries of the files inside it.

The NARs that benefit the most from deduplication, such as VSCode or Zoom, often contain thousands of small files.
Storing each file separately would mean fetching thousands of objects from the storage backend to rebuild a single NAR, which is slow and expensive.
Chunking the whole NAR lets small files share a chunk and keeps the number of chunks manageable.
[`casync`](https://github.com/systemd/casync) takes the same approach.

[The Tvix store protocol](https://flokli.de/posts/2022-06-30-store-protocol/), in contrast, chunks individual files.
Celler's design is driven by making good use of existing platforms, such as S3, and their practical limits.

## Compression

The server compresses each chunk before writing it to the storage backend (zstd by default, see [`[compression]`](../reference/server-config.md#compression)).
Deduplication uses the hash of the _uncompressed_ chunk, together with the compression type.
If you change the compression type, new uploads don't reuse chunks stored with the old type.
NAR-level deduplication isn't affected.

```
                        ┌───────────────────────────────────►Chunk Hash
                        │
                        │
                        ├───────────────────────────────────►Chunk Size
                        │
                ┌───────┴────┐  ┌──────────┐  ┌───────────┐
 Chunk Stream──►│Chunk Hasher├─►│Compressor├─►│File Hasher├─►File Stream─►S3
                └────────────┘  └──────────┘  └─────┬─────┘
                                                    │
                                                    ├───────►File Hash
                                                    │
                                                    │
                                                    └───────►File Size
```

## Missing or Corrupt Chunks

If a chunk is deleted from the database, every NAR that uses it becomes unavailable and the server responds with an error (503).
This fixes itself once someone uploads any NAR that contains the chunk again.

At the moment, Celler can't detect on its own whether a chunk in the storage backend is corrupt or missing.
Telling temporary storage errors apart from permanent ones is difficult.
