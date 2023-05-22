import $ from "sanctuary-def";
import S from "sanctuary";

/**
 * Define a custom type for the Service structure, see https://w3c.github.io/did-core/#services.
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
 * @typedef {object} DIDDocument
 * @property {String} id - The ID of the verification method.
 * @property {String} type - The type of the verification method.
 * @property {String} controller - The DID that controls the verification method.
 */
export const DIDDocument = $.NamedRecordType("DIDDocument")(
  "https://github.com/identinet/identinet/types#DIDDocument",
)([])({
  crv: $.NonEmpty($.String),
  kty: $.NonEmpty($.String),
  x: $.NonEmpty($.String),
});
