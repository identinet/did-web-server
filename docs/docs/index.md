---
title: Introduction
sidebar_position: 1
---

# Introduction

**did-web-server** is an HTTP server for managing DID Documents for the
[did:web](https://w3c-ccg.github.io/did-method-web/) method in a self-sovereign
manner. did:web is a straightforward DID method that use DNS and HTTP servers to
host DID Documents.

Unlike other DID methods, did:web doesn't require a blockchain to store
identifier states. It uses an HTTP server, which is cost effective to operate
and leverages established protocols and software libraries.

However, traditional HTTP servers are centrally managed and do not allow users
to managed hosted documents, making decentralized identifier management
challenging. did-web-server addresses this issue by being the first DID-centered
HTTP server, enabling self-sovereign management for owners of did:web
identifiers.

![Architecture diagram](/figures/did-web-server-context.svg)

# Tutorial Intro

Let's discover **Docusaurus in less than 5 minutes**.

## Getting Started

Get started by **creating a new site**.

Or **try Docusaurus immediately** with
**[docusaurus.new](https://docusaurus.new)**.

### What you'll need

- [Node.js](https://nodejs.org/en/download/) version 18.0 or above:
  - When installing Node.js, you are recommended to check all checkboxes related
    to dependencies.

## Generate a new site

Generate a new Docusaurus site using the **classic template**.

The classic template will automatically be added to your project after you run
the command:

```bash
npm init docusaurus@latest my-website classic
```

You can type this command into Command Prompt, Powershell, Terminal, or any
other integrated terminal of your code editor.

The command also installs all necessary dependencies you need to run Docusaurus.

## Start your site

Run the development server:

```bash
cd my-website
npm run start
```

The `cd` command changes the directory you're working with. In order to work
with your newly created Docusaurus site, you'll need to navigate the terminal
there.

The `npm run start` command builds your website locally and serves it through a
development server, ready for you to view at http://localhost:3000/.

Open `docs/intro.md` (this page) and edit some lines: the site **reloads
automatically** and displays your changes.

Blub
