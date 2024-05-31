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
curl --fail-with-body http://localhost:8000/person/did.json | jq
```
