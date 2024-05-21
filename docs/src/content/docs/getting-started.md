---
title: Getting Started
---

Get started by **creating a DID for the server's owner**.

Or **use your existing
[Decentralized Identifiers (DID)](https://w3c.github.io/did-core)** - did:key,
did:jwk, and did:web methods are supported.

Then, start your server.

## Create a DID for the server's owner

did-web-server leverages DIDs to manage every aspect of the service. The first
step in setting up did-web-server is to create the DID for the server's owner.
Multiple ways exist to do that. The excellent
[DIDKit](https://www.spruceid.dev/didkit/didkit/installation) CLI can be used to
do that.

1. Generate a key:

```bash
docker run -it --rm -u "$(id -u):(id -g)" -v "$PWD:/app" -w "/app" ghcr.io/spruceid/didkit-cli:latest key generate ed25519 > owner.jwk
```

2. Derive a did:key DID:

```bash
docker run -it --rm -u "$(id -u):(id -g)" -v "$PWD:/app" -w "/app" ghcr.io/spruceid/didkit-cli:latest key-to-did -k owner.jwk
```

## Start did-web-server

Prepare to start a local test server via Docker that serves DIDs on your
computer and stores DIDs on the local file system. The first step is to create a
configuration file called `.env` with the following contents:

```bash title=".env"
DID_WEB_SERVER_OWNER=did:key:xxxx # Put the created or existing DID here.
DID_WEB_SERVER_EXTERNAL_HOSTNAME=id.localhost # Hostname and port determine the DIDs that are managed by this server, e.g. did:web:id.localhost%3A3000:xyz.
DID_WEB_SERVER_EXTERNAL_PORT=3000 # Set DID_WEB_SERVER_PORT and DID_WEB_SERVER_EXTERNAL_PORT to the same value for this test.
DID_WEB_SERVER_PORT=3000 # Set DID_WEB_SERVER_PORT and DID_WEB_SERVER_EXTERNAL_PORT to the same value for this test.
DID_WEB_SERVER_BACKEND=file # Store DIDs on the local file system.
DID_WEB_SERVER_BACKEND_FILE_STORE=/server/dids # DIDs will be stored in the `dids` folder below your current directory.
# DID_WEB_SERVER_TLS=/server/cert.pem # For compatibilty with DID resolvers, a certificate is required. It will be added later.
```

The `id.localhost` hostname (variable `DID_WEB_SERVER_EXTERNAL_HOSTNAME`) works
on every operating system without additional configuration. However, it is only
accessible by the local computer. Other systems will not be able to resolve the
stored DIDs. For testing purposes, this configuration is fully suitable.

With the configuration in place, it is time to start the server. Execute the
following command to start the server in the current directory. Newly created
DIDs will be stored in the `dids` directory on the local file system:

`docker run -it --rm -p 3000 --env-file .env -u "$(id -u):(id -g)" -v "$PWD:/server" -w "/server" registry.41ppl.com/did-web-server:latest`

## Create the first local DID

Congratulations, the server is up and running! It does not contain any DID, yet.
Let's create the first DID: `did:web:id.localhost%3A3000:user1`

Every DID requires a public private key pair. We can reuse the command that we
used to create the owner's key pair:

```bash
docker run -it --rm -u "$(id -u):(id -g)" -v "$PWD:/app" -w "/app" ghcr.io/spruceid/didkit-cli:latest key generate ed25519 > user1.jwk
```

TODO: continue here

```json title="user1.json"
{
  "@context": ["https://www.w3.org/ns/did/v1"],
  "id": "did:web:id.localhost%3A3000:user1",
  "verificationMethod": [
    {
      "@context": {
        "sec": "https://w3id.org/security/v2#",
        "jwk2020": "https://w3c.github.io/vc-jws-2020/contexts/v1#"
      },
      "id": "did:web:id.localhost%3A3000:user1#key1",
      "type": "did:Ed25519VerificationKey2018",
      "sec:controller": "did:web:id.localhost%3A3000:user1",
      "jwk2020:publicKeyJwk": {
        "jwk2020:kty": "OKP",
        "jwk2020:crv": "Ed25519",
        "jwk2020:x": "IkMQNWwqe-y6KEpIjP2DOKbxF9cqrhur6o-l4OJ8AwA"
      }
    }
  ],
  "authentication": ["web:web-did-server%3A8000:valid#key1"],
  "assertionMethod": ["web:web-did-server%3A8000:valid#key1"]
}
```

With the key pair in place, we prepare a DID document:

## Add a TLS certificate
