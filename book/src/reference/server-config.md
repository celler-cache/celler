# Server Configuration

`cellerd` reads its configuration from a [TOML](https://toml.io) file.
It uses the file given with `-f`/`--config`.
Without that flag, it reads `$XDG_CONFIG_HOME/celler/server.toml` (usually `~/.config/celler/server.toml`).

On NixOS, you don't write this file yourself.
The `services.cellerd.settings` option generates it, using the same names as below.
See [Deploying to NixOS](../admin-guide/deployment/nixos.md).

To check a configuration file without starting the server, run:

```console
$ cellerd -f server.toml --mode check-config
```

Unknown options at the top level are rejected, so typos are caught early.

## Minimal Example

```toml
[database]
url = "sqlite:///var/lib/cellerd/server.db?mode=rwc"

[storage]
type = "local"
path = "/var/lib/cellerd/storage"

[jwt]
rs256-public-key-file = "/etc/cellerd/public-key.pem"

[chunking]
nar-size-threshold = 4194304  # 4 MiB
min-size = 2097152            # 2 MiB
avg-size = 8388608            # 8 MiB
max-size = 16777216           # 16 MiB
```

The sections `[database]`, `[storage]`, `[jwt]`, and `[chunking]` are required.
Everything else is optional.

## Top-Level Options

| Option | Type | Default | Description |
|---|---|---|---|
| `listen` | string | `"[::]:8080"` | Address and port to listen on. The `--listen` command-line flag overrides it. |
| `allowed-hosts` | list of strings | `[]` (all allowed) | Accepted values of the HTTP `Host` header. **Set this in production.** |
| `api-endpoint` | string | derived from the `Host` header | Public URL of the API, shown to clients. Must end with `/`, e.g. `"https://cache.example.com/"`. **Set this in production.** |
| `substituter-endpoint` | string | same as `api-endpoint` | Public URL of the binary cache that Nix downloads from, if different from `api-endpoint`. |
| `soft-delete-caches` | bool | `false` | Mark deleted caches as deleted instead of removing them from the database. The names of soft-deleted caches can't be reused. |
| `require-proof-of-possession` | bool | `true` | Require clients to upload the full data even if the server already has it. See [Security Model](../concepts/security.md#proof-of-possession). |
| `max-nar-info-size` | integer (bytes) | `1048576` (1 MiB) | Maximum size of the metadata a client sends with an upload. |

> [!WARNING]
> If `allowed-hosts` and `api-endpoint` are not set, the server trusts the `Host` header that clients send.
> Behind a reverse proxy, set both to your public hostname and URL.

## `[database]`

| Option | Type | Default | Description |
|---|---|---|---|
| `url` | string | **required** | Database connection URL. See below. |
| `heartbeat` | bool | `false` | Send a query to the database every minute to keep the connection alive. |

Celler supports SQLite and PostgreSQL:

```toml
# SQLite. mode=rwc creates the file if it doesn't exist.
url = "sqlite:///var/lib/cellerd/server.db?mode=rwc"

# PostgreSQL over a Unix socket
url = "postgresql:///cellerd?host=/run/postgresql"

# PostgreSQL over TCP
url = "postgresql://user:password@db.example.com/cellerd"
```

SQLite is enough for a single server.
Use PostgreSQL if you run several API servers.

## `[storage]`

Where Celler stores the cached data.
The `type` option selects the storage backend.

### Local Storage

| Option | Type | Default | Description |
|---|---|---|---|
| `type` | string | | `"local"` |
| `path` | string | **required** | Directory to store files in. |

```toml
[storage]
type = "local"
path = "/var/lib/cellerd/storage"
```

### S3 Storage

Works with Amazon S3 and S3-compatible services, such as Garage, MinIO, Cloudflare R2, and Backblaze B2.

| Option | Type | Default | Description |
|---|---|---|---|
| `type` | string | | `"s3"` |
| `region` | string | **required** | The region of the bucket. |
| `bucket` | string | **required** | The name of the bucket. |
| `endpoint` | string | AWS | URL of the S3 service. Set this for anything other than Amazon S3. |
| `credentials.access_key_id` | string | from environment | Access key ID. |
| `credentials.secret_access_key` | string | from environment | Secret access key. |

```toml
[storage]
type = "s3"
region = "us-east-1"
bucket = "my-celler-bucket"
endpoint = "https://s3.example.com"
```

If you leave out `credentials`, Celler reads them from the `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` environment variables.
This keeps secrets out of the configuration file.
On NixOS, put them in the file set by `services.cellerd.environmentFile`.

Note that the option names under `credentials` use underscores, unlike the other options.

To use an endpoint over plain `http://`, set the environment variable `AWS_ALLOW_HTTP=true`.

## `[jwt]`

How the server checks the tokens that clients send.
Set exactly one of `rs256-public-key-file` and `hs256-secret-key-file`.

| Option | Type | Default | Description |
|---|---|---|---|
| `rs256-public-key-file` | path | | PEM file with an RSA public key. Tokens must be signed with the matching private key. |
| `hs256-secret-key-file` | path | | File with a shared secret (raw bytes, not Base64). Tokens must be signed with the same secret. |
| `token-bound-issuer` | string | none | If set, tokens must have this `iss` (issuer) claim. |
| `token-bound-audiences` | list of strings | none | If set, tokens must have an `aud` (audience) claim that contains one of these values. |

RS256 is recommended: the server only has the public key, so a compromised server can't be used to create new tokens.
Create tokens with `celler admin make-token` and the matching private key or secret.

## `[chunking]`

How uploaded NARs are split into chunks for deduplication.
For recommended values and trade-offs, see [Chunking](../admin-guide/chunking.md).

All values are in bytes.
All four options are required, even if chunking is disabled.

| Option | Description |
|---|---|
| `nar-size-threshold` | Only chunk NARs that are at least this large. `0` disables chunking; `1` chunks all NARs. |
| `min-size` | Preferred minimum chunk size. |
| `avg-size` | Preferred average chunk size. |
| `max-size` | Preferred maximum chunk size. |

Changes only affect new uploads.

## `[compression]`

How chunks are compressed in storage.

| Option | Type | Default | Description |
|---|---|---|---|
| `type` | string | `"zstd"` | One of `"none"`, `"zstd"`, `"brotli"`, `"xz"`. |
| `level` | integer | depends on `type` | Compression level. Defaults: zstd 8, brotli 5, xz 2. |

Changing the compression only affects new uploads.
After changing `type`, new uploads don't reuse chunks stored with the old type, so deduplication is worse for a while.

## `[garbage-collection]`

| Option | Type | Default | Description |
|---|---|---|---|
| `interval` | duration | `"12 hours"` | How often to run garbage collection. `"0s"` disables automatic runs; you can still run `cellerd --mode garbage-collector-once`. |
| `default-retention-period` | duration | `"0s"` (keep forever) | Delete store paths that were neither uploaded nor downloaded for this long. Caches can override it with `celler cache configure --retention-period`. |

Durations are strings like `"30s"`, `"12 hours"`, `"3 months"`, or `"1y"`.

## Full Example

```toml
listen = "[::]:8080"
allowed-hosts = ["cache.example.com"]
api-endpoint = "https://cache.example.com/"

[database]
url = "postgresql:///cellerd?host=/run/postgresql"

[storage]
type = "s3"
region = "auto"
bucket = "celler"
endpoint = "https://<account-id>.r2.cloudflarestorage.com"

[jwt]
rs256-public-key-file = "/etc/cellerd/public-key.pem"

[chunking]
nar-size-threshold = 4194304  # 4 MiB
min-size = 2097152            # 2 MiB
avg-size = 8388608            # 8 MiB
max-size = 16777216           # 16 MiB

[compression]
type = "zstd"

[garbage-collection]
interval = "12 hours"
default-retention-period = "3 months"
```
