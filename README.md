# web-id Server

The web-id Server provides an HTTP server for managing DID Documents for the
[did:web method](https://w3c-ccg.github.io/did-method-web/). did:web is a simple
DID method that relies on DNS and HTTP servers to provide DID Documents that
represent a self-sovereign identities. Unlike many other DID methods, did:web
doesn't not require a blockchain to operate it. The aim of the web-id Server is
to bring the elements of self-sovereign identity management to did:web by
allowing the owner of the identity to rotate keys and perform other activities
on the DID.

## API

| **Functionality**     | **Method** | **Path**                                     | **Return Codes**                                                                   |
| --------------------- | ---------- | -------------------------------------------- | ---------------------------------------------------------------------------------- |
| Retrieve DID document | GET        | `/<id>/did.json`<br> `/v1/web/<id>/did.json` | `200` OK<br> `400` Bad Request<br> `404` Not Found<br> `500` Internal Server Error |
| Create DID document   | POST       | `/v1/web/<id>/did.json`                      | `200`<br> `400`                                                                    |
| Update DID document   | PUT        | `/v1/web/<id>/did.json`                      | `200`<br> `400`                                                                    |
| Delete DID document   | DELETE     | `/v1/web/<id>/did.json`                      | `200`<br> `400`                                                                    |

## Technology Stack

- [Rocket web framework](https://rocket.rs/)
- [SSI Lib](https://github.com/spruceid/ssi/)

## Architecture

```plantuml
title "web-id Server Context"

!include <C4/C4_Context.puml>
!include <tupadr3/font-awesome/server>
!include <tupadr3/font-awesome/user>
!include <tupadr3/font-awesome/user_o>
!include <tupadr3/font-awesome/building>
!include <tupadr3/font-awesome/cogs>

Person(admin, "Administrator", "Administrator of the did:web server", $sprite="user_o")
Person(user, "User", "did:web identity holder", $sprite="user")
System_Ext(wallet, "User's wallet", "Universal Identity Wallet", $sprite="cogs")
System(system, "web-id Server", "web-id Server server did:web identities", $sprite="server")
System_Ext(thirdpartysystem, "DID Resolver", "Universal DID Resolver", $sprite="server")

Rel(user, wallet, "manages", "")
Rel(wallet, system, "manages personal DID", "")
Rel(admin, system, "creates and removes DIDs", "")
Rel(thirdpartysystem, system, "resolves DIDs", "")

SHOW_LEGEND()
```

## Implementation Instructions

## Installation

- Download binary
- Set the [environment variables](#configuration) and start the service, e.g.:

```sh
EXTERNAL_HOSTNAME=example.com EXTERNAL_PORT=3000 EXTERNAL_PATH=/dids DID_STORE=/tmp/did_store ./web-id-server
```

## Configuration

Set the following environment variables according to the requirements:

| **Environment Variable Name** | **Description**                                            | **Required** | **Default**                                            | **Example**             |
| ----------------------------- | ---------------------------------------------------------- | ------------ | ------------------------------------------------------ | ----------------------- |
| `EXTERNAL_HOSTNAME`           | External DNS domain name that the server can be reached at | yes          | `localhost`                                            | `example.com`           |
| `EXTERNAL_PORT`               | External port that the server can be reached at            | no           | `443` if `$HOSTNAME != "localhost"`, otherwise `$PORT` | `3000`                  |
| `EXTERNAL_PATH`               | External path that the DIDs shall be served at             | yes          | `/`                                                    | `/dids`                 |
| `DID_STORE`                   | Path to the directory that holds the JSON DID files        | yes          | `$PWD/did_store`                                       | `/usr/web-id/did_store` |

<!-- | not yet implemented `PORT`    | Port that the service operates on                          | yes          | `8080`                                                 | `80`                    | -->

## Development

### Requirements

- [`rustup`](https://rustup.rs/)
  - Install rust toolchain: `rustup update stable`
- [`cargo-watch`](https://github.com/watchexec/cargo-watch)
- [`just`](https://just.systems/)

### Start Development

1. Continuously watch and build source files: `just dev`
2. Continuously watch and execute tests: `just dev-test`
3. Start editing the source files

### Build Release

- `just build`
