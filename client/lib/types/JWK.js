import $ from "sanctuary-def";

/** PublicKey JWK encoded key: https://www.rfc-editor.org/rfc/rfc7517 */
export const PublicKey = $.NamedRecordType("PublicKey")(
  "https://github.com/identinet/identinet/types#PublicKey",
)([])({
  crv: $.NonEmpty($.String),
  kty: $.NonEmpty($.String),
  x: $.NonEmpty($.String),
});

/** PrivateKey JWK encoded key: https://www.rfc-editor.org/rfc/rfc7517 */
export const PrivateKey = $.NamedRecordType("PrivateKey")(
  "https://github.com/identinet/identinet/types#PublicKey",
)([])({
  crv: $.NonEmpty($.String),
  kty: $.NonEmpty($.String),
  x: $.NonEmpty($.String),
  d: $.NonEmpty($.String),
});
