import { $, S } from "./sanctuary.js";
import { PrivateKey, PublicKey } from "./types/JWK.js";

/** takeKeys :: Object -> Array String -> Maybe Object
 * takeKeys returns an object in which only the specified keys are present.
 *
 * @param {Object} object Any javascript object
 * @param {Array<String>} keys Array of strings that will be retrieved from object
 * @returns {Maybe<Object>} Maybe an object if all keys were present in object
 *
 * Tests:
 * > S.show(takeKeys({})([])));
 * "Just ({})"
 * > S.show(takeKeys({})(["b"])));
 * "Nothing"
 * > S.show(takeKeys({ a: 1 })(["b"])));
 * "Nothing"
 * > S.show(takeKeys({ a: 1 })(["a"])));
 * "Just ({\"a\": 1})"
 * > S.show(takeKeys({ a: 1, b: 2 })(["a"])));
 * "Just ({\"a\": 1})"
 * > S.show(takeKeys({ a: 1, b: 2, c: 3 })(["a", "b"])));
 * "Just ({\"a\": 1, \"b\": 2})"
 */
export const takeKeys = S.def("takeKeys")({})([
  $.StrMap($.NonEmpty($.String)),
  $.Array($.String),
  $.Maybe($.StrMap($.NonEmpty($.String))),
])((object) => (keys) => {
  let result = S.Just({});
  for (const key of keys) {
    if (object?.[key] !== undefined) {
      result = S.map((o) => S.insert(key)(object[key])(o))(result);
    } else {
      result = S.Nothing;
    }
  }
  return result;
});

/** loadTextFileAsJson :: (String|Undefined) -> Maybe StrMap String
 */
const loadTextFileAsJson = (file) => {
  const parseJsonToStrMap = S.parseJson(S.is($.StrMap($.String)));
  return S.pipe([
    (file) =>
      file !== undefined ? S.Just(Deno.readTextFileSync(file)) : S.Nothing,
    S.chain(parseJsonToStrMap),
  ])(file);
};

/** loadPublicKey loads a JWK encoded public key from a file and returns it
 *
 * loadPublicKey :: String -> Either String PublicKey
 *
 * @param {String} publicKeyFile File name that contains public key
 * @returns {Either<String,PublicKey>} Either the public key or an error message
 */
export const loadPublicKey = S.def("loadPublicKey")({})([
  $.String,
  $.Either($.String)(PublicKey),
])(S.pipe([
  loadTextFileAsJson,
  // S.chain(S.ifElse(S.is(PublicKey))(S.Just)(() => S.Nothing)),
  S.chain((key) => takeKeys(key)(["x", "kty", "crv"])),
  S.maybeToEither("ERROR: public key couldn't be loaded"),
]));

/** loadPrivateKey loads a JWK encoded public key from a file and returns it
 *
 * loadPrivateKey :: String -> Either String PrivateKey
 *
 * @param {String} publicKeyFile File name that contains public key
 * @returns {Either<String,PrivateKey>} Either the public key or an error message
 */
export const loadPrivateKey = S.def("loadPrivateKey")({})([
  $.String,
  $.Either($.String)(PrivateKey),
])(S.pipe([
  loadTextFileAsJson,
  // S.chain(S.ifElse(S.is(PrivateKey))(S.Just)(() => S.Nothing)),
  S.chain((key) => takeKeys(key)(["x", "kty", "crv", "d"])),
  S.maybeToEither("ERROR: private key couldn't be loaded"),
]));
