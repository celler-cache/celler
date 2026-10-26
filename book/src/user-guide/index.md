# User Guide

## Logging In

You should have received an `celler login` command from an admin like the following:

```
celler login central https://celler.domain.tld/ eyJ...
```

The `celler` client can work with multiple servers at the same time.
To select the `foo` cache from server `central`, use one of the following:

- `foo`, if the `central` server is configured as the default
- `central:foo`

To configure the default server, set `default-server` in `~/.config/celler/config.toml`.

## Enabling a Cache

To configure Nix to automatically use cache `foo`:

```
celler use foo
```

This adds the binary cache to your `~/.config/nix/nix.conf` and configures the credentials required to access it.

If you wish to configure Nix manually, you can view the binary cache endpoint and the cache public key:

```console
$ celler cache info foo
               Public: true
           Public Key: foo:WcnO6s4aVkB6CKRaPPpKvHLZykWXASV6c+/Ssg8uQEY=
Binary Cache Endpoint: https://celler.domain.tld/foo
      Store Directory: /nix/store
             Priority: 41
  Upstream Cache Keys: ["cache.nixos.org-1"]
     Retention Period: Global Default
```

On NixOS, you can configure the cache declaratively in your system configuration with the above information:

```nix
{
  nix.settings = {
    substituters = [
      "https://celler.domain.tld/foo"
    ];
    trusted-public-keys = [
      "foo:WcnO6s4aVkB6CKRaPPpKvHLZykWXASV6c+/Ssg8uQEY="
    ];
  };
}
```

## Disabling a Cache

To configure Nix to no longer use a cache, remove the corresponding entries from the list of `substituters` and `trusted-public-keys` in `~/.config/nix/nix.conf`

## Pushing to the Cache

To push a store path to cache `foo`:

```bash
celler push foo /nix/store/...
```

Other examples include:

```bash
celler push foo ./result
celler push foo /run/current-system
```
