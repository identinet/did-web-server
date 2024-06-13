# did-web-server

**did-web-server** is an HTTP server for providing [Decentralized Identifiers (DIDs)](https://w3c.github.io/did-core)
that conform to the [did:web](https://w3c-ccg.github.io/did-method-web/) method. did:web is a straightforward DID method
that uses DNS and HTTP servers to host DIDs.

Unlike other DID methods, did:web doesn't require a blockchain to store identifier states. It uses an HTTP server, which
is cost effective to operate and leverages established protocols and software libraries.

However, traditional HTTP servers are centrally managed and do not allow users to managed hosted documents, making
decentralized identifier management challenging. did-web-server addresses this issue by being the first DID-centered
HTTP server that enables self-sovereign management of identifiers by their controllers.

did-web-server leverages DIDs, [Verifiable Credentials (VC)](https://w3c.github.io/vc-data-model), and Verifiable
Presentations to authenticate identifiers and encode identifier data. No API tokens, usernames, or passwords are
required to interact with the service.

### Documentation

- User and API documentation is available at [https://dws.identinet.io]().
- Docker Container Image [did-web-server](https://hub.docker.com/r/identinet/did-web-server)

### Technology Stack

- [Rocket web framework](https://rocket.rs/)
- [Spruce Systems' SSI lib](https://github.com/spruceid/ssi/)

## Development

### Requirements

- [Rust](https://www.rust-lang.org/)
  - Or use [`rustup`](https://rustup.rs/) and install toolchain via: `rustup update stable`
- [`cargo-watch`](https://github.com/watchexec/cargo-watch)
- [`just`](https://just.systems/) task runner
- [`nu`](https://nushell.sh/) the best shell for CI tasks
- [Nixpkgs](https://nixos.org) for building reproduciable docker images

### Start Development

1. Continuously watch and build source files: `just dev`
2. Continuously watch and execute tests: `just dev-test`
3. Start editing the source files

### Build Release

- `just build`