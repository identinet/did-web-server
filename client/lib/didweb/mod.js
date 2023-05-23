import * as vc from "@digitalbazaar/vc";
import { S } from "../sanctuary/mod.js";
import {
  buildDIDRequest,
  DID_CRUD_OPERATIONS,
  fetchProofParameters,
} from "./did.js";
import { encaseP, promise } from "fluture";
import {
  documentLoader,
  validUntil,
  VC_LD_TEMPLATE,
  VP_LD_TEMPLATE,
  withClaim,
} from "../vc/mod.js";

// import { Ed25519Signature2020 } from "@digitalbazaar/ed25519-signature-2020";

/**
 * create registers a new DID on a compatible did-web-server.
 *
 * @param {Ed25519Signature2020} suite - Cryptographic signature suite that is used to represent the user in requests made to the server. For more details on the signature suite, see https://www.w3.org/community/reports/credentials/CG-FINAL-di-eddsa-2020-20220724/
 * @param {object} diddoc - DID document following the did:web DID method. For more details on allowed values, see https://w3c.github.io/did-core/
 *
 * @returns {Promise<object>} Resolves to the registered DID document or rejects with an error message.
 */
export function create(suite, diddoc) {
  // TODO: implementation
  // 1. put the DID document in a VC and sign it with the owner's private key
  createVCfromDiddoc(suite, diddoc);
  // 2. fetch proof parameters?? - they could also be computed locally based upon the old/expected DID doc
  fetchProofParameters(did - web - did);
  // 3. put the VC in a verifiable presentation and sign it withthe owner's private key
  createVP(suite, vc, proofparams); // use either vailidUntil in the credential or an ID that will be saved by the server to prevent replay attacks
  // Retrieve a request for a presentation that's then filled out by the user's wallet; there's no standard for that
  // 4. create the DID document on the server
  createDIDonServer(vp);
}

/**
 * update updates an existing DID on a compatible did-web-server.
 *
 * @param {Ed25519Signature2020} suite - Cryptographic signature suite that is used to represent the user in requests made to the server. For more details on the signature suite, see https://www.w3.org/community/reports/credentials/CG-FINAL-di-eddsa-2020-20220724/.
 * @param {object} diddoc - DID document following the did:web DID method. For more details on allowed values, see https://w3c.github.io/did-core/.
 *
 * @returns {Promise<object>} Resolves to the updated DID document or rejects with an error message.
 */
export function update(suite, diddoc) {
  // TODO: implementation
  update;
}

/**
 * deactivate deletes an existing DID on a compatible did-web-server.
 *
 * @param {DID} issuer - DID of the issuer.
 * @param {Ed25519Signature2020} suite - Cryptographic signature suite that is used to represent the user in requests made to the server. For more details on the signature suite, see https://www.w3.org/community/reports/credentials/CG-FINAL-di-eddsa-2020-20220724/.
 * @param {DID} did - DID following the did:web DID method.
 *
 * @returns {Promise<String>} Resolves to the DID that has been deleted or rejects with an error message.
 */
export function deactivate(issuer, suite, did) {
  // TODO: build the VP
  // TODO: perform the signing
  //
  // 1. create a VP with a simple VC that just points to the DID
  let credential = withClaim(VC_LD_TEMPLATE, {
    id: did,
  });
  const validityPeriodInMilliseconds = 1 * 60 * 1000;
  credential = validUntil(
    credential,
    new Date(Date.now() + validityPeriodInMilliseconds),
  );
  credential.issuer = issuer;
  // credential = withId(credential, "https://example.com/credentials/1872");
  // const now = (new Date()).toJSON();
  // credential.issuanceDate = `${now.substr(0, now.length - 5)}Z`;
  // credential = withContext(
  //   credential,
  //   "https://www.w3.org/2018/credentials/examples/v1",
  // );
  // credential = withType(credential, "AlumniCredential");
  // credential.credentialSubject = credential.credentialSubject[0];
  // console.log(credential);
  // console.log(signedCredential);
  return S.pipe([
    // sign verifiable credential
    encaseP(vc.issue),
    // add credential to verifiable presentation
    S.chain((signedCredential) =>
      S.pipe([
        fetchProofParameters,
        S.chain((proofParameters) => {
          let presentation = withCredential(VP_LD_TEMPLATE, signedCredential);
          presentation = withHolder(presentation, issuer);
          return encaseP(vc.issue)({
            credential: presentation,
            suite,
            challenge: proofParameters,
            documentLoader,
          });
        }),
      ])(did)
    ),
    // send request to the server
    S.chain(buildDIDRequest(DID_CRUD_OPERATIONS.deactivate)(did)),
    S.chain(encaseP(fetch)),
    promise,
  ])({
    credential,
    suite,
    documentLoader,
  });
}
