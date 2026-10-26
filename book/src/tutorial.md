# Tutorial

In this tutorial, you run a Celler server on your own machine, create a binary cache, push a store path to it, and pull it back.
You don't need NixOS, S3, or a database server: everything lives in a single directory.

You need:

- [Nix](https://nixos.org/download/) with [flakes enabled](https://nixos.wiki/wiki/Flakes#Enable_flakes_permanently_in_NixOS)
- Two terminal windows (or tabs)

This setup is for trying Celler out.
To run Celler for real, follow [Deploying to NixOS](admin-guide/deployment/nixos.md) afterwards.

## Get Celler

You use two terminals in this tutorial:

- The **server terminal** runs the Celler server.
- The **client terminal** is where you run all other commands.

Open both terminals, and in each one, start a shell that has `celler`, `cellerd`, and `openssl`:

```console
$ nix shell github:celler-cache/celler nixpkgs#openssl
```

## Start the Server

In the **server terminal**, create a directory for the server's data:

```console
$ mkdir celler-demo
$ cd celler-demo
```

Celler uses signed tokens ([JWTs](https://jwt.io/introduction)) to decide who may do what.
Generate a key pair to sign them.
The server only needs the public key; you keep the private key to create tokens:

```console
$ openssl genrsa -out private-key.pem 4096
$ openssl rsa -in private-key.pem -pubout -out public-key.pem
writing RSA key
```

Create `server.toml` with the following content:

```toml
listen = "[::]:8080"

# Keep the database in a SQLite file in this directory.
[database]
url = "sqlite://server.db?mode=rwc"

# Store the cached files in the storage/ directory.
[storage]
type = "local"
path = "storage"

# Accept tokens signed with our private key.
[jwt]
rs256-public-key-file = "public-key.pem"

# Split large files into chunks for deduplication.
# These are the recommended values.
[chunking]
nar-size-threshold = 4194304  # 4 MiB
min-size = 2097152            # 2 MiB
avg-size = 8388608            # 8 MiB
max-size = 16777216           # 16 MiB
```

All options are described in the [server configuration reference](reference/server-config.md).

Start the server:

```console
$ cellerd -f server.toml
Celler 0.1.0 (release)
Running migrations...
* Migrating NARs to chunks...
* Migrating NAR schema...
Starting API server...
Listening on [::]:8080...
```

Leave the server running.
Use the **client terminal** for the rest of the tutorial.

## Create a Cache

In the client terminal, go to the `celler-demo` directory, because it contains the private key.
Then create a token for a user called `alice`.
It allows her to create the cache `hello`, configure it, push to it, and pull from it:

```console
$ cd celler-demo
$ celler admin make-token --signing-key private-key.pem --sub alice --validity '3 months' \
    --pull hello --push hello \
    --create-cache hello --configure-cache hello --configure-cache-retention hello
eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ...
```

Log in to the server with this token.
`local` is the name the client uses to refer to this server:

```console
$ celler login local http://localhost:8080 eyJ...
✍️ Configuring server "local"
```

Now create the cache:

```console
$ celler cache create hello
✨ Created cache "hello" on "local"
```

## Push to the Cache

Push the `celler` package itself to the cache:

```console
$ celler push hello $(which celler)
⚙️ Pushing 1 paths to "hello" on "local" (0 already cached, 6 in upstream)...
✅ h6pjy7d28mj6wihhn4fz2kqx7x9wy4my-celler-0.1.0 (50.09 MiB, 21.54 MiB/s)
```

`celler push` uploads the store path together with everything it depends on (its _closure_).
Here, it skipped six dependencies because they are already available from `cache.nixos.org` ("in upstream").
You can change this behavior per cache.

## Pull from the Cache

Configure Nix to use the new cache:

```console
$ celler use hello
Configuring Nix to use "hello" on "local":
+ Substituter: http://localhost:8080/hello
+ Trusted Public Key: hello:HE3fhNGlAymSg6uGGrMMZGC20Of10/p8aZClxFzJ4rE=
+ Access Token
```

This adds the cache, its public key, and your access token to `~/.config/nix/nix.conf` and `~/.config/nix/netrc`.

`celler` is already in your Nix store, so Nix would not download it again.
To see the download happen, let Nix fetch it into a separate store in the `nix-demo` directory:

```console
$ nix-store --store $PWD/nix-demo -r $(which celler)
copying path '/nix/store/h6pjy7d28mj6wihhn4fz2kqx7x9wy4my-celler-0.1.0' from 'http://localhost:8080/hello'...
warning: you did not specify '--add-root'; the result might be removed by the garbage collector
/nix/store/h6pjy7d28mj6wihhn4fz2kqx7x9wy4my-celler-0.1.0
```

The `copying path ... from 'http://localhost:8080/hello'` line shows that Nix downloaded it from your cache.

> [!NOTE]
> To pull into your real `/nix/store` on a multi-user Nix installation, your user must be [trusted](https://nix.dev/manual/nix/stable/command-ref/conf-file.html#conf-trusted-users) by the Nix daemon, or the cache must be configured system-wide.
> See [the User Guide](user-guide/index.md#enabling-a-cache).

## Make the Cache Public

So far, only someone with a token can read from the cache:

```console
$ curl http://localhost:8080/hello/nix-cache-info
{"code":401,"error":"Unauthorized","message":"Unauthorized."}
```

Make it public, so that anyone can pull from it without a token:

```console
$ celler cache configure hello --public
✅ Configured "hello" on "local"

$ curl http://localhost:8080/hello/nix-cache-info
WantMassQuery: 1
StoreDir: /nix/store
Priority: 41
```

Pushing still requires a token with push permission.

## Clean Up Old Paths

Without limits, a binary cache grows forever.
You can set a _retention period_: store paths that nobody has pushed or pulled for that long are deleted by the garbage collector.

Set the retention period to one second, so we don't have to wait:

```console
$ celler cache configure hello --retention-period '1s'
✅ Configured "hello" on "local"
```

The server runs garbage collection every 12 hours.
Instead of waiting, run it once by hand:

```console
$ cellerd -f server.toml --mode garbage-collector-once
```

The store path is gone from the cache:

```console
$ curl http://localhost:8080/hello/h6pjy7d28mj6wihhn4fz2kqx7x9wy4my.narinfo
{"code":404,"error":"NoSuchObject","message":"The requested object does not exist."}
```

Reset the retention period to the server default, which is to keep everything:

```console
$ celler cache configure hello --reset-retention-period
✅ Configured "hello" on "local"
```

## Clean Up

When you are done, stop the server with <kbd>Ctrl</kbd>+<kbd>C</kbd> in the server terminal.
Then remove the `celler-demo` directory.
Nix makes the files in `nix-demo` read-only, so make them writable first:

```console
$ cd ..
$ chmod -R u+w celler-demo
$ rm -rf celler-demo
```

`celler use` changed your Nix configuration.
Remove the `http://localhost:8080/hello` substituter and the `hello:...` key from `~/.config/nix/nix.conf`.
Otherwise Nix keeps trying to reach the stopped server.

If you do the tutorial again, the new cache gets a new signing key, but Nix may still remember the old one for a while.
If Nix reports that a path is `not signed by any of the keys in 'trusted-public-keys'`, delete `~/.cache/nix`.

## Summary

You have:

1. Run a Celler server with local storage
2. Created a token and a cache
3. Pushed a store path and pulled it back with Nix
4. Made the cache public
5. Removed old paths with garbage collection

## What's Next

- [User Guide](user-guide/index.md): use caches that someone else runs.
- [Deploying to NixOS](admin-guide/deployment/nixos.md): run Celler as a service.
- [Concepts](concepts/index.md): how Celler stores, deduplicates, and protects your data.
- [`celler admin make-token --help`](reference/celler-cli.md): all token permissions, including wildcards like `--push 'alice-*'`.
