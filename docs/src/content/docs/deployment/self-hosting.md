---
title: Self-Hosting
sidebar:
  order: 1
---

First, follow the instructions in the [Getting Started guide](/getting-started). Then continue with this guide.

## Add TLS certificate

With the completion of the [Getting Started guide](/getting-started), the server is functional to create, update and
delete DIDs. However, when operating did-web-server under a DNS name other than `localhost` the did:web specification
**requires resolvers to _only_ accept encrypted traffic**. Therefore, a certificate needs to be added to the server.

If possible, obtain a valid certificate from a known Certificate Authority (CA) like Let's Encrypt and continue with
section [Install Certifcation](#install-certificate). If this is not possible,
[create a local CA](#create-local-certificate-authority) and with a self-issued certificate.

### Create local Certificate Authority

The excellent [mkcert](https://github.com/FiloSottile/mkcert) tool simplifies the creation and operating system
integration of a local Certificate Authority. Follow these steps to set up the Certificate Authority:

1. Install mkcert following the instructions on
   [https://github.com/FiloSottile/mkcert](https://github.com/FiloSottile/mkcert)
2. Setup and install local CA:

```bash
mkcert -install
```

Ensure that the previous command completed successfully before proceeding to the next step.

### Issue self-signed Certificate for Server

To issue the certificate, first determine the DNS name of the server. `example.com` is assumed in the following steps.

Create private key and issue certificate:

```bash
mkcert example.com
```

### Install Certificate

1. Now, let's enable the certificate in the configuration:

```bash title=".env" {7}
# Put the created or an existing DID here.
DWS_OWNER=did:key:xxxx
# Set DWS_ADDRESS to bind to all IPv4 and IPv6 addresses so the service can be exposed to the local computer.
DWS_ADDRESS=::
# Hostname and port determine the DIDs that are managed by this server, e.g. did:web:id.localhost%3A8000:xyz.
DWS_EXTERNAL_HOSTNAME=example.com
# Store DIDs on the local file system.
DWS_BACKEND=file
# DIDs will be stored in the `dids` folder below your current directory.
DWS_BACKEND_FILE_STORE=/run/dws/did_store
DWS_LOG_LEVEL=normal
# For compatibilty with DID resolvers, a certificate is required. It will be added later.
DWS_TLS={certs="example.com.pem",key="example.com-key.pem"}
```

2. With the updated configuration in place, let's restart the server:

```bash
docker run -it --rm -p 8000:443 --env-file .env -u "$(id -u):$(id -g)" -v "$PWD:/run/dws" identinet/did-web-server:0.4.1
```

### Test Functionality

The validity of the test server's certificate can be tested by either visiting
[https://example.com/person/did.json](https://example.com/person/did.json) in the browser or running the following
command:

```bash
curl --fail-with-body https://example.com/person/did.json
```

Congratulations, you have a fully operational did-web-server instance! 🎉
