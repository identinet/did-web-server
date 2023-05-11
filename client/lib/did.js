import { $, S } from "./sanctuary.js";
import { DIDWeb } from "./types/DIDWeb.js";
import { NonZeroPortNumber } from "./types/PortNumber.js";

/**
 * stringToDIDWeb transforms a string into a DID.
 * @param {string} did - String that contains a DID.
 * @returns {Either<string,DIDWeb>} returns the DID object or an error message.
 */
export const stringToDIDWeb = S.def("stringToDID")({})([
  $.String,
  $.Either($.String)(DIDWeb),
])(
  (did) => {
    const DID = {};
    const elements = S.splitOn(":")(did);
    if (elements.length < 3 || elements[0] !== "did" || elements[1] !== "web") {
      return S.Left("Provided string is not a did:web DID.");
    }
    DID.name = elements[1];
    const [domain, port] = S.pipe([decodeURIComponent, S.splitOn(":")])(
      elements[2],
    );
    DID.domain = domain;
    if (S.type(port).name !== "Undefined") {
      const portNumber = S.maybeToNullable(S.parseInt(10)(port));
      if (!S.is(NonZeroPortNumber)(portNumber)) {
        return S.Left("Provided port number is not valid.");
      }
      DID.port = portNumber;
    } else {
      DID.port = 443;
    }
    DID.path = S.map(decodeURIComponent)(elements.slice(3));
    return S.Right(DID);
  },
);

/* DIDDocument is a builder class for DID Documents
 */
export class DIDDocument {
  #builder = [];
  constructor() {
    this.withContext("https://www.w3.org/ns/did/v1");
  }
  #setProperty(property, value) {
    return (object) => {
      if (typeof value !== "undefined") {
        object[property] = value;
      }
      return object;
    };
  }
  #appendProperty(property, value) {
    return (object) => {
      if (typeof value !== "undefined") {
        if (!object[property]) {
          object[property] = [];
        }
        object[property].push(value);
      }
      return object;
    };
  }

  withContext(context) {
    this.#builder.push(this.#appendProperty("context", context));
    return this;
  }
  withSubject(subject) {
    this.#builder.push(this.#setProperty("id", subject));
    return this;
  }
  withController(controller) {
    this.#builder.push(this.#setProperty("controller", controller));
    return this;
  }
  withAuthentication(authentication) {
    this.#builder.push(this.#appendProperty("authentication", authentication));
    return this;
  }
  withAssertionMethod(assertionMethod) {
    this.#builder.push(
      this.#appendProperty("assertionMethod", assertionMethod),
    );
    return this;
  }
  withKeyAgreement(keyAgreement) {
    this.#builder.push(
      this.#appendProperty("keyAgreement", keyAgreement),
    );
    return this;
  }
  withCapabilityInvocation(capabilityInvocation) {
    this.#builder.push(
      this.#appendProperty("capabilityInvocation", capabilityInvocation),
    );
    return this;
  }
  withCapabilityDelegation(capabilityDelegation) {
    this.#builder.push(
      this.#appendProperty("capabilityDelegation", capabilityDelegation),
    );
    return this;
  }
  withVerificationMethod(
    { id, type, controller, publicKeyJwk, publicKeyMultibase },
  ) {
    const verificationMethod = [
      this.#setProperty("controller", controller),
      this.#setProperty("publicKeyJwk", publicKeyJwk),
      (object) => {
        if (typeof publicKeyJwk !== "undefined") {
          return object;
        }
        return this.#setProperty("publicKeyMultibase", publicKeyMultibase)(
          object,
        );
      },
    ].reduce((acc, fn) => fn(acc), { id, type });
    this.#builder.push(
      this.#appendProperty("verificationMethod", verificationMethod),
    );
    return this;
  }

  renderLD() {
    return this.#builder.reduce((acc, fn) => fn(acc), {});
  }
}

/** Define a custom type for DID strings, see https://w3c.github.io/did-core/#did-syntax
 *
 * @type {string}
 *
 * Tests:
 * S.is(DID)("did:web:example.com")
 * true
 * S.is(DID)("did:web:example.com:joe")
 * true
 * S.is(DID)("did:web:example.com:john:doe")
 * true
 * S.is(DID)("did:WEB:example.com")
 * false
 */
export const DID = $.NullaryType("DID")(
  "https://github.com/identinet/identinet/types#DID",
)([])((x) =>
  typeof x === "string" && /^did:[a-z0-9]+:[a-zA-Z0-9_.:%-]+$/.test(x)
);

/** Define a custom type for the Service structure, see https://w3c.github.io/did-core/#services
 *
 * @typedef {object} PlainOrSetType
 */
export const PlainOrSetType = (type) =>
  $.NullaryType("PlainOrSetType")(
    "https://github.com/identinet/identinet/types#PlainOrSetType",
  )([])(
    (x) => S.is(type)(x) || S.is($.Array(type))(x),
  );

/** Define a custom type for the Service structure, see https://w3c.github.io/did-core/#services
 *
 * @typedef {object} PlainSetOrMapType
 * for (
 *   const [value, expectedResult] of [
 *     ["string", true],
 *     [["string"], true],
 *     [[], true],
 *     [{ a: "string" }, true],
 *     [{}, true],
 *     [{ a: 1 }, false],
 *     [[1], false],
 *   ]
 * ) {
 *   const result = S.is(PlainSetOrMapType)(value);
 *   if (result != expectedResult) {
 *     console.log(
 *       `Test StringOrSetOfStrings for '${value}'. Expected result: ${expectedResult}. Actual result: ${result}`,
 *     );
 *   }
 * }
 */
export const PlainSetOrMapType = (type) =>
  $.NullaryType(
    "PlainSetOrMapType",
  )(
    "https://github.com/identinet/identinet/types#PlainSetOrMapType",
  )([])(
    (x) =>
      S.is(type)(x) || S.is($.Array(type))(x) ||
      S.is($.StrMap(type))(x),
  );

/** Define a custom type for the Service structure, see https://w3c.github.io/did-core/#services
 *
 * @typedef {object} Service
 * @property {URI} id - The ID of the service.
 * @property {String} type - The type of the verification method.
 * @property {String} serviceEndpoint - The type of the verification method.
 */
export const Service = $.NamedRecordType("Service")(
  "https://github.com/identinet/identinet/types#Service",
)([])({
  id: DID,
  type: PlainOrSetType($.String),
  // TODO: change this to URI type
  serviceEndpoint: PlainSetOrMapType($.String),
});

/**
 * @typedef {object} DIDDocument2
 * @property {String} id - The ID of the verification method.
 * @property {String} type - The type of the verification method.
 * @property {String} controller - The DID that controls the verification method.
 */
export const DIDDocument2 = $.NamedRecordType("DIDDocument")(
  "https://github.com/identinet/identinet/types#DIDDocument",
)([])({
  crv: $.NonEmpty($.String),
  kty: $.NonEmpty($.String),
  x: $.NonEmpty($.String),
});

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
