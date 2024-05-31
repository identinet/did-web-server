---
title: Local Test Server
sidebar:
  order: 1
  hidden: true
---

First, follow the instructions in the [Getting Started Guide](/getting-started). Then continue with this guide.

## Add TLS certificate

With the completion of the [Getting Started Guide](/getting-started), the server is functional to create, update and
delete DIDs. However, one important element is still missing: **did:web requires resolvers to _only_ accept encrypted
traffic from servers**. Therefore, a certificate needs to be added to the test server.

Since the test server is operated on your local computer, we can't easily get a TLS certificate that is accepted as
valid by your browser and other applications, like a local DID resolver. To mitigate this challenge, we will create a
local Certificate Authority (CA) that is integrated into your operating system's list of accepted CAs. This CA will then
issue a valid certificate that can be used by the test server.

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

### Issue Certificate for Test Server

To issue the certificate for the test server, first switch to the directory in which you created the server's
configuration (see [Getting Started Guide](/getting-started)). Then follow these step to issue and use the certificate:

1. Create private key and issue certificate:

```bash
mkcert localhost
```

2. Now, let's enable the certificate in the configuration:

```bash title=".env" {7}
# Put the created or an existing DID here.
DWS_OWNER=did:key:xxxx
# Set DWS_ADDRESS to bind to all IPv4 and IPv6 addresses so the service can be exposed to the local computer.
DWS_ADDRESS=::
# Hostname and port determine the DIDs that are managed by this server, e.g. did:web:id.localhost%3A8000:xyz.
DWS_EXTERNAL_HOSTNAME=localhost
# Store DIDs on the local file system.
DWS_BACKEND=file
# DIDs will be stored in the `dids` folder below your current directory.
DWS_BACKEND_FILE_STORE=/run/dws/did_store
DWS_LOG_LEVEL=normal
# For compatibilty with DID resolvers, a certificate is required. It will be added later.
DWS_TLS={certs="localhost.pem",key="localhost-key.pem"}
```

4. With the updated configuration in place, let's restart the server:

```bash
docker run -it --rm -p 8000 --env-file .env -u "$(id -u):$(id -g)" -v "$PWD:/server" -w "/server" registry.41ppl.com/did-web-server:latest
```

### Test functionality

The validity of the test server's certificate can be tested by either visiting
[https://localhost:8000/person/did.json](https://localhost:8000/person/did.json) in the browser or running the following
command:

```bash
curl -f https://localhost:8000/person/did.json
echo $? # If everything is set up correctly, the output will be: 0
```

Congratulations, you have a fully operational did-web-server instance on your local computer!
