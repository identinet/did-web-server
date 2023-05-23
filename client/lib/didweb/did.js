import { $, S } from "../sanctuary/mod.js";
import { attemptP, encaseP, reject, resolve } from "fluture";
import { StructuredDID } from "./types/StructuredDID.js";
import { DID } from "./types/DID.js";
import { NonZeroPortNumber } from "./types/PortNumber.js";

/**
 * DID CRUD operations for modifying did:web DIDs.
 *
 * @typedef DID_CRUD_OPERATIONS
 * @type {object}
 */
export const DID_CRUD_OPERATIONS = {
  "read": "GET",
  "create": "POST",
  "update": "PUT",
  "deactivate": "DELETE",
};

/**
 * HTTP_METHODS list of supported HTTP method names.
 */
const HTTP_METHODS = $.EnumType("HTTP_METHODS")(
  "https://github.com/identinet/identinet/types#HTTP_METHODS",
)(["GET", "POST", "PUT", "DELETE"]);

/**
 * id2DID computes a did:web DID for a domain name and an id.
 *
 * @param {string} domain - Valid DNS domain name with optional port separated by a colon, e.g. localhost:3000.
 * @param {string} id - account id. Though the id is optional for did:web, it's required here.
 *
 * @returns {DID} returns encoded did:web DID.
 */
export const id2DID = S.def("id2DID")({})([
  $.NonEmpty($.String),
  $.NonEmpty($.String),
  DID,
])(
  (domain) => (id) =>
    `did:web:${encodeURIComponent(domain)}:${
      id.split("/").map(encodeURIComponent).join(":")
    }`,
);

/**
 * did2StructuredDID transforms a DID URL into a StructuredDID object.
 *
 * @param {DID} did - DID URL string.
 *
 * @returns {Either<Error,StructuredDID>} returns the DID object or an error message.
 */
export const did2StructuredDID = S.def("did2StructuredDID")({})([
  DID,
  $.Either($.Error)(StructuredDID),
])(
  (did) => {
    const structuredDID = {};
    const elements = S.splitOn(":")(did);
    if (elements.length < 3 || elements[0] !== "did" || elements[1] !== "web") {
      return S.Left(new Error("Provided string is not a did:web DID."));
    }
    structuredDID.name = elements[1];
    const [domain, port] = S.pipe([decodeURIComponent, S.splitOn(":")])(
      elements[2],
    );
    structuredDID.domain = domain;
    if (S.type(port).name !== "Undefined") {
      const portNumber = S.maybeToNullable(S.parseInt(10)(port));
      if (!S.is(NonZeroPortNumber)(portNumber)) {
        return S.Left(new Error("Provided port number is not valid."));
      }
      structuredDID.port = portNumber;
    } else {
      structuredDID.port = 443;
    }
    structuredDID.path = S.map(decodeURIComponent)(elements.slice(3));
    return S.Right(structuredDID);
  },
);

/**
 * did2URL turns a structured DID into an URL for interacting with the DID document.
 *
 * @param {DID} did - a valid did:web DID, see https://w3c-ccg.github.io/did-method-web/.
 *
 * @returns {Either<Error,URL>} Returns the URL to resolve the DID document.
 */
const did2URL = S.def("did2URL")({})([
  DID,
  $.Either($.Error)($.URL),
])(
  S.pipe([
    did2StructuredDID,
    S.map((did) => {
      const schema = did.domain === "localhost" ? "http" : "https";
      const path = did.path.length === 0
        ? ".well-known"
        : S.joinWith("/")(did.path);
      return `${schema}://${did.domain}:${did.port}/${path}/did.json`;
    }),
    S.chain((url) => {
      try {
        return S.Right(new URL(url));
      } catch (err) {
        return S.Left(err);
      }
    }),
  ]),
);

/**
 * fetchProofParameters returns proof parameters/challenge for modifying a DID.
 *
 * @param {DID} did - a valid did:web DID, see https://w4c-ccg.github.io/did-method-web/.
 *
 * @returns {Future<Error,string>} Returns proof parameters/challenge or rejects with an error message.
 */
export const fetchProofParameters = S.def("fetchProofParameters")({})([
  DID,
  $.Future($.Error)($.String),
])(
  S.pipe([
    // build URL
    did2URL,
    S.map((url) => new URL("?proofParameters", url)),
    S.either((msg) => reject(new Error(msg)))(resolve),
    // retrieve result
    S.chain(encaseP(fetch)),
    // I might have to use F.chain for an unknown reason
    S.chain((response) => {
      return response.ok
        ? attemptP(() => response.text())
        : reject(response.statusText);
    }),
  ]),
);

/**
 * buildDIDRequest prepares a request that performs a CRUD operation on a did:web DID.
 *
 * @param {string} operation - CRUD operation performed on the DID - see @link DID_CRUD_OPERATIONS.
 * @param {DID} did - did:web DID.
 * @param {object} payload - Payload that's required to perform the opeations. See did-web-server protocol for more details.
 *
 * @returns {Either<string,Request>} Returns either an HTTP request that can be passed to `fetch()` or an error message. The request always uses HTTPS unless the DID's domain is `localhost`.
 */
export const buildDIDRequest = S.def("buildDIDRequest")({})([
  HTTP_METHODS,
  DID,
  $.Unknown,
  $.Either($.String)($.Request),
])(
  (operation) => (did) => (payload) =>
    S.pipe([
      did2URL,
      S.map((url) =>
        new Request(url, { method: operation, body: JSON.stringify(payload) })
      ),
    ])(did),
);
