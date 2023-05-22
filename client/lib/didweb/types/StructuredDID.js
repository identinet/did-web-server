import $ from "sanctuary-def";
import { NonZeroPortNumber } from "./PortNumber.js";

/**
 * Structured DID Web according to https://w3c.github.io/did-core/#did-syntax.
 *
 * @typedef DIDWeb
 * @type {object}
 * @property {string} name - DID method name
 * @property {string} domain - Domain name
 * @property {number} port - Port number that the connection is established to, default 443
 * @property {string[]} path - Method specific ID
 */
export const StructuredDID = $.NamedRecordType("StructuredDID")(
  "https://github.com/identinet/identinet/types#StructuredDID",
)([])({
  name: $.NonEmpty($.String),
  domain: $.NonEmpty($.String),
  port: NonZeroPortNumber,
  path: $.Array($.NonEmpty($.String)),
});
