---
title: Introduction
---

**did-web-server** is an HTTP server for providing [Decentralized Identifiers (DIDs)](https://w3c.github.io/did-core)
that conform to the [did:web](https://w3c-ccg.github.io/did-method-web/) method. did:web is a straightforward DID method
that uses DNS and HTTP servers to host DIDs.

Unlike other DID methods, did:web doesn't require a blockchain to store identifier states. It uses an HTTP server, which
is cost effective to operate and leverages established protocols and software libraries.

However, traditional HTTP servers are centrally managed and do not allow users to managed hosted documents, making
decentralized identifier management challenging. did-web-server addresses this issue by being the first DID-centered
HTTP server that enables self-sovereign management of identifiers by their controllers.

did-web-server leverages DIDs, [Verifiable Credentials (VC)](https://w3c.github.io/vc-data-model), and Verifiable
Presentations to authenticate identifiers and encode identifier data. No API tokens, usernames, or passwords are
required to interact with the service.

![Architecture diagram](/figures/did-web-server-context.svg)

## Tutorial

Let's discover [**did-web-server in less than 15 minutes**](/getting-started).
