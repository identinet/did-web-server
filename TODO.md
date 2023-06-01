# Doing

- [i] Implement CLI client in JS
- [i] Implement ownership for create, update and delete endpoints
  - [x] I need to observe the owner's DID first in the configuration of the
        server
  - [x] I need to adjust the file format for creating DIDs
  - [x] I need to process the authentication request as I did for the update
        method
  - [x] I need to add tests that verify the behavior
  - [ ] Implement delete authentication

# Do

- [ ] Create release of the binary
- [ ] Create release of the binary in a docker container
- [ ] Create helm chart / kustomize configuration for hosting the chart

- [ ] Can I generate an OpenAPI description instead of a manual description so I
      get proper docs and examples?
- [ ] Add more integration tests
  - add .well-known test
  - test access to ids starting with "."

- [ ] DID Documents also come with a proof .. could this be leveraged?
  - think about document integrity verification as proposed here:
    https://w3c-ccg.github.io/did-method-web/#did-document-integrity-verification
    - to a certain extent, this is already partially supported - the server
      generates proof parameters for interacting with the documents, it could be
      leveraged for hashlinks
- [ ] Support additional storage mechanims other than the file system, e.g. a
      databases like postgres, mssql, sqlite
- [ ] Add support for a history of documents, including a history of operations
      and authentications that were used for audit purposes
- [ ] Document that credentials should have a validUntil timestamp to mitigate
      "replay attacks", i.e. the same request being sent twice

# Done

- [x] Check SCC integration .. is everything correct?
- [x] Add cliff and gh as release tools
- [x] add server ownership via a DID
- [x] add built-in resolver
- [x] implement memory backend
- [x] implement authentication when putting documents via Verifiable
      Presentations
- [x] compatibility with the did:web API .. not sure if there's much imposed:
      https://w3c-ccg.github.io/did-method-web/
- [x] compatibility with the universal registrar API?
- [x] return content type application/did+json for get requests
- [x] support .well-known/did.json documents
- [x] support subpaths for storing DIDs
- [x] ensure that the web-did-resolver test suite passes:
      https://github.com/decentralized-identity/web-did-resolver/blob/master/src/**tests**/resolver.test.ts

# Canceled

- [ ] Think about multi-tenant support - one server for multiple domains .. is
      that even a good idea?
- [ ] Require the presentation to be not older than XX min (configurable?) -
      validUntil should be used by the client, why have an opinion on the
      server?
