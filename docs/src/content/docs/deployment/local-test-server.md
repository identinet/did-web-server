---
title: Local Test Server
sidebar:
  order: 1
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

1. Install mkcert following the instructions on [https://github.com/FiloSottile/mkcert]()
2. Setup and install local CA: `mkcert -install`

Ensure that the previous command completed successfully before proceeding to the next step.

### Issue Certificate for Test Server

To issue the certificate for the test server, first switch to the directory in which you created the server's
configuration (see [Getting Started Guide](/getting-started)). Then follow these step to issue and use the certificate:

1. Issue certificate:

```bash
mkcert localhost
```

2. Two new files have been generated, `localhost.pem` and `localhost-key.pem`. They need to be combined into one file:

```bash
cat localhost.pem localhost-key.pem > cert.pem
```

3. Now, let's enable the certificate in the configuration:

```bash title=".env" {7}
DID_WEB_SERVER_OWNER=did:key:xxxx # Put the created or existing DID here.
DID_WEB_SERVER_EXTERNAL_HOSTNAME=localhost # Hostname and port determine the DIDs that are managed by this server, e.g. did:web:id.localhost%3A3000:xyz.
DID_WEB_SERVER_EXTERNAL_PORT=3000 # Set DID_WEB_SERVER_PORT and DID_WEB_SERVER_EXTERNAL_PORT to the same value for this test.
DID_WEB_SERVER_PORT=3000 # Set DID_WEB_SERVER_PORT and DID_WEB_SERVER_EXTERNAL_PORT to the same value for this test.
DID_WEB_SERVER_BACKEND=file # Store DIDs on the local file system.
DID_WEB_SERVER_BACKEND_FILE_STORE=/server/did_store # DIDs will be stored in the `dids` folder below your current directory.
DID_WEB_SERVER_TLS=/server/cert.pem # For compatibilty with DID resolvers, a certificate is required. It will be added later.
```

4. With the updated configuration in place, let's restart the server:

```bash
docker run -it --rm -p 3000 --env-file .env -u "$(id -u):$(id -g)" -v "$PWD:/server" -w "/server" registry.41ppl.com/did-web-server:latest
```

### Test functionality

The validity of the test server's certificate can be tested by either visiting
[https://localhost:3000/person1/did.json]() in the browser or running the following command:

```bash
curl -f https://localhost:3000/person1/did.json
echo $? # If everything is set up correctly, the output will be: 0
```

Congratulations, you have a fully operational did-web-server instance on your local computer!
