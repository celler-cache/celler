# Deploying to NixOS

Celler provides [a NixOS module](https://github.com/celler-cache/celler/blob/main/nixos/cellerd.nix) that allows you to deploy the Celler Server on a NixOS machine.

## Prerequisites

1. A machine running NixOS
1. _(Optional)_ A dedicated bucket on S3 or a S3-compatible storage service
    - You can either [set up Garage](https://search.nixos.org/options?query=services.garage) or use a hosted service like [Backblaze B2](https://www.backblaze.com/b2/docs) and [Cloudflare R2](https://developers.cloudflare.com/r2).
1. _(Optional)_ A PostgreSQL database

## Generating the Credentials File

The RS256 JWT secret can be generated with the `openssl` utility:

```bash
$ nix shell nixpkgs#openssl
$ openssl genrsa -out private-key.pem 4096
$ openssl rsa -in private-key.pem -pubout -out public-key.pem
```

Keep the private-key.pem file in a secure location. You need it to sign tokens using `celler admin make-token`.

The public key will be part of the server configuration.

## Importing the Module

You can import the module in one of two ways:

- Ad-hoc: Import the `nixos/cellerd.nix` from [the repository](https://github.com/celler-cache/celler).
- Flakes: Add `github:celler-cache/celler` as an input, then import `celler.nixosModules.cellerd`.

## Configuration

> Note: These options are subject to change.

```nix
{
  services.cellerd = {
    enable = true;

    # Replace with absolute path to your environment file
    environmentFile = "/etc/cellerd.env";

    settings = {
      listen = "[::]:8080";

      jwt = {
        rs256-public-key-file = ./public-key.pem;
      };

      # Chunking is off by default. To enable it, see the Chunking page.
    };
  };
}
```

`settings` accepts all options from the [server configuration reference](../../reference/server-config.md).
Unless you configure them, the module uses SQLite and local storage in `/var/lib/cellerd`.

After the new configuration is deployed, the Celler Server will be accessible on port 8080.
It's highly recommended to place it behind a reverse proxy like [NGINX](https://nixos.wiki/wiki/Nginx) to provide HTTPS.
