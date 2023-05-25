import { $, S } from "../sanctuary/mod.js";
import { appendProperty, setProperty } from "./vc_ld.js";

/**
 * Template for a verifiable presentation.
 * @type {Presentation}
 */
export const VP_LD_TEMPLATE = {
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
  ],
  // "id": "TO_BE_DEFINED",
  "type": ["VerifiablePresentation"],
  // "issuer": "https://example.com/issuer/123",
  // "validFrom": "2010-01-01T00:00:00Z",
  // "credentialSubject": [{
  //   "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
  // }, {
  //   "id": "did:example:c276e12ec21ebfeb1f712ebc6f1",
  //   "name": "Morgan Doe",
  //   "spouse": "did:example:ebfeb1f712ebc6f1c276e12ec21",
  // }],
};

/**
 * withCredential adds a credential to a verifiable presentation.
 *
 * @param {Presentation} presentation - A verifiable presentation object.
 * @param {Credential} credential - A credential that will be added to the veriable presentation.
 *
 * @returns {Presentation} Updated credential.
 */
export function withCredential(presentation, credential) {
  return appendProperty(presentation)("verifiableCredential")(credential);
}

/**
 * withHolder sets the holder for a verifiable presentation.
 *
 * @param {Presentation} presentation - A verifiable presentation object.
 * @param {DID} holder - DID that will be set as holder.
 *
 * @returns {Presentation} Updated credential.
 */
export function withHolder(presentation, holder) {
  return setProperty(presentation)("holder")(holder);
}
