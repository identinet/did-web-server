import { $, S } from "./sanctuary.js";
import {
  assert,
  assertEquals,
  assertStrictEquals,
} from "std/testing/asserts.ts";
import { stringToDIDWeb } from "./did.js";

Deno.test("stringToDIDWeb", () => {
  const did = stringToDIDWeb("");
  assert(
    S.isLeft(did),
    "When an empty string is passed, then an error is returned",
  );
});

Deno.test("stringToDIDWeb", () => {
  const did1 = stringToDIDWeb("did:webX:example.org");
  const did2 = stringToDIDWeb("did:webX:example.org");
  assert(
    S.isLeft(did1),
    "When DID doesn't start with did:web, then an error is returned",
  );
  assert(
    S.isLeft(did2),
    "When DID doesn't start with did:web, then an error is returned",
  );
});

Deno.test("stringToDIDWeb", () => {
  const did = stringToDIDWeb("did:web:example.org");
  assertStrictEquals(
    S.either(() => false)((did) => did.domain)(did),
    "example.org",
    "When a valid did:web DID is passed, then the domain name gets properly extracted",
  );
  assertStrictEquals(
    S.either(() => false)((did) => did.port)(did),
    443,
    "When no port number is provided, then the default port number is assumed",
  );
});

Deno.test("stringToDIDWeb", () => {
  const did = stringToDIDWeb("did:web:example.org%3Aa10");
  assert(
    S.isLeft(did),
    "When an invalid port number is provided, then an error is returned",
  );
});

Deno.test("stringToDIDWeb", () => {
  const did = stringToDIDWeb("did:web:example.org%3A1024");
  assertStrictEquals(
    S.either(() => false)((did) => did.port)(did),
    1024,
    "When a vaild port number is provided, then it is extracted",
  );
});

Deno.test("stringToDIDWeb", () => {
  const did = stringToDIDWeb("did:web:example.org%3A1024:user:test%3A");
  assertEquals(
    S.either(() => false)((did) => did.path)(did),
    ["user", "test:"],
    "When an id path is provided, the components get extracted and decoded",
  );
});
