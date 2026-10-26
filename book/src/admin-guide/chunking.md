# Chunking

Celler can split uploaded NARs into chunks with the [FastCDC
algorithm](https://www.usenix.org/conference/atc16/technical-sessions/presentation/xia),
so that data shared between NARs is stored only once. This saves storage
space, but makes Celler slower and can get it throttled by S3 providers.

> [!IMPORTANT]
> Only enable chunking if you need the storage savings. If you enable it, start with
> the [recommended values](#recommended-values).

All chunking options are described in the
[server configuration reference](../reference/server-config.md#chunking).

## Why Chunking Has a Cost

Without chunking, Celler can usually let clients download store paths
directly from the storage backend, using pre-signed URLs. The Celler server
doesn't have to handle the data itself.

With chunking, the Celler server downloads every chunk of a NAR from the
storage backend and puts the NAR back together for the client. This is much
more work for the server.

Every chunk is also a separate request to the storage backend. The smaller
the chunks, the more requests Celler sends. Hosted S3 providers limit how
many requests you can send. If Celler hits that limit, downloads become very
slow or fail.

## Disabling Chunking

Set `nar-size-threshold` to `0`:

```toml
[chunking]
nar-size-threshold = 0

# Not used while chunking is disabled, but required.
min-size = 2097152    # 2 MiB
avg-size = 8388608    # 8 MiB
max-size = 16777216   # 16 MiB
```

This only affects new uploads. NARs that are already chunked stay chunked.
Celler still stores identical NARs only once, even with chunking disabled.

## Recommended Values

These values have proven to work well in a stable setup:

```toml
[chunking]
# Only chunk NARs that are 4 MiB or larger.
nar-size-threshold = 4194304  # 4 MiB

min-size = 2097152            # 2 MiB
avg-size = 8388608            # 8 MiB
max-size = 16777216           # 16 MiB
```

## Using Smaller Chunks

You can experiment with smaller chunks. They deduplicate better, so Celler
needs less storage space. In exchange, Celler sends more requests to the
storage backend, which makes it slower. With a hosted S3 provider, small
chunks _will_ cause throttling.

If you run your own S3 server, such as [Garage](https://garagehq.deuxfleurs.fr/),
or use local storage, nobody throttles your requests. Smaller chunks can be
a good trade-off there, as long as the performance is good enough for you.

For example, these values use chunks that are about 64 times smaller than
the recommended ones:

```toml
[chunking]
nar-size-threshold = 131072  # 128 KiB
min-size = 65536             # 64 KiB
avg-size = 131072            # 128 KiB
max-size = 262144            # 256 KiB
```

Measure how fast downloads are and how much storage you save before you
settle on smaller values.

## Changing Values Later

Choose your values before you upload a lot of data. If you change them
later, new uploads are split at different points, so they share few chunks
with existing data. Deduplication will be worse for a while.
