---
title: Update DID
sidebar:
  order: 3
---

Updating a DID document is a reserved operation for the DID's controller. As described in the
[Getting Started guide](/getting-started), the prerequisite for managing a DID document is access to the DID's
cryptographic privat key. In the following sections, the private key is assumed to be stored in file `person.jwk`.

## Update did:web DID

Let's add a second key to the DID: `did:web:localhost%3A8000:person`

did-web-server uses DIDs, Verifiable Credentials (VCs) and Verifiable Presentations (VPs) to verify access and encode
data. The following diagram depicts the preparation process for an updated DID document to be sent to and stored on the
server:

1. First, another cryptographic key is created.
2. The DID document is updated to include the second key.
3. A Verifiable Credential is created that includes the DID document. The VC is signed by an authorized key.
4. A Verifiable Presentation is created that includes the VC. The VP is signed by an authorized key. To mitigate replay
   attacks, the VP must also contain specific proof parameters that can be retrieved from did-web-server.
5. If the submitted VP and VC are successfully verified, the included DID document is stored on the server.

![Component diagram for creating and updating a DID document](/figures/did-creation-components.svg)

### Create second key

Every DID requires a public private key pair. We can reuse the previous command to create another key pair for the new
DID:

```bash
# Load the configuration into the local shell
source .env
```

```bash title="person.jwk"
docker run --rm identinet/didkit-cli:0.3.2-5 key generate ed25519 > person.jwk2
```

Let's store the DID in a file for quick access:

```bash title="person.did
echo "did:web:${DWS_EXTERNAL_HOSTNAME}%3A8000:person" > person.did
```

### Update DID document

Execute the following command to create the DID document that includes both public keys:

```bash title="person-did.json"
cat > person-did.json <<EOF
{
  "@context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/jws-2020/v1"
  ],
  "id": "did:web:${DWS_EXTERNAL_HOSTNAME}%3A8000:person",
  "verificationMethod": [
    {
      "id": "did:web:${DWS_EXTERNAL_HOSTNAME}%3A8000:person#key1",
      "type": "JsonWebKey2020",
      "controller": "did:web:${DWS_EXTERNAL_HOSTNAME}%3A8000:person",
      "publicKeyJwk": {
        "kty": "OKP",
        "crv": "Ed25519",
        "x": "$(jq -r .x person.jwk)"
      }
    },
    {
      "id": "did:web:${DWS_EXTERNAL_HOSTNAME}%3A8000:person#key2",
      "type": "JsonWebKey2020",
      "controller": "did:web:${DWS_EXTERNAL_HOSTNAME}%3A8000:person",
      "publicKeyJwk": {
        "kty": "OKP",
        "crv": "Ed25519",
        "x": "$(jq -r .x person.jwk2)"
      }
    }
  ],
  "authentication": ["did:web:${DWS_EXTERNAL_HOSTNAME}%3A8000:person#key1", "did:web:${DWS_EXTERNAL_HOSTNAME}%3A8000:person#key2"],
  "assertionMethod": ["did:web:${DWS_EXTERNAL_HOSTNAME}%3A8000:person#key1",  "did:web:${DWS_EXTERNAL_HOSTNAME}%3A8000:person#key2"]
}
EOF
```

### Place DID document in Verifiable Credential

Since did-web-server uses Verifiable Credentials for authentication and authorization, and DID documents as data, the
created DID document needs to be placed within a Verifiable Credential. Execute the following command to create and sign
the credential:

```bash title="person-vc-did.json"
cat > person-vc-did.json <<EOF
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1"
  ],
  "id": "uuid:49387f58-c0d9-4b14-a4f4-bc31a021d925",
  "type": ["VerifiableCredential"],
  "issuer": "$(cat person.did)",
  "issuanceDate": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "credentialSubject": $(cat person-did.json)
}
EOF
```

Sign credential:

```bash title="person-vc-did-signed.json"
VERIFICATION_METHOD="$(docker run --rm --network=host identinet/didkit-cli:0.3.2-5 did resolve "$(cat person.did)" | jq -r '.assertionMethod.[0]')"
docker run -i --rm -u "$(id -u):$(id -g)" -v "$PWD:/run/didkit" --network=host identinet/didkit-cli:0.3.2-5 credential issue \
  -k person.jwk -p assertionMethod -t Ed25519Signature2018 -v "$VERIFICATION_METHOD" < person-vc-did.json > person-vc-did-signed.json
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
curl --fail-with-body -o person-vp-proof-parameters.json http://${DWS_EXTERNAL_HOSTNAME}:8000/person/did.json?proofParameters
```

With the proof parameters in place, the next step is to create the presentation:

```bash title="person-vp.json"
cat > person-vp.json <<EOF
{
  "@context": "https://www.w3.org/2018/credentials/v1",
  "type": ["VerifiablePresentation"],
  "holder": "$(cat person.did)",
  "verifiableCredential": $(cat person-vc-did-signed.json)
}
EOF
```

Finally, sign the presentation with the correct proof parameters:

```bash title="person-vp-did-signed.json"
VERIFICATION_METHOD="$(docker run --rm --network=host identinet/didkit-cli:0.3.2-5 did resolve "$(cat person.did)" | jq -r '.assertionMethod.[0]')"
DOMAIN="$(jq -r .domain person-vp-proof-parameters.json)"
CHALLENGE="$(jq -r .challenge person-vp-proof-parameters.json)"
PROOF_PURPOSE="$(jq -r .proof_purpose person-vp-proof-parameters.json)"
docker run -i --rm -u "$(id -u):$(id -g)" -v "$PWD:/run/didkit" --network=host identinet/didkit-cli:0.3.2-5 presentation issue \
  -k person.jwk -p "$PROOF_PURPOSE" -t Ed25519Signature2018 -v "$VERIFICATION_METHOD" -d "$DOMAIN" -C "$CHALLENGE" \
< person-vp.json > person-vp-signed.json
```

### Update DID on server

The last step is to submit the signed presentation to the server:

```bash
curl --fail-with-body -X PUT -d @person-vp-signed.json http://${DWS_EXTERNAL_HOSTNAME}:8000/person/did.json
```

Let's retrieve the DID document from did-web-server for inspection:

```bash
curl --fail-with-body http://${DWS_EXTERNAL_HOSTNAME}:8000/person/did.json | jq
```

Congratulations, you've updated the DID document! 🎉
