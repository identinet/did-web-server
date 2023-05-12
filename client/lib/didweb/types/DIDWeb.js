import $ from "sanctuary-def";
import { NonZeroPortNumber } from "./PortNumber.js";

/**
 * DID according to https://w3c.github.io/did-core/#did-syntax for the did:web method definition (https://w3c.github.io/did-spec-registries/#did-methods).
 * @typedef DIDWeb
 * @type {object}
 * @property {string} name - DID method name
 * @property {string} domain - Domain name
 * @property {number} port - Port number that the connection is established to, default 443
 * @property {string[]} path - Method specific ID
 */
export const DIDWeb = $.NamedRecordType("DIDWeb")(
  "https://github.com/identinet/identinet/types#DIDWeb",
)([])({
  name: $.NonEmpty($.String),
  domain: $.NonEmpty($.String),
  port: NonZeroPortNumber,
  path: $.Array($.NonEmpty($.String)),
});
