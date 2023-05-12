import * as vc from "@digitalbazaar/vc";
import { buildRequest, stringToDID } from "./did.js";
import { validUntil, VC_LD_TEMPLATE, withClaim } from "../vc/vc_ld.js";

/**
 * create registers a new DID on a compatible did-web-server.
 *
 * @param {Ed25519Signature2020} suite - Cryptographic signature suite that is used to represent the user in requests made to the server. For more details on the signature suite, see https://www.w3.org/community/reports/credentials/CG-FINAL-di-eddsa-2020-20220724/
 * @param {object} diddoc - DID document following the did:web DID method. For more details on allowed values, see https://w3c.github.io/did-core/
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
 * @returns {Promise<object>} Resolves to the updated DID document or rejects with an error message.
 */
export function update(suite, diddoc) {
  // TODO: implementation
  update;
}

/**
 * deactivate deletes an existing DID on a compatible did-web-server.
 *
 * @param {Ed25519Signature2020} suite - Cryptographic signature suite that is used to represent the user in requests made to the server. For more details on the signature suite, see https://www.w3.org/community/reports/credentials/CG-FINAL-di-eddsa-2020-20220724/.
 * @param {String} did - DID following the did:web DID method.
 * @returns {Promise<String>} Resolves to the DID that has been deleted or rejects with an error message.
 */
export function deactivate(suite, did) {
  // TODO: implementation
  // 1. create a VP with a simple VC that just points to the DID
  const DID = stringToDID(did);
  let credential = withClaim(VC_LD_TEMPLATE, { id: did });
  const validityPeriodInMilliseconds = 1 * 60 * 1000;
  credential = validUntil(
    credential,
    new Date(Date.now() + validityPeriodInMilliseconds),
  );
  const signedCredential = vc.issue({
    credential,
    purpose,
    suite, // provides issuer?
  });

  // TODO: implement fetchProofParameters
  const proofParameters = fetchProofParameters(); // with the proof parameters added
  let presentation = withCredential(VP_LD_TEMPLATE, signedCredential);
  presentation = withHolder(presentation, signedCredential);
  const payload = vp.issue(presentation, challenge = proofParameters);
  // 1. send request
  buildRequest(DID_CRUD_OPERATIONS.delete)(payload)(DID);
}
