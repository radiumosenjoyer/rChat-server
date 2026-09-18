# RChat server

Rust server for the RChat communications API v1. SQLite stores public keys,
salted Argon2 invite hashes, token hashes, challenges, and opaque message
ciphertext. Message encryption and signature verification stay on clients.

## Run

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
It is possible to add automatic account deletion TTLs in the config.json file.

The server intentionally does not implement HPKE. It validates and stores the
wire envelope as opaque bytes; RFC 9180 encryption, message framing, raw-payload
signature verification, downgrade approval, and server certificate validation
belong to the client implementation.

If an existing config is malformed or contains invalid values, startup leaves
it untouched and tells you to restore it or delete it to generate a fresh copy.
Relative database, certificate, and private-key paths are resolved beside the
selected config file, so server and invite commands use the same files.
