# RChat server

Rust server for the RChat communications API v1. SQLite stores public keys,
salted Argon2 invite hashes, token hashes, challenges, and opaque message
ciphertext. Message encryption and signature verification stay on clients.

This branch can run as a local TLS process against a SQLite file, or as an
HTTP-only origin on Vercel Fluid with a Turso (libSQL) remote database. The
edge terminates HTTPS; the origin does not speak TLS.

## Local process (SQLite + TLS)

On first run, the server creates `config.json` if it does not exist. Review its
TLS certificate paths, then:

```sh
cargo run --release keygen
cargo run --release invite add 'correct horse battery' --uses 1
cargo run --release serve
```

Override the configured listen address for one run with:

```sh
cargo run --release serve --host 0.0.0.0 --port 8443
```

`keygen` creates a self-signed certificate for `localhost` without overwriting
existing files. Use `--host chat.example` for the hostname clients connect to;
clients must explicitly trust the generated certificate.

TLS is restricted to TLS 1.3. For an isolated development network only, set
`"allow_insecure_http": true`. When TLS paths are configured, the same listener
accepts both HTTP and HTTPS; without TLS paths it accepts HTTP only. The server
refuses to start without TLS unless that flag is explicit.

```sh
cargo test
```

Tests always use a temporary local SQLite file.

It is possible to add automatic account deletion TTLs in the `config.json` file.

## Vercel + Turso (this branch)

Create a libSQL database in the Turso dashboard (the default engine, not
`--tursodb`). Copy the URL and a token. Never put the token in `config.json` or
commit it.

Set these on the Vercel project (and in the shell when running invite locally
against the same mailbox):

| Variable | Required | Purpose |
| --- | --- | --- |
| `TURSO_DATABASE_URL` | yes | libSQL URL, for example `libsql://<db>.turso.io` |
| `TURSO_AUTH_TOKEN` | yes | Turso auth token |
| `ACCOUNT_TTL_SECONDS` | no | Account expiry; `0` disables (same as config) |
| `MESSAGE_TTL_SECONDS` | no | Message ciphertext TTL; default 30 days |

Deploy this branch to Vercel. `vercel.json` rewrites `/v1/*` to the Rust
function in `api/index.rs` (`vercel_runtime` with Axum, Fluid compute). The
function is Hobby-friendly: it is an HTTP origin only. Clients connect with
HTTPS to the Vercel edge; TLS 1.3-only, if wanted, is a client requirement.

Point `invite` at the same database the function uses:

```sh
export TURSO_DATABASE_URL='libsql://<db>.turso.io'
export TURSO_AUTH_TOKEN='...'
cargo run --release invite add 'correct horse battery' --uses 1
cargo run --release invite revoke 'correct horse battery'
```

`serve` and `invite` share that backend whenever both Turso variables are set.
A long-lived `serve` still runs the expiry loop. On Vercel, expiry is the
existing on-request sweep only (`expire_accounts` / `expire_ephemeral`). Rate
limits stay in memory per instance.

If `PORT` is set (container or platform bind), `serve` listens on
`0.0.0.0:$PORT` over HTTP and does not load rustls.

The wire API is unchanged: `/v1/clients`, `/v1/auth/challenges`,
`/v1/auth/sessions`, `/v1/messages`, `/v1/messages/ack`.

The server intentionally does not implement HPKE. It validates and stores the
wire envelope as opaque bytes; RFC 9180 encryption, message framing, raw-payload
signature verification, downgrade approval, and server certificate validation
belong to the client implementation.

If an existing config is malformed or contains invalid values, startup leaves
it untouched and tells you to restore it or delete it to generate a fresh copy.
Relative database, certificate, and private-key paths are resolved beside the
selected config file, so server and invite commands use the same files when
Turso env vars are not set.
