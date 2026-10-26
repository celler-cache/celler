# Security Model

## Authentication with Tokens

Clients authenticate with [JSON Web Tokens (JWTs)](https://jwt.io/introduction).
A token contains everything the server needs to know: who the user is, when the token expires, and which caches it may access with which permissions.
The server checks the token's signature and doesn't store anything about users or sessions.
That's why API servers can be [replicated](./architecture.md#server-components) freely.

Permissions are granted per cache name pattern, for example `--push 'alice-*'`.
A token only has the permissions it was created with.
For example, a token that can push and pull can't create caches.

The available permissions are: pull, push, delete store paths, create caches, configure caches, configure retention, and destroy caches.
See `celler admin make-token --help` in the [`celler` CLI reference](../reference/celler-cli.md).

Tokens are signed either with an RSA key pair (RS256) or with a shared secret (HS256).
With RS256, the server only has the public key, so a compromised server can't be used to create new tokens.

Because tokens are not stored on the server, you can't revoke a single token.
Give tokens a limited validity.

## Managed Signing

Nix only accepts store paths from a binary cache if they are signed by a key it trusts.
In Celler, each cache has its own signing key, which the server keeps.
The server signs store paths when they are downloaded.

Users who push to a cache never have access to the signing key.
They can add paths to the cache, but can't sign anything on their own.

## Isolation Between Caches

Caches on the same server don't trust each other.
The metadata of a store path, such as its name, references, and deriver, belongs to a single cache.
The global stores only contain data identified by its hash, without any metadata.

So if someone uploads a path with wrong or malicious metadata, only their own cache is affected.

## Proof of Possession

When a client uploads a NAR that the server already has, it still has to upload the full NAR.
The server hashes the data and throws it away instead of storing it again.

This prevents a user from getting access to a NAR in another cache just by knowing its hash.
Without this check, anyone who knows the hash of a private NAR on the server could add it to their own cache and download it.

You can turn this off with `require-proof-of-possession = false` in the [server configuration](../reference/server-config.md#top-level-options).
Uploads of NARs that the server already has then complete immediately.
Only do this if all users of the server trust each other.
