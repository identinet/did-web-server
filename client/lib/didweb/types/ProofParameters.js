import $ from "sanctuary-def";
import { DID } from "./DID.js";

/**
 * Proof parameters required for modifying a DID.
 *
 * @typedef {object} ProofParameters
 * @property {DID} did - DID.
 * @property {String} challenge - Challenge for modifying the DID.
 * @property {String} domain - Domain name of the did-web-server.
 * @property {String} proof_purpose - The verifiable credential proof purpose for modifying the DID.
 */
export const ProofParameters = $.NamedRecordType("ProofParameters")(
  "https://github.com/identinet/identinet/types#ProofParameters",
)([])({
  did: $.NonEmpty(DID),
  challenge: $.NonEmpty($.String),
  domain: $.NonEmpty($.String),
  proof_purpose: $.NonEmpty($.String),
});
