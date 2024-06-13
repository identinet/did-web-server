---
title: Deactivate DID
sidebar:
  order: 4
---

Deactivating a DID is a reserved operation for the server's owner. The steps for registering a DID are described in the
[Getting Started guide](/getting-started).

## Deactivate did:web DID

did-web-server uses DIDs, Verifiable Credentials (VCs) and Verifiable Presentations (VPs) to verify access and encode
data. The following diagram depicts the preparation process for removing a DID document from the server:

1. The DID document is required and can be fetched from the server.
2. A Verifiable Credential is created that includes the DID document. The VC is signed by an authorized key.
3. A Verifiable Presentation is created that includes the VC. The VP is signed by an authorized key. To mitigate replay
   attacks, the VP must also contain specific proof parameters that can be retrieved from did-web-server.
4. If the submitted VP and VC are successfully verified, the included DID document is removed from the server.

![Component diagram for creating and updating a DID document](/figures/did-creation-components.svg)

### Retrieve DID document

Execute the following command to create the DID document that includes both public keys:

```bash title="person-did.json"
curl --fail-with-body -o person-did.json http://localhost:8000/person/did.json
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

### Deactivate DID on server

The last step is to submit the signed presentation to the server:

```bash
curl --fail-with-body -X DELETE -d @person-vp-signed.json http://localhost:8000/person/did.json
```

Let's verify that the DID document doesn't exist anymore:

```bash
curl --fail-with-body http://localhost:8000/person/did.json
```

Congratulations, you've deleted the DID document! 🎉
