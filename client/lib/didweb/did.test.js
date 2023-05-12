import { $, log, S } from "../sanctuary/mod.js";
import {
  assert,
  assertEquals,
  assertStrictEquals,
} from "std/testing/asserts.ts";
import { buildRequest, DID_CRUD_OPERATIONS, stringToDIDWeb } from "./did.js";

Deno.test("stringToDIDWeb", () => {
  const did = stringToDIDWeb("");
  assert(
    S.isLeft(did),
    "When an empty string is passed, then an error is returned.",
  );
});

Deno.test("stringToDIDWeb", () => {
  const did1 = stringToDIDWeb("did:webX:example.org");
  const did2 = stringToDIDWeb("did:webX:example.org");
  assert(
    S.isLeft(did1),
    "When DID doesn't start with did:web, then an error is returned.",
  );
  assert(
    S.isLeft(did2),
    "When DID doesn't start with did:web, then an error is returned.",
  );
});

Deno.test("stringToDIDWeb", () => {
  const did = stringToDIDWeb("did:web:example.org");
  assertStrictEquals(
    S.either(() => false)((did) => did.domain)(did),
    "example.org",
    "When a valid did:web DID is passed, then the domain name gets properly extracted.",
  );
  assertStrictEquals(
    S.either(() => false)((did) => did.port)(did),
    443,
    "When no port number is provided, then the default port number is assumed.",
  );
});

Deno.test("stringToDIDWeb", () => {
  const did = stringToDIDWeb("did:web:example.org%3Aa10");
  assert(
    S.isLeft(did),
    "When an invalid port number is provided, then an error is returned.",
  );
});

Deno.test("stringToDIDWeb", () => {
  const did = stringToDIDWeb("did:web:example.org%3A1024");
  assertStrictEquals(
    S.either(() => false)((did) => did.port)(did),
    1024,
    "When a vaild port number is provided, then it is extracted.",
  );
});

Deno.test("stringToDIDWeb", () => {
  const did = stringToDIDWeb("did:web:example.org%3A1024:user:test%3A");
  assertEquals(
    S.either(() => false)((did) => did.path)(did),
    ["user", "test:"],
    "When an id path is provided, the components get extracted and decoded.",
  );
});

Deno.test("buildRequest", () => {
  const did = "did:web:example.org%3A1024:user:test";
  const operation = DID_CRUD_OPERATIONS.deactivate;
  const payload = {};
  const req = S.pipe([
    stringToDIDWeb,
    S.map(buildRequest(operation)(payload)),
  ])(did);
  assertEquals(
    S.either(() => false)((req) => req.url)(req),
    "https://example.org:1024/user/test/did.json",
    "When a DID, a operation, and a payload is provided, then a request is created.",
  );
  assertEquals(
    S.either(() => false)((req) => req.method)(req),
    "DELETE",
    "When a DID, a operation, and a payload is provided, then the request uses the sepcified HTTP method.",
  );
});

Deno.test("buildRequest", () => {
  const did = "did:web:localhost%3A1024:user:test";
  const operation = DID_CRUD_OPERATIONS.deactivate;
  const payload = {};
  const req = S.pipe([
    stringToDIDWeb,
    S.map(buildRequest(operation)(payload)),
  ])(did);
  assertEquals(
    S.either(() => false)((req) => req.url)(req),
    "http://localhost:1024/user/test/did.json",
    "When a DID's domain is localhost, then the http scheme is used instead of https.",
  );
});

Deno.test("buildRequest", () => {
  const did = "did:web:example.org";
  const operation = DID_CRUD_OPERATIONS.deactivate;
  const payload = {};
  const req = S.pipe([
    stringToDIDWeb,
    S.map(buildRequest(operation)(payload)),
  ])(did);
  assertEquals(
    S.either(() => false)((req) => req.url)(req),
    "https://example.org/.well-known/did.json",
    "When a DID doesn't contain a port number and doesn't specify a user id, 443 and .well-known/did.json is used.",
  );
});
