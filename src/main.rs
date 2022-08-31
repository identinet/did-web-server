// TODO: Things to consider:
//
// * [ ] implement authentication when posting documents via JWT?
// * [ ] implement authentication when putting documents via Verifiable Presentations
// * [ ] compatibility with the did:web API .. not sure if there's much imposed: https://w3c-ccg.github.io/did-method-web/
// * [ ] compatibility with the universal registrar API?
// * [x] return content type application/did+json for get requests
// * [x] support .well-known/did.json documents
// * [x] support subpaths for storing DIDs
// * [x] ensure that the web-did-resolver test suite passes: https://github.com/decentralized-identity/web-did-resolver/blob/master/src/__tests__/resolver.test.ts
// * [ ] rethink the API of the service, maybe reserve the admin operations under a specific API endpoint while keeping the user-focused operations at the root level so that the interaction directly happens as intended .. but maybe also the admin operations should happen there
// * [ ] support additional storage mechanims other than the file system
// * [ ] maybe add support for a history of documents, including a history of operations and authentications that were used for audit purposes
// * [ ] think about document integrity verification as proposed here: https://w3c-ccg.github.io/did-method-web/#did-document-integrity-verification

mod config;
mod content_types;
mod did;
mod error;
mod store;
mod util;

// use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::content_types::DIDContentTypes;
use crate::did::{DIDWeb, ProofParameters};
use crate::error::DIDError;
use crate::store::get_filename_from_id;
use crate::util::{get_env, log};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use ssi::did::Document;
use ssi::vc::{LinkedDataProofOptions, Presentation};
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

#[macro_use]
extern crate rocket;

/// Retrieve stored DID Document.
///
/// - `config` Global Rocket configuration
/// - `id` - requested id, e.g. `alice`
/// - returns Result<Document, DIDError>
fn retrieve_document(config: &rocket::State<Config>, id: PathBuf) -> Result<Document, DIDError> {
    get_filename_from_id(&config.didstore, &id)
        // .map(|f| {
        //     f.to_str().map(log("path"));
        //     f
        // })
        .map_err(|e| DIDError::DIDNotFound(e.to_string()))
        // TODO: use let doc = match Document::from_json(include_str!("../tests/did-example-foo.json")) {
        .and_then(|filename| {
            if filename.exists() {
                Ok(filename)
            } else {
                Err(DIDError::DIDNotFound("DID not found".to_string()))
            }
        })
        // .map(|f| {
        //     f.to_str().map(log("path"));
        //     f
        // })
        .and_then(|filename| fs::read(filename).map_err(|e| DIDError::NoFileRead(e.to_string())))
        .and_then(|b| String::from_utf8(b).map_err(|e| DIDError::ContentConversion(e.to_string())))
        .and_then(|ref s| {
            serde_json::from_str::<Document>(s)
                .map_err(|e| DIDError::ContentConversion(e.to_string()))
        })
}

/// Retrieve DID document proof parameters.
///
/// - `config` Global Rocket configuration
/// - `id` - requested id, e.g. `alice`
/// - returns ProofParameters
#[allow(clippy::unused_unit)]
#[get("/v1/web/<id..>?proofParameters")]
fn get_proof_parameters(
    config: &rocket::State<Config>,
    id: PathBuf,
) -> Result<Json<ProofParameters>, DIDError> {
    retrieve_document(config, id)
        .and_then(|ref d: Document| ProofParameters::new(config, d))
        .map_err(log("get, got error:"))
        .map(Json)
}

#[allow(clippy::wrong_self_convention)]
#[get("/<id..>?proofParameters")]
fn get_proof_parameters_root(
    config: &rocket::State<Config>,
    id: PathBuf,
) -> Result<Json<ProofParameters>, DIDError> {
    get_proof_parameters(config, id)
}

#[allow(clippy::wrong_self_convention)]
#[get("/v1/web/.well-known/did.json?proofParameters")]
fn get_proof_parameters_wellknown(
    config: &rocket::State<Config>,
) -> Result<Json<ProofParameters>, DIDError> {
    get_proof_parameters(config, PathBuf::from("/.well-known/did.json"))
}

#[allow(clippy::wrong_self_convention)]
#[get("/.well-known/did.json?proofParameters")]
fn get_proof_parameters_wellknown_root(
    config: &rocket::State<Config>,
) -> Result<Json<ProofParameters>, DIDError> {
    get_proof_parameters(config, PathBuf::from("/.well-known/did.json"))
}

/// Retrieve DID document.
///
/// - `config` Global Rocket configuration
/// - `id` - requested id, e.g. `alice`
/// - returns JSON encoded DID document
#[get("/v1/web/<id..>")]
fn get(
    config: &rocket::State<Config>,
    id: PathBuf,
) -> (ContentType, Result<Json<Document>, DIDError>) {
    // TODO: verify that the DID in the doc is equal to the DID that has been requested - bail out otherwise
    // TODO: maybe return json_api for errors?
    let result = retrieve_document(config, id)
        // .and_then(|ref d: Document| {
        //     serde_json::to_string(d).map_err(|e| MyErrors::ConversionError(e.to_string()))1. [x] identinet: Work on did:web based file hosting service - get the service going with the integration of the SSI library
        // })
        .map_err(log("get, got error:"));
    let content_type = match &result {
        Ok(_diddoc) => {
            // if diddoc.context {
            DIDContentTypes::DID_LD_JSON
            // } else {
            // DIDContentTypes::DID_JSON
            // }
        }
        Err(_) => ContentType::JSON,
    };
    (content_type, result.map(Json))
}

#[get("/<id..>")]
fn get_root(
    config: &rocket::State<Config>,
    id: PathBuf,
) -> (ContentType, Result<Json<Document>, DIDError>) {
    get(config, id)
}

// Required to explicitly allow access to a path starting with "."
#[get("/v1/web/.well-known/did.json")]
fn get_wellknown(
    config: &rocket::State<Config>,
) -> (ContentType, Result<Json<Document>, DIDError>) {
    get(config, PathBuf::from("/.well-known/did.json"))
}

// Required to explicitly allow access to a path starting with "."
#[get("/.well-known/did.json")]
fn get_wellknown_root(
    config: &rocket::State<Config>,
) -> (ContentType, Result<Json<Document>, DIDError>) {
    get(config, PathBuf::from("/.well-known/did.json"))
}

/// Creates a DID document at the given position. The DID Document's id must match the DID the
/// computed DID at this position otherwise the DID wouldn't be manageable.
///
/// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
/// - `doc` - JSON-LD DID Document as specified in https://w3c.github.io/did-core/
/// - returns DID as JSON string
///
/// # TODO
///
/// * Implement authentication via some fitting method, JWT or actual signed requests via a private
///   key
/// * Support subfolder so that the DIDs don't only have to live in the top-level folder
/// * implement .well-known support
#[post("/v1/web/<id..>", data = "<doc>")]
fn create(
    config: &rocket::State<Config>,
    id: PathBuf,
    doc: Json<Document>,
) -> Result<Json<ProofParameters>, DIDError> {
    // guard to ensure that the DID is in general manageable
    let computed_did = match DIDWeb::did_from_config(config, &id) {
        Ok(did) => did,
        Err(e) => return Err(e),
    };
    if doc.id != format!("{}", computed_did) {
        return Err(DIDError::DIDMismatch(format!(
            "DIDs don't match - expected: {} received: {}",
            computed_did, doc.id
        )));
    }
    let document = doc.into_inner();
    get_filename_from_id(&config.didstore, &id)
        .map_err(|e| DIDError::NoFileName(e.to_string()))
        .and_then(|filename| {
            if filename.exists() {
                Err(DIDError::DIDExists(format!(
                    "DID already exists: {}",
                    computed_did
                )))
            } else {
                Ok(filename)
            }
        })
        // Store DID doc in file
        .and_then(|filename| {
            // TODO: externalize into a separate function store_did_doc
            std::fs::File::create(filename)
                .map_err(|e| DIDError::NoFileWrite(e.to_string()))
                .and_then(|mut f| {
                    // rocket::serde::json::to_string(&doc)
                    // ah, ich muss da direkt das document hineinstecken, denn das ist serializable ..
                    // wie kommen an das document?
                    serde_json::to_string(&document)
                        .map_err(|e| DIDError::ContentConversion(e.to_string()))
                        .and_then(|s| {
                            f.write(s.as_bytes())
                                .map_err(|e| DIDError::NoFileWrite(e.to_string()))
                        })
                })
        })
        .map_err(log("post, got error:"))
        .and_then(|_| ProofParameters::new(config, &document))
        .map(Json)
}

/// Updates a DID Document if the identity is authorized to perform this operation.
///
/// * `presentation` - verifable presentation that holds the updated DID Document
///
/// # TODO
///
/// * Prevent replay attacks
/// * implement .well-known support
#[put("/v1/web/<id>/did.json", data = "<presentation>")]
fn update(
    config: &rocket::State<Config>,
    id: PathBuf,
    presentation: Json<Presentation>,
) -> Result<Json<ProofParameters>, DIDError> {
    if presentation.validate().is_err() {
        return Err(DIDError::PresentationInvalid(
            "Presentation invalid, proof missing".to_string(),
        ));
    }

    // retrieve proof parameters required to verify the correctness of the presentation
    let doc = retrieve_document(config, id)?;
    let proof_parameters = ProofParameters::new(config, &doc)?;

    // verify that at least one proof refers to the controller of this DID; other proofs are
    // ignored for this case
    let authentication_methods_in_document: Vec<String> = doc
        .verification_method
        .map(|verification_methods| {
            verification_methods
                .iter()
                .map(|verification_method| verification_method.get_id(&proof_parameters.did))
                .map(|x| x)
                .collect::<Vec<String>>()
        })
        .unwrap_or_else(|| Vec::new());
    // next, walk through the proof sections and ensure that a least one refers to an ID in the
    // authentication section of the DID Document
    // TODO: use ssi::did_resolve::get_verification_methods(did, verification_relationship, resolver) instead?
    presentation
        .proof
        .as_ref()
        .ok_or_else(|| {
            DIDError::PresentationInvalid("Presentation invalid, no proof found".to_string())
        })
        .and_then(|proofs| {
            if proofs.any(|proof| {
                proof
                    .verification_method
                    .as_ref()
                    .and_then(|verification_method| {
                        // if verification_method.starts_with(&proof_parameters.did) {
                        if authentication_methods_in_document.contains(&verification_method) {
                            println!("proof found {}", verification_method);
                            Some(verification_method)
                        } else {
                            println!("proof not found");
                            None
                        }
                    })
                    .is_some()
            }) {
                Ok(true)
            } else {
                Err(DIDError::PresentationInvalid(
                    "Presentation invalid, no proof has been signed by expected did".to_string(),
                ))
            }
        })?;

    let mut _opts = LinkedDataProofOptions::default();
    _opts.challenge = Some(proof_parameters.challenge.to_string());
    _opts.domain = Some(proof_parameters.domain.to_string());
    _opts.proof_purpose = Some(ssi::vc::ProofPurpose::Authentication);
    // _opts.created = xx; // this is set to now_ms, not sure if that's correct .. I guess that is should have be created max a minute ago

    // TODO:
    // * [ ] use the HTTP resolver that's built-in
    // * [ ] verify the presentation
    // * [ ] use/extract the did doc VC to update the stored document
    // * [ ] update proof parameters for the new update challenge
    // * [ ] verify that everything works

    // presentation.verify(Some(_opts), resolver);

    Ok(Json(proof_parameters))
}

/// Deletes a DID Document if the identity is authorized to perform this operation.
///
/// * `presentation` - verifable presentation that holds the updated DID Document
///
/// # TODO
///
/// * Implement authorization - admin + user who can delete their own id .. good?
#[delete("/v1/web/<id..>")]
fn delete(config: &rocket::State<Config>, id: PathBuf) -> Result<Json<String>, DIDError> {
    // test existence - if not, then return 404
    // try deletion - return 503 if something goes wrong, otherwise 200
    let computed_did = match DIDWeb::did_from_config(config, &id) {
        Ok(did) => did,
        Err(e) => return Err(e),
    };
    get_filename_from_id(&config.didstore, &id)
        .map_err(|e| DIDError::NoFileName(e.to_string()))
        .and_then(|filename| {
            if filename.exists() {
                println!("filename exists");
                Ok(filename)
            } else {
                println!("filename doesnt exists");
                Err(DIDError::DIDNotFound(format!(
                    "DID doesn't exist: {}",
                    computed_did
                )))
            }
        })
        // Delete file that stores DID doc
        .and_then(|filename| {
            std::fs::remove_file(filename).map_err(|e| DIDError::NoFileWrite(e.to_string()))
        })
        .map_err(log("delete, got error:"))
        // Return the DID
        .map(|_| Json(computed_did.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Config::new(
            get_env("EXTERNAL_HOSTNAME", "localhost"),
            get_env("EXTERNAL_PORT", "8000"),
            get_env("EXTERNAL_PATH", "/"),
            PathBuf::from(&get_env(
                "DID_STORE",
                // by default store all files in $PWD/did_store/
                &std::env::current_dir()
                    .map(|val| val.join("did_store").to_str().unwrap_or(".").to_string())
                    .unwrap_or_else(|_| ".".to_string()),
            )),
        ))
        .mount(
            "/",
            routes![
                create,
                delete,
                get,
                get_proof_parameters,
                get_proof_parameters_root,
                get_proof_parameters_wellknown,
                get_proof_parameters_wellknown_root,
                get_root,
                get_wellknown,
                get_wellknown_root,
                update,
            ],
        )
}
