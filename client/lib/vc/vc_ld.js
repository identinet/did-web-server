import { $, S } from "../sanctuary/mod.js";

/**
 * A Verifiable Credential that can be enhanced with addtional functions from this module.
 * See also @link https://w3c.github.io/vc-data-model/.
 * @typedef {object} Credential */

/**
 * Template for a verifiable credential.
 * @type {Credential}
 */
export const VC_LD_TEMPLATE = {
  "@context": [
    "https://www.w3.org/ns/credentials/v2",
  ],
  // "id": "TO_BE_DEFINED",
  "type": ["VerifiableCredential"],
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
 * setProperty sets a property to a value. If the property doesn't exist, it is created.
 *
 * @param {Credential} credential - A credential object.
 * @param {string} property - A property name.
 * @param {any} value - The value that's appended to the property.
 * @returns {Credential} Updated credential.
 * @throws Throws exception if types aren't correct.
 */
export const setProperty = S.def("setProperty")({})([
  $.Object,
  $.String,
  $.Unknown,
  $.Object,
])(
  (credential) => (property) => (value) => ({
    ...credential,
    [property]: value,
  }),
);

/**
 * appendProperty appends a value to a property. If the property is undefined, an array is created containing the new
 * value. If the property is an array, the new value is appended at the end. In all other cases, an array is created
 * and the existing and the new value are added to it.
 *
 * @param {Credential} credential - A credential object.
 * @param {string} property - A property name.
 * @param {any} value - The value that's appended to the property.
 * @returns {Credential} Updated credential.
 * @throws Throws exception if types aren't correct.
 */
export const appendProperty = S.def("appendProperty")({})([
  $.Object,
  $.String,
  $.Unknown,
  $.Object,
])(
  (credential) => (property) => (value) => {
    const new_credential = { ...credential };
    if (S.type(new_credential[property]).name === "Undefined") {
      new_credential[property] = [value];
    } else if (S.type(new_credential[property]).name === "Array") {
      new_credential[property].push(value);
    } else {
      new_credential[property] = [new_credential[property], value];
    }
    return new_credential;
  },
);

/**
 * withId sets the ID property of a credential.
 *
 * @param {Credential} credential - A credential object.
 * @param {string} id - ID for the credential.
 * @returns {Credential} Updated credential.
 * @throws Throws exception if types aren't correct.
 */
export function withId(credential, id) {
  return setProperty(credential)("id")(id);
}

/**
 * validUntil defines how long a credential will be valid for.
 *
 * @param {Credential} credential - A credential object.
 * @param {Date} date - Date until the credential will be valid.
 * @returns {Credential} Updated credential.
 * @throws Throws exception if types aren't correct.
 */
export function validUntil(credential, date) {
  if (!S.is($.ValidDate)(date)) {
    throw new Error(
      `TypeError: date is not of type ValidDate. Actual: ${S.type(date).name}`,
    );
  }
  return setProperty(credential)("validUntil")(date.toISOString());
}

/**
 * withClaim adds an additional claim to a credential.
 *
 * @param {Credential} credential - A credential object.
 * @param {object} claim - Claim that will be added to the credential.
 * @returns {Credential} Updated credential.
 * @throws Throws exception if types aren't correct.
 */
export function withClaim(credential, claim) {
  return appendProperty(credential)("credentialSubject")(claim);
}

/**
 * withContext adds an additional context to a credential.
 *
 * @param {Credential} credential - A credential object.
 * @param {(string|object)} context - A JSON-LD context.
 * @returns {Credential} Updated credential.
 * @throws Throws exception if types aren't correct.
 */
export function withContext(credential, context) {
  return appendProperty(credential)("@context")(context);
}

/**
 * withType adds an additional type to a credential.
 *
 * @param {Credential} credential - A credential object.
 * @param {string} type - A JSON-LD type. Make sure the corresponding context has been added via @link withContext.
 * @returns {Credential} Updated credential.
 * @throws Throws exception if types aren't correct.
 */
export function withType(credential, type) {
  return appendProperty(credential)("type")(type);
}
