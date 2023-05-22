import { driver } from "@digitalbazaar/did-method-key";
import { Ed25519VerificationKey2020 } from "@digitalbazaar/ed25519-verification-key-2020";
import { Ed25519Signature2020 } from "@digitalbazaar/ed25519-signature-2020";
import { S } from "../sanctuary/mod.js";
import { id2DID } from "./did.js";
import { deactivate } from "./mod.js";

export const didDocToVC = (ownerDID) => (diddoc) => {
  const diddocLD = diddoc.renderLD();
  return {
    "@context": ["https://www.w3.org/2018/credentials/v1"],
    type: ["VerifiableCredential"],
    id: diddocLD.id,
    credentialSubject: { ...diddocLD },
    issuer: ownerDID,
  };
};

async function useDeactivate() {
  // const keyPair = await Ed25519VerificationKey2020.generate();
  // console.log(keyPair);

  const keyPair = {
    id: undefined,
    controller: undefined,
    revoked: undefined,
    type: "Ed25519VerificationKey2020",
    publicKeyMultibase: "z6MkwSo3P2obKCTN6n3gfKC2XbnrJiKtftrzZZbVKgVwkgoZ",
    privateKeyMultibase:
      "zrv1ZeLYyQSLKav9RWdinNNisSmpMFvLLCC2XyQSzYJ73myqeT1i3VF6SCXYdFrzmQyxieGetSZHYVfvbhLiQyMguVZ",
  };

  const verificationKeyPair = await Ed25519VerificationKey2020.from(keyPair);
  const didKeyDriver = driver();
  didKeyDriver.use({
    multibaseMultikeyHeader: "z6Mk",
    fromMultibase: Ed25519VerificationKey2020.from,
  });
  const { didDocument, keyPairs, methodFor } = await didKeyDriver.fromKeyPair({
    verificationKeyPair,
  });
  const assertionKeyPair = methodFor({ purpose: "assertionMethod" });
  // console.log(assertionKeyPair);

  // here, I'd integrate with the KMS by passing a signer and verifier function
  const suite = new Ed25519Signature2020({ key: assertionKeyPair });
  // console.log(suite);
  const did = id2DID("localhost:3000")("jceb");
  // console.log(did);

  deactivate(assertionKeyPair.controller, suite, did);
}
await useDeactivate();
