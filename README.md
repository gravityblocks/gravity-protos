# gravity-protos

Searcher protos for GBX, backwards compatible with the
[Jito](https://github.com/jito-labs/mev-protos) block engine interface.
Unsupported endpoints, including transaction submission and token refresh, are
intentionally omitted.

## Access

Access is limited to authorised searchers.

- The only supported role is `SEARCHER = 1`.
- Authenticate through `AuthService` and pass the access token as
  `authorization: Bearer <token>` on subscription requests.

## Streams

- `SubscribeMempool`: pending transactions tagged with the originating connector's identity.
- `SubscribeBundles`: pending bundles tagged with the connector identity and bundle ID. Transaction signatures are
  stripped (zeroed) before bundles are streamed, so they can be inspected but
  not landed independently.

## Development

Schemas live in `protos/`. Generated Rust bindings are checked in under
`src/generated`, so normal builds do not require `protoc`:

```sh
cargo build
```

After editing `protos/*.proto`, install `protoc` and regenerate the bindings:

```sh
REGENERATE_PROTO=1 cargo build
```

CI follows `solana-protos`: Rust formatting and compilation checks, Buf schema
validation, and wire/JSON compatibility checks for the supported subset of
`jito-labs/mev-protos`.

```sh
cargo fmt --all -- --check
cargo check --all-targets --all-features
buf build protos
buf breaking protos --against 'https://github.com/jito-labs/mev-protos.git#branch=master'
```
