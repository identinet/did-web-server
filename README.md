# did-web Server

The did-web server provides an HTTP server for managing DID Documents for the
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
EXTERNAL_HOSTNAME=example.com EXTERNAL_PORT=3000 EXTERNAL_PATH=/dids DID_STORE=/tmp/did_store ./did-web-server
```

## Configuration

Set the following environment variables according to the requirements:

| **Environment Variable Name** | **Description**                                                                                                     | **Required** | **Default**                                            | **Example**                                     |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------ | ------------------------------------------------------ | ----------------------------------------------- |
| `EXTERNAL_HOSTNAME`           | External DNS domain name that the server can be reached at                                                          | yes          | `localhost`                                            | `example.com`                                   |
| `EXTERNAL_PORT`               | External port that the server can be reached at                                                                     | no           | `443` if `$HOSTNAME != "localhost"`, otherwise `$PORT` | `3000`                                          |
| `EXTERNAL_PATH`               | External path that the DIDs shall be served at                                                                      | yes          | `/`                                                    | `/dids`                                         |
| `DID_STORE`                   | Path to the directory that holds the JSON DID files                                                                 | yes          | `$PWD/did_store`                                       | `/usr/web-id/did_store`                         |
| `DID_RESOLVER_OVERRIDE`       | DID HTTP Resolver compatible with https://w3c-ccg.github.io/did-resolution/                                         | yes          | `http://uni-resolver-web:8080/1.0/identifiers/`        | `http://uni-resolver-web:8080/1.0/identifiers/` |
| `ROCKET_PORT`                 | Port that the service operates at                                                                                   | yes          | `8000`                                                 | `3000`                                          |
| `ROCKET_TLS`                  | Key and certificate for serving a HTTPS/TLS secured service                                                         | no           |                                                        | `{certs="my.crt", key="private.key"}`           |
| `ROCKET_XXX`                  | Rocket offers more configuration settings, see https://rocket.rs/v0.5-rc/guide/configuration/#environment-variables | no           |                                                        |                                                 |

## Usage

### Start Server

`./did-web-server`

### Create DID

Either:

- Store valid [DID document](https://w3c.github.io/did-core/#did-documents) in
  the configured DID store directory, e.g. `./did_store/valid.json`
- (not yet implemented) Or use the admin API endpoint to create a DID, e.g.
  `curl -X POST -H "Authentication: Bearer XYZ" -d @diddoc.json http://localhost:8000/valid/did.json`

### Query DID

`curl http://localhost:8000/valid/did.json`

### Update DID

Start the resolver services:

`just dev-compose`

Query proof parameters:

- `curl 'http://localhost:8000/valid/did.json?proofParameters'`
- Take note of the challenge and domain values

Prepare presentation with a DID document credential:

- Install [didkit](https://www.spruceid.dev/didkit/didkit/installation)
- Create key: `didkit generate-ed25519-key > key.jwk`
- Prepare credential (attention, the issuer DID must exist already). Create file
  `credential.json` with the following content:

```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1"
  ],
  "id": "uuid:49387f58-c0d9-4b14-a4f4-bc31a021d925",
  "type": ["VerifiableCredential"],
  "issuer": "did:web:web-did-server%3A8000:valid",
  "issuanceDate": "2010-01-01T00:00:00Z",
  "credentialSubject": {
    "@context": { "did": "https://www.w3.org/ns/did/v1#" },
    "did:id": "did:web:web-did-server%3A8000:valid",
    "did:verificationMethod": [{
      "@context": {
        "sec": "https://w3id.org/security/v2#",
        "jwk2020": "https://w3c.github.io/vc-jws-2020/contexts/v1#"
      },
      "id": "did:web:web-did-server%3A8000:valid#controller",
      "type": "did:Ed25519VerificationKey2018",
      "sec:controller": "did:web:web-did-server%3A8000:valid",
      "jwk2020:publicKeyJwk": {
        "jwk2020:kty": "OKP",
        "jwk2020:crv": "Ed25519",
        "jwk2020:x": "IkMQNWwqe-y6KEpIjP2DOKbxF9cqrhur6o-l4OJ8AwA"
      }
    }],
    "did:authentication": [
      "did:web:web-did-server%3A8000:valid#controller"
    ],
    "did:assertionMethod": [
      "did:web:web-did-server%3A8000:valid#controller"
    ],
    "did:service": [{
      "id": "did:web:web-did-server%3A8000:valid#linked-domain",
      "did:serviceEndpoint": "https://bar.example.com"
    }]
  }
}
```

- Issue/sign credential:
  `DID_RESOLVER_OVERRIDE=http://localhost:8080/1.0/identifiers/ didkit vc-issue-credential -k key.jwk -p assertionMethod -t Ed25519Signature2018 -v 'did:web:web-did-server%3A8000:valid#controller' < credential.json > credential-signed.json`
- Ensure credential is valid:
  `didkit vc-verify-credential -R http://localhost:8080/1.0/identifiers/ < credential-signed.json`
- Prepare presentation (attention, the issuer DID must exist already). Create
  file `presentation.json` with the following content:

```json
{
  "@context": "https://www.w3.org/2018/credentials/v1",
  "id": "uuid:931538ea-c23d-46c1-b387-69d3ccdd1424",
  "type": ["VerifiablePresentation"],
  "holder": "did:web:web-did-server%3A8000:valid",
  "verifiableCredential": {
    "@context": ["https://www.w3.org/2018/credentials/v1"],
    "id": "uuid:49387f58-c0d9-4b14-a4f4-bc31a021d925",
    "type": ["VerifiableCredential"],
    "credentialSubject": {
      "@context": { "did": "https://www.w3.org/ns/did/v1#" },
      "did:authentication": ["did:web:web-did-server%3A8000:valid#controller"],
      "did:assertionMethod": ["did:web:web-did-server%3A8000:valid#controller"],
      "did:service": [
        {
          "did:serviceEndpoint": "https://bar.example.com",
          "id": "did:web:web-did-server%3A8000:valid#linked-domain"
        }
      ],
      "did:id": "did:web:web-did-server%3A8000:valid",
      "did:verificationMethod": [
        {
          "@context": {
            "jwk2020": "https://w3c.github.io/vc-jws-2020/contexts/v1#",
            "sec": "https://w3id.org/security/v2#"
          },
          "id": "did:web:web-did-server%3A8000:valid#controller",
          "jwk2020:publicKeyJwk": {
            "jwk2020:crv": "Ed25519",
            "jwk2020:kty": "OKP",
            "jwk2020:x": "IkMQNWwqe-y6KEpIjP2DOKbxF9cqrhur6o-l4OJ8AwA"
          },
          "sec:controller": "did:web:web-did-server%3A8000:valid",
          "type": "did:Ed25519VerificationKey2018"
        }
      ]
    },
    "issuer": "did:web:web-did-server%3A8000:valid",
    "issuanceDate": "2010-01-01T00:00:00Z",
    "proof": {
      "type": "Ed25519Signature2018",
      "proofPurpose": "assertionMethod",
      "verificationMethod": "did:web:web-did-server%3A8000:valid#controller",
      "created": "2022-09-06T06:32:15.494Z",
      "jws": "eyJhbGciOiJFZERTQSIsImNyaXQiOlsiYjY0Il0sImI2NCI6ZmFsc2V9..OK45cR1bRkNPYrcAU4j5hPzhMNHc04WmC3P54GCm843LH6S9IDWaX6jQ34OJXEVl_uXGovYredqJy1QZWocDBA"
    }
  }
}
```

- Issue/sign presentation with challenge and domain values (attention: replace
  _challenge_ and _domain_ values!):
  `DID_RESOLVER_OVERRIDE=http://localhost:8080/1.0/identifiers/ didkit vc-issue-presentation -C a2a03e6164c8279d62ec026dd483dc3aff50d3da2bc0f651651477a96ac96e26 -d web-did-server -k key.jwk -p authentication -v 'did:web:web-did-server%3A8000:valid#controller' < presentation.json > presentation-signed.json`
- Ensure presentation is valid:
  `didkit vc-verify-presentation -R http://localhost:8080/1.0/identifiers/ < presentation-signed.json`
- Update DID Document:
  `curl -d @presentation-signed.json -X PUT http://localhost:8000/v1/web/valid/did.json`

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
