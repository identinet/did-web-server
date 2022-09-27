mod config;
mod content_types;
mod did;
mod error;
mod resolver;
mod store;
mod util;

// use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::content_types::DIDContentTypes;
use crate::did::{DIDWeb, ProofParameters};
use crate::error::DIDError;
use crate::util::log;
use rocket::figment::providers::{Env, Serialized};
use rocket::figment::{Figment, Profile};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use ssi::did::Document;
use ssi::vc::{CredentialOrJWT, LinkedDataProofOptions, Presentation};
use std::path::PathBuf;

#[macro_use]
extern crate rocket;

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
    config
        .store
        .get(&id)
        .and_then(|ref doc| ProofParameters::new(config, doc))
        .map_err(log("get, got error:"))
        .map(Json)
}

#[allow(clippy::unused_unit)]
#[get("/<id..>?proofParameters")]
fn get_proof_parameters_root(
    config: &rocket::State<Config>,
    id: PathBuf,
) -> Result<Json<ProofParameters>, DIDError> {
    get_proof_parameters(config, id)
}

#[allow(clippy::unused_unit)]
#[get("/v1/web/.well-known/did.json?proofParameters")]
fn get_proof_parameters_wellknown(
    config: &rocket::State<Config>,
) -> Result<Json<ProofParameters>, DIDError> {
    get_proof_parameters(config, PathBuf::from("/.well-known/did.json"))
}

#[allow(clippy::unused_unit)]
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
    let result = config
        .store
        .get(&id)
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
    config
        .store
        .create(&id, document)
        .and_then(|doc| ProofParameters::new(config, &doc))
        .map_err(log("post, got error:"))
        .map(Json)
}

/// Updates a DID Document if the identity is authorized to perform this operation.
///
/// * `presentation` - verifable presentation that holds the updated DID Document
///
/// # TODO
///
/// * implement .well-known support
#[put("/v1/web/<id..>", data = "<presentation>")]
async fn update(
    config: &rocket::State<Config>,
    id: PathBuf,
    presentation: Json<Presentation>,
) -> Result<Json<ProofParameters>, DIDError> {
    // retrieve proof parameters required to verify the correctness of the presentation
    let doc = config.store.get(&id)?;
    let proof_parameters = ProofParameters::new(config, &doc)?;

    // verify that at least one proof refers to the controller of this DID; other proofs are
    // ignored for this case
    let authentication_methods_in_document: Vec<String> = doc
        .verification_method
        .as_ref()
        .map(|verification_methods| {
            verification_methods
                .iter()
                .map(|verification_method| verification_method.get_id(&proof_parameters.did))
                .collect::<Vec<String>>()
        })
        .unwrap_or_else(Vec::new);

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
                        if authentication_methods_in_document.contains(verification_method) {
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

    println!(
        "proof parameters, challenge: {}",
        proof_parameters.challenge
    );
    println!("proof parameters, domain: {}", proof_parameters.domain);
    let mut _opts = LinkedDataProofOptions {
        challenge: Some(proof_parameters.challenge.to_string()),
        domain: Some(proof_parameters.domain.to_string()),
        proof_purpose: Some(ssi::vc::ProofPurpose::Authentication),
        // created: xx; // this is set to now_ms, not sure if that's correct .. I guess that is should have be created max a minute ago
        ..Default::default()
    };

    let resolver = config.reslover_options.get_resolver();

    // TODO: test if all the containing credentials are also fully verified
    // - [x] ensure that signatures are correct
    // - [-] are the options also applied to every single credential or do they need to be reapplied?
    let result = presentation.verify(Some(_opts), &resolver).await;
    println!("checks {}", result.checks.len());
    println!("warn {}", result.warnings.len());
    println!("errors {}", result.errors.len());

    if !result.errors.is_empty() {
        return Err(DIDError::PresentationInvalid(
            "Presentation invalid, verification failed".to_string(),
        ));
    }

    let mut _opts = LinkedDataProofOptions {
        proof_purpose: Some(ssi::vc::ProofPurpose::AssertionMethod),
        ..Default::default()
    };
    // - [ ] find the credential that was issued by the DID itself .. or the controlling DID?
    // - [ ] ensure that the DID is the subject of the DID Document

    // ensure there's A valid did document in the credentials
    let diddoc_in_vc = presentation.verifiable_credential.as_ref().and_then(|vcs| {
        println!("evaluating");
        vcs.into_iter()
            .map(|credential| match credential {
                CredentialOrJWT::Credential(c) => {
                    println!("credential");
                    c.credential_subject
                        .clone()
                        .into_iter()
                        .fold(None, |acc, o| {
                            println!("default? {:?}", acc);
                            // - [ ] wie stelle ich den Typ eines DidDocs fest? as fehlt irgendwie in der DID Doc definition
                            // - [x] und prüfen, ob die ID die eigene ID ist
                            let id_equals_proof_parameter_did = o.id.as_ref().and_then(|id| {
                                println!("default id? {:?}", id.to_string());
                                if id.to_string() == proof_parameters.did {
                                    Some(true)
                                } else {
                                    None
                                }
                            });
                            if acc.is_none() && id_equals_proof_parameter_did.is_some() {
                                println!(
                                    "credential has been issued for DID {}",
                                    proof_parameters.did
                                );
                                // TODO: ensure that document is a DID Doc
                                Some(o)
                            } else {
                                println!("default");
                                acc
                            }
                        })
                }
                CredentialOrJWT::JWT(_) => {
                    println!("credential jwt");
                    // ignore JWT credentials
                    None
                }
            })
            .fold(None, |acc, o| {
                if acc.is_none() {
                    if o.is_some() {
                        println!("found some");
                        o
                    } else {
                        acc
                    }
                } else {
                    acc
                }
            })
    });

    match diddoc_in_vc
        .as_ref()
        .and_then(|doc| serde_json::to_string(doc).ok())
        // .map(log("json"))
        .and_then(|s| serde_json::from_str::<Document>(&s).ok())
    {
        Some(document) => config
            .store
            .update(&id, document) // TODO: confinue here to fix the update methode .. how to do that?
            .and_then(|doc| ProofParameters::new(config, &doc))
            .map_err(log("post, got error:"))
            .map(Json),
        None => Err(DIDError::PresentationInvalid(
            "Presentation invalid, DID Doc invalid".to_string(),
        )),
    }
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
    config
        .store
        .remove(&id)
        .map_err(log("delete, got error:"))
        // Return the DID
        .map(|_| Json(computed_did.to_string()))
}

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(rocket::Config::default()))
        // .merge(Toml::file("Didwebserver.toml").nested())
        .merge(Env::prefixed("DID_SERVER_").global())
        .select(Profile::from_env_or("DID_SERVER__PROFILE", "default"));
    rocket::custom(figment).manage(Config::default()).mount(
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
