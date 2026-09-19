# RChat server

Rust mailbox for the RChat communications API v1. Turso (libSQL) stores public
keys, salted Argon2 invite hashes, token hashes, challenges, and opaque message
ciphertext. Message encryption and signature verification stay on clients.

This branch deploys as an HTTP origin on Vercel Fluid. The edge terminates
HTTPS. The origin does not speak TLS.

## Deploy

Create a libSQL database in the Turso dashboard (the default engine, not
`--tursodb`). Copy the URL and a token. Never commit the token.

Set these on the Vercel project:

| Variable | Required | Purpose |
| --- | --- | --- |
| `TURSO_DATABASE_URL` | yes | libSQL URL, for example `libsql://<db>.turso.io` |
| `TURSO_AUTH_TOKEN` | yes | Turso auth token |
| `ACCOUNT_TTL_SECONDS` | no | Account expiry. `0` disables it. |
| `MESSAGE_TTL_SECONDS` | no | Message ciphertext TTL. Default 30 days. |

Deploy this branch. `vercel.json` rewrites `/v1/*` to the Rust function in
`api/index.rs` (`vercel_runtime` with Axum). The function is Hobby-friendly: it
is HTTP only. Clients connect with HTTPS to the Vercel edge. TLS 1.3-only, if
you want that, is a client requirement.

Expiry is the on-request sweep (`expire_accounts` / `expire_ephemeral`). Rate
limits stay in memory per instance.

## Invites

Point `invite` at the same Turso database the function uses:

```sh
export TURSO_DATABASE_URL='libsql://<db>.turso.io'
export TURSO_AUTH_TOKEN='...'
cargo run --release --bin rchat-server invite add 'correct horse battery' --uses 1
cargo run --release --bin rchat-server invite revoke 'correct horse battery'
```

## API

`/v1/clients`, `/v1/auth/challenges`, `/v1/auth/sessions`, `/v1/messages`,
`/v1/messages/ack`.

The server does not implement HPKE. It validates and stores the wire envelope
as opaque bytes. RFC 9180 encryption, message framing, raw-payload signature
verification, downgrade approval, and server certificate validation belong to
the client.

```sh
cargo test
```
