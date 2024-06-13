---
title: Getting Started
---

Get started by **spinning up a did-web-server**.

Either **create a new DID for the server's owner** or **use an existing DID**. The following DID methods are supported:
`did:key`, `did:jwk`, and `did:web`. Then, start your server.

## Create Server Owner's DID

The first step in setting up did-web-server is to create the DID for the server's owner. Multiple ways exist to do that.
The excellent [DIDKit](https://www.spruceid.dev/didkit/didkit/installation) CLI can be used to do that.

1. Generate a key:

```bash title="owner.jwk"
docker run --rm identinet/didkit-cli:0.3.2-5 key generate ed25519 > owner.jwk
```

2. Derive a did:key DID:

```bash title="owner.did"
docker run --rm -u "$(id -u):$(id -g)" -v "$PWD:/run/didkit" identinet/didkit-cli:0.3.2-5 key to did \
  -k owner.jwk | tee owner.did
# Output should look like this:
# did:key:z6MktLbz19wirLPGiWm2PoJg7rYGB5B1a59DxQxNp4F6o96K
```

## Start your Server

Prepare to start a local test server via Docker that serves DIDs on your computer and stores them on the local file
system. The first step is to configure did-web-server via environment variables. Create a configuration file called
`.env` with the following contents:

```bash title=".env"
# Put the created or an existing DID here.
DWS_OWNER=did:key:xxxx
# Set DWS_ADDRESS to bind to all IPv4 and IPv6 addresses so the service can be exposed to the local computer.
DWS_ADDRESS=::
# Hostname and port determine the DIDs that are managed by this server, e.g. did:web:id.localhost%3A8000:xyz.
DWS_EXTERNAL_HOSTNAME=localhost
# Store DIDs on the local file system.
DWS_BACKEND=file
# DIDs will be stored in the `dids` folder below your current directory.
DWS_BACKEND_FILE_STORE=/run/dws/did_store
DWS_LOG_LEVEL=normal
# For compatibilty with DID resolvers, a certificate is required. It will be added later.
# DWS_TLS={certs="localhost.pem",key="localhost-key.pem"}
```

The `localhost` hostname (variable `DWS_EXTERNAL_HOSTNAME`) works on every operating system without additional
configuration. However, it is only accessible by the local computer. Other systems will not be able to resolve DIDs on
this server. For testing purposes on your computer, this configuration is fully sufficient.

With the configuration in place, it is time to start the server. Execute the following command to start the server in
the current directory. Newly created DIDs will be stored in the `./did_store` directory:

```bash
docker run -it --rm -p 8000:8000 --env-file .env -u "$(id -u):$(id -g)" -v "$PWD:/run/dws" identinet/did-web-server:0.3.0
```

## Create the first did:web DID

Congratulations, the server is up and running! It does not contain any DID, yet. Let's create the first DID:
`did:web:localhost%3A8000:person`

did-web-server uses DIDs, Verifiable Credentials (VCs) and Verifiable Presentations (VPs) to verify access and encode
data. The following diagram depicts the preparation process for a DID document to be sent to and stored on the server:

1. First, a cryptographic key is created or an existing key is selected.
2. The DID document is created that references the key.
3. A Verifiable Credential is created that includes the DID document. The VC is signed by an authorized key (when the
   DID is first created, the server owner's key must sign the VC).
4. A Verifiable Presentation is created that includes the VC. The VP is signed by an authorized key (when the DID is
   first created, the server owner's key must sign the VP). To mitigate replay attacks, the VP must also contain
   specific proof parameters that can be retrieved from did-web-server.
5. If the submitted VP and VC are successfully verified, the included DID document is stored on the server.

![Component diagram for creating and updating a DID document](/figures/did-creation-components.svg)

### Create key

Every DID requires a public private key pair. We can reuse the previous command to create another key pair for the new
DID:

```bash title="person.jwk"
docker run --rm identinet/didkit-cli:0.3.2-5 key generate ed25519 > person.jwk
```

### Create DID document

Execute the following command to create the DID document that includes the generated public key:

```bash title="person-did.json"
cat > person-did.json <<EOF
{
  "@context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/jws-2020/v1"
  ],
  "id": "did:web:localhost%3A8000:person",
  "verificationMethod": [
    {
      "id": "did:web:localhost%3A8000:person#key1",
      "type": "JsonWebKey2020",
      "controller": "did:web:localhost%3A8000:person",
      "publicKeyJwk": {
        "kty": "OKP",
        "crv": "Ed25519",
        "x": "$(jq -r .x person.jwk)"
      }
    }
  ],
  "authentication": ["did:web:localhost%3A8000:person#key1"],
  "assertionMethod": ["did:web:localhost%3A8000:person#key1"]
}
EOF
```

### Place DID document in Verifiable Credential

Since did-web-server uses Verifiable Credentials for authentication and authorization, and DID documents as data, the
created DID document needs to be placed within a Verifiable Credential. Execute the following command to create and sign
the credential:

:::note

The credential must be issued by the server's owner, because only the owner has the right to create DIDs on this server.
In contrast, updates are performed by the controller of the DID and not by the server's owner.

:::

```bash title="person-vc-did.json"
cat > person-vc-did.json <<EOF
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1"
  ],
  "id": "uuid:49387f58-c0d9-4b14-a4f4-bc31a021d925",
  "type": ["VerifiableCredential"],
  "issuer": "$(cat owner.did)",
  "issuanceDate": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "credentialSubject": $(cat person-did.json)
}
EOF
```

Sign credential:

```bash title="person-vc-did-signed.json"
VERIFICATION_METHOD="$(docker run --rm --network=host identinet/didkit-cli:0.3.2-5 did resolve "$(cat owner.did)" | jq -r '.assertionMethod.[0]')"
docker run -i --rm -u "$(id -u):$(id -g)" -v "$PWD:/run/didkit" --network=host identinet/didkit-cli:0.3.2-5 credential issue \
  -k owner.jwk -p assertionMethod -t Ed25519Signature2018 -v "$VERIFICATION_METHOD" < person-vc-did.json > person-vc-did-signed.json
```

### Place Verifiable Credential in Verifiable Presentation

The last step in preparing the data for submission is to place the signed Verifiable Credential within a Verifiable
Presentation and secure the registration against replay attacks. did-web-server prevents reply attacks, i.e. the
observation and resubmission of a valid presentation with the goal of overwriting the current configuration of the DID,
by expecting the hash of the current DID document to be present as a
[challenge](https://www.w3.org/TR/vc-data-integrity/#proofs) in the proof section of the Verifiable Presentation,
alongside other parameters.

The first step of placing the Verifiable Credential inside a Verifiable Presentation is to retrieve the proof parameters
for the DID:

```bash title="person-vp-proof-parameters.json"
curl --fail-with-body -o person-vp-proof-parameters.json http://localhost:8000/person/did.json?proofParameters
```

:::note

The initial proof parameters will be the same on all systems that use the same DNS name, i.e. `localhost`! This is by
design. The did:web method relies on a secure DNS configuration to work properly!

:::

With the proof parameters in place, the next step is to create the presentation:

```bash title="person-vp.json"
cat > person-vp.json <<EOF
{
  "@context": "https://www.w3.org/2018/credentials/v1",
  "type": ["VerifiablePresentation"],
  "holder": "$(cat owner.did)",
  "verifiableCredential": $(cat person-vc-did-signed.json)
}
EOF
```

Finally, sign the presentation with the correct proof parameters:

```bash title="person-vp-did-signed.json"
VERIFICATION_METHOD="$(docker run --rm --network=host identinet/didkit-cli:0.3.2-5 did resolve "$(cat owner.did)" | jq -r '.assertionMethod.[0]')"
DOMAIN="$(jq -r .domain person-vp-proof-parameters.json)"
CHALLENGE="$(jq -r .challenge person-vp-proof-parameters.json)"
PROOF_PURPOSE="$(jq -r .proof_purpose person-vp-proof-parameters.json)"
docker run -i --rm -u "$(id -u):$(id -g)" -v "$PWD:/run/didkit" --network=host identinet/didkit-cli:0.3.2-5 presentation issue \
  -k owner.jwk -p "$PROOF_PURPOSE" -t Ed25519Signature2018 -v "$VERIFICATION_METHOD" -d "$DOMAIN" -C "$CHALLENGE" \
< person-vp.json > person-vp-signed.json
```

### Register DID on server

The last step is to submit the signed presentation to the server:

```bash
curl --fail-with-body -X POST -d @person-vp-signed.json http://localhost:8000/person/did.json
```

Let's retrieve the DID document from did-web-server for inspection:

```bash
curl --fail-with-body http://localhost:8000/person/did.json | jq
```

Congratulations, you've registered the first DID! 🎉 From here, you can either move on and explore
[DID Management](/did-management/resolve-did/) or follow the [Self-Hosting guide](/deployment/self-hosting) to deploy
did-web-server in production.
