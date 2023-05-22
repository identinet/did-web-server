import $ from "sanctuary-def";

/**
 * Define a custom type for DID strings, see https://w3c.github.io/did-core/#did-syntax.
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
