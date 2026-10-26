# FAQs

## Does It Replace [Cachix](https://www.cachix.org)?

No, it does not.
Cachix is an awesome product and the direct inspiration for the user experience of Celler.
It works at a much larger scale than Celler and is a proven solution.
Numerous open-source projects in the Nix community use Cachix to share publicly-available binaries.

Celler provides a similar user experience at a much smaller scale (personal or team use).

## What Happens If a User Uploads a Path That Is Already on the Server?

The user still uploads the full path to prove they have it, but the server doesn't store it again.
It adds a reference from the user's cache to the existing data.
See [Deduplication and Compression](concepts/deduplication.md) and [Proof of Possession](concepts/security.md#proof-of-possession).

## What Happens If a User Uploads a Path with Incorrect or Malicious Metadata?

It only affects their own cache.
See [Isolation Between Caches](concepts/security.md#isolation-between-caches).

## How Is Authentication Handled?

With signed tokens (JWTs) that contain the user's permissions.
See [Authentication with Tokens](concepts/security.md#authentication-with-tokens).

## How Does Deduplication Work?

On two levels: whole NARs and chunks of NARs.
See [Deduplication and Compression](concepts/deduplication.md).

## What Happens If a Chunk Is Corrupt or Missing?

NARs that contain it become unavailable until someone uploads the chunk again.
See [Missing or Corrupt Chunks](concepts/deduplication.md#missing-or-corrupt-chunks).
