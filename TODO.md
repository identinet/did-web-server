# Tasks

- [i] add integration tests
  - add .well-known test
  - test access to ids starting with "."
- [ ] implement ownership for create, update and delete endpoints
- [ ] implement authentication when posting documents via JWT or DID docs?
- [ ] add unit tests
- [ ] Require the presentation to be not older than XX min
- [ ] DID Documents also come with a proof .. could this be leveraged?
  - think about document integrity verification as proposed here:
    https://w3c-ccg.github.io/did-method-web/#did-document-integrity-verification
- [ ] rethink the API of the service, maybe reserve the admin operations under a
      specific API endpoint while keeping the user-focused operations at the
      root level so that the interaction directly happens as intended .. but
      maybe also the admin operations should happen there
- [ ] support additional storage mechanims other than the file system
- [ ] maybe add support for a history of documents, including a history of
      operations and authentications that were used for audit purposes
- [ ] Think about multi-tenant support - one server for multiple domains .. is
      that even a good idea?
- [ ] create release of the binary
- [ ] create release of the binary in a docker container

# Done

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
