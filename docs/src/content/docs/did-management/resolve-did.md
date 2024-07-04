---
title: Resolve DID
sidebar:
  order: 1
---

Resolve a DID is an operation available without prior authentication. did-web-server implements the
[did:web method specification](https://w3c-ccg.github.io/did-method-web/) for resolving DIDs.

Example:

- Given DID `did:web:localhost#3A8000:person`
- Execute this command to resolve it:

```bash
# Load the configuration into the local shell
source .env
```

```bash
curl --fail-with-body http://${DWS_EXTERNAL_HOSTNAME}:8000/person/did.json | jq
```
