import { assertEquals, assertStrictEquals } from "std/testing/asserts.ts";
import {
  date2ISOStringWithoutMilliseconds,
  validUntil,
  withClaim,
  withId,
} from "./vc_ld.js";

Deno.test("withId", () => {
  const credential = withId({}, "myid");
  assertStrictEquals(credential.id, "myid", "When no id exists, then set it");
});

Deno.test("withId", () => {
  const credential = withId({ id: "anotherid" }, "myid");
  assertStrictEquals(credential.id, "myid", "When an id exists, then set it");
});

Deno.test("withId", () => {
  const credential = withId({ id: "anotherid", otherattr: "value" }, "myid");
  assertStrictEquals(credential.id, "myid", "When an id exists, then set it");
  assertStrictEquals(
    credential.otherattr,
    "value",
    "When other attributes exist, keep them untouched",
  );
});

Deno.test("withClaim", () => {
  const claim = { claimid: "my id", some: "claim" };
  const credential = withClaim({ id: "anotherid" }, claim);
  assertEquals(
    credential.credentialSubject,
    [claim],
    "When no claim exists, create credentialSubject and add claim",
  );
});

Deno.test("withClaim", () => {
  const existingClaim = { id: "did:example", a: "claim" };
  const newClaim = { claimid: "my id", some: "claim" };
  const credential = withClaim({
    id: "anotherid",
    credentialSubject: existingClaim,
  }, newClaim);
  assertEquals(
    credential.credentialSubject,
    [existingClaim, newClaim],
    "When a claim exists, then move it into a list.",
  );
});

Deno.test("withClaim", () => {
  const existingClaim = { id: "did:example", a: "claim" };
  const newClaim = { claimid: "my id", some: "claim" };
  const credential = withClaim({
    id: "anotherid",
    credentialSubject: [existingClaim],
  }, newClaim);
  assertEquals(
    credential.credentialSubject,
    [existingClaim, newClaim],
    "When a claim exists in a list, then append new claim to list.",
  );
});

Deno.test("validUntil", () => {
  const date = new Date();
  const credential = validUntil({
    id: "anotherid",
  }, date);
  assertEquals(
    credential.validUntil,
    date2ISOStringWithoutMilliseconds(date),
    "When a valid date is provided, then the validity is set properly",
  );
});
