import { defaultDocumentLoader } from "@digitalbazaar/vc";
import jsigs from "jsonld-signatures";
import { context as ed25519_context } from "./contexts/ed25519-signature-2020-v1.js";

export const documentLoader = jsigs.extendContextLoader((url) => {
  // DEBUG:
  // console.log("url", url);
  if (url === "https://w3id.org/security/suites/ed25519-2020/v1") {
    return {
      contextUrl: null,
      documentUrl: url,
      document: ed25519_context,
    };
  }
  return defaultDocumentLoader(url);
});
