// import * as vc from "@digitalbazaar/vc";
// import { Ed25519VerificationKey2020 } from "@digitalbazaar/ed25519-verification-key-2020";
// import { X25519KeyAgreementKey2020 } from "@digitalbazaar/x25519-key-agreement-key-2020";
// import { CryptoLD } from "crypto-ld";
// import jsigs from "jsonld-signatures";
// import {
//   Ed25519Signature2020,
//   suiteContext,
// } from "@digitalbazaar/ed25519-signature-2020";
// import { Resolver } from "did-resolver";
// import { getResolver } from "web-did-resolver";
// import { encaseP, promise } from "fluture";
import { $, S } from "./sanctuary.js";
import { didDocToVC, DIDDocument } from "./did.js";
import { Command } from "cliffy/command/mod.ts";
import { Context, getLogger } from "./ctx.js";
import { loadPrivateKey, loadPublicKey } from "./lib.js";
import { log } from "./utils.js";

/** createDIDDoc :: NonEmpty String -> StrMap (NonEmpty String) -> Object
 *
 * @param {String} subject Name of subject, can't be empty
 * @param {StrMap} map Public key in JWK format with fields x, kty, and crv
 * @returns {DIDDocument}
 */
const createDIDDoc = S.def("createDIDDoc")({})([
  // subject
  $.NonEmpty($.String),
  // { x, kty, crv }
  $.StrMap($.NonEmpty($.String)),
  $.Object,
])((subject) => ({ x, kty, crv }) =>
  new DIDDocument()
    .withSubject(subject)
    .withVerificationMethod({
      id: "1",
      type: "JsonWebKey2020",
      publicKey: {
        crv,
        x,
        kty,
        "kid": "_Qq0UL2Fq651Q0Fjd6TvnYE-faHiOpRlPVQcY_-tA4A",
      },
    })
  // TODO: also register proper rights, assertionMethod, etc
);

/** create :: (Context, StrMap, Array String) -> Either StrMap String
 *
 * @param ctx {Context} Execution context.
 * @param options {Object} JS Object with options.
 * @param args {Array<String>} List of arguments.
 * @returns Either an object with the created DID Document or an error message.
 */
function create(ctx, options, args) {
  // - [x] create DID Document
  // - [t] construct VC
  // - [t] construct VP
  // - [ ] sign VC with key
  // - [ ] pull proof parameters
  // - [ ] sign presentation with key
  // - [ ] use API to send the document to the API
  // - [ ] wait for result
  const logger = getLogger(ctx);
  logger.info("create");
  logger.info(options);

  const privateKey = loadPrivateKey(options?.key);
  const didVC = S.pipe([
    loadPublicKey,
    S.map(createDIDDoc(args[0])),
    S.map(didDocToVC(options?.ownerDID)),
  ])(options?.publicKey);
  return S.map(
    (key) => S.map((vc) => console.log("TODO: continue here", key, vc))(didVC),
  )(privateKey);
}

function update(ctx, options, args) {
  // receive VC
  // sign VC with key
  // pull proof parameters
  // sign presentation with key
  // use API to send the document to the API
  // wait for result
  console.log("update");
}

function template(ctx, options, args) {
  const logger = getLogger(ctx);
  logger.debug("debug");
  logger.info("info");
  logger.warning("warning");
  logger.error("error");
  return diddoc;
}

function del(ctx, options, args) {
  // pull proof parameters
  // sign presentation with key
  // use API to send the document to the API
  // wait for result
  console.log("delete");
}

/* main is the entrypoint of the application
 */
function main(args) {
  const moduleName = "did-web-server-cli";
  let ctx = new Context();
  ctx = ctx.withValue(
    "logger",
    getLogger(ctx, moduleName, "DEBUG"),
  );
  ctx = ctx.withValue("module", moduleName);

  return new Command()
    .name(ctx.value("module"))
    .version("0.1.0")
    .description("Commandline interface for did-web-server")
    // .default("create")
    //
    // ----- create
    //
    .command("create")
    .description("Create a new DID")
    .option(
      "-k, --key <privateKeyJWK:file>",
      "Server owner's private key in JWK format.",
    )
    .option(
      "-o, --owner-did <did:string>",
      "Server owner's DID that will issue the VP and VC that will contain the DID document.",
    )
    .option("--insecure", "Ignore SSL errors / allow non-encrypted connections")
    .option(
      "-i, --public-key <publicKeyJWK:file>",
      "DID owner's public key in JWK format.",
    )
    .arguments("<did:string>")
    .action((options, ...args) => create(ctx, options, args))
    //
    // ----- update
    //
    .command("update")
    .description("Update an existing DID")
    .option(
      "-k, --key <privateKeyJWK:file>",
      "DID owner's private key in JWK format.",
    )
    .option("--insecure", "Ignore SSL errors / allow non-encrypted connections")
    .arguments("<diddoc:file>")
    .action((options, ...args) => update(ctx, options, args))
    //
    // ----- template
    //
    .command("template")
    .description(
      "Retrieve existing DID Document and generate a new DID Document for updating it",
    )
    .option(
      "-r, --rotate <keyId:string> <newPublicKey:string>",
      "Rotate key-id to new public key",
    )
    .option("-a, --add <newPublicKey:string>", "Add new public key")
    .option("-x, --delete <key-id:string>", "Delete key from DID document")
    .arguments("<did:string>")
    .action((options, ...args) => template(ctx, options, args))
    //
    // ----- delete
    //
    .command("delete")
    .description("Delete an existing DID")
    .option(
      "-k, --key <privateKeyJWK:file>",
      "DID owner's private key in JWK format.",
    )
    .option("--insecure", "Ignore SSL errors / allow non-encrypted connections")
    .arguments("<did:string>")
    .action((options, ...args) => del(ctx, options, args))
    .parse(args);
}

main(Deno.args).then((res) => Deno.exit(res));
