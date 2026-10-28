# Introduction

**Celler** is a self-hosted binary cache for Nix.
Push your build results to it, and Nix on your other machines and in CI downloads them instead of building them again.

Celler is working towards a stable release.
Until then, the configuration and APIs may still change between releases.
We are looking for more testers!

> [!IMPORTANT]
> **Upgrading from Attic or an older Celler?** The configuration format has changed.
> See the [changelog](https://github.com/celler-cache/celler/blob/main/CHANGELOG.md) and the [Attic migration guide](admin-guide/attic-migration.md).

## Features

- **Works with plain Nix**: Nix pulls from a Celler cache like from any other binary cache.
  The `celler` client pushes store paths and sets up Nix for you.
- **Many caches, one server**: Create a private cache for yourself, one for your team, and a public one for your project.
  Caches are [isolated](concepts/security.md#isolation-between-caches): what happens in one cache can't affect the others.
- **Stores everything once**: If the same data is pushed to several caches, Celler stores it only once.
  Optional [chunking](admin-guide/chunking.md) also deduplicates data shared between similar store paths.
- **Fits any size**: From a single-machine setup with local storage to several API servers in front of S3-compatible storage, Celler scales to any size.
- **Signing keys stay on the server**: The server signs store paths when Nix downloads them.
  People who push to a cache never see its signing key.
- **Automatic cleanup**: Celler can delete store paths that haven't been used for a while.
  You choose how long to keep them, per cache.

## Where to Go Next

- **Try it out**: The [Tutorial](tutorial.md) runs a Celler server on your machine in about 15 minutes.
- **Use a cache someone else runs**: See the [User Guide](user-guide/index.md).
- **Run your own server**: See the [Admin Guide](admin-guide/index.md).

## About the Name

_Celler_ is Catalan for a wine cellar, the place where you store all your valuable wines.
Celler is where you store your valuable build results.

And of course, Celler is a fork of [Attic](https://github.com/zhaofengli/attic), the self-hosted Nix binary cache by Zhaofeng Li.
