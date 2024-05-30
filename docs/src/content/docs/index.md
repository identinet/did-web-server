---
title: Introduction
---

**did-web-server** is an HTTP server for providing [Decentralized Identifiers (DIDs)](https://w3c.github.io/did-core)
that conform to the [did:web](https://w3c-ccg.github.io/did-method-web/) method. did:web is a simple DID method that
uses DNS and HTTP servers to host DIDs.

Unlike other DID methods, did:web doesn't require a blockchain to store identifier states. Instead, it uses an HTTP
server, which is cost-effective to operate and leverages established protocols and software libraries.

However, traditional HTTP servers are centrally managed and don't allow users to manage hosted documents, making
decentralized identifier management challenging. did-web-server solves this problem by being the first DID-centered HTTP
server that enables self-sovereign management of identifiers by their controllers.

did-web-server uses DIDs, [Verifiable Credentials (VCs)](https://w3c.github.io/vc-data-model), and Verifiable
Presentations (VPs) to verify access and encode identifier data. No API tokens, usernames, or passwords are required to
interact with the service.

The following diagram depicts the context in which did-web-server operates. A wallet manages the keys that control an
identifier. Only via these keys can DID documents on did-web-server be modified and updated. The main components that
did-web-server interacts with are DID resolvers. They retrieve DID documents from the server via the standardized
[did:web](https://w3c-ccg.github.io/did-method-web/) DID method.

![Architecture diagram](/figures/did-web-server-context.svg)

## Tutorial

Let's discover [**did-web-server in less than 15 minutes**](/getting-started).
