// Fail build if feature is requsted, see https://www.reddit.com/r/rust/comments/8oz7md/make_cargo_fail_on_warning/
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

mod config;
mod content_types;
mod did;
mod error;
mod resolver;
mod store;
mod utils;

use crate::config::Config;
use crate::content_types::DIDContentTypes;
use crate::did::{DIDWeb, ProofParameters};
use crate::error::{CustomStatus, DIDError};
use crate::utils::log;
use chrono::{DateTime, Utc};
use rocket::figment::providers::{Env, Serialized};
use rocket::figment::{Figment, Profile};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use ssi::did::Document;
use ssi::vc::{CredentialOrJWT, LinkedDataProofOptions, Presentation};
use std::path::PathBuf;

#[cfg(test)]
mod test;
#[cfg(test)]
mod test_resolver;
#[cfg(test)]
use crate::test_resolver::DIDWebTestResolver;
#[cfg(test)]
use ssi::did_resolve::SeriesResolver;

#[macro_use]
extern crate rocket;

/// Retrieve DID document proof parameters.
///
/// - `config` Global Rocket configuration
/// - `id` - requested id, e.g. `alice`
/// - returns ProofParameters
#[allow(clippy::unused_unit)]
#[get("/<id..>?proofParameters")]
fn get_proof_parameters(
    config: &rocket::State<Config>,
    id: PathBuf,
) -> Result<Json<ProofParameters>, DIDError> {
    config
        .store
        .get(&id)
        .and_then(|ref doc| ProofParameters::new(config, doc))
        .map_err(log("get, got error:"))
        .map(log("got proof parameters:"))
        .map(Json)
}

#[allow(clippy::unused_unit)]
#[get("/.well-known/did.json?proofParameters")]
fn get_proof_parameters_wellknown(
    config: &rocket::State<Config>,
) -> Result<Json<ProofParameters>, DIDError> {
    get_proof_parameters(config, PathBuf::from("/.well-known/did.json"))
}

/// Retrieve DID document.
///
/// - `config` Global Rocket configuration
/// - `id` - requested id, e.g. `alice`
/// - returns JSON encoded DID document
#[get("/<id..>")]
fn get(
    config: &rocket::State<Config>,
    id: PathBuf,
) -> (ContentType, Result<Json<Document>, DIDError>) {
    // TODO: verify that the DID in the doc is equal to the DID that has been requested - bail out otherwise
    // TODO: maybe return json_api for errors?
    match DIDWeb::did_from_config(config, &id) {
        Ok(did) => Some(did),
        Err(_) => None,
    };
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

// Required to explicitly allow access to a path starting with "."
#[get("/.well-known/did.json")]
fn get_wellknown(
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
#[post("/<id..>", data = "<doc>")]
fn create(
    config: &rocket::State<Config>,
    id: PathBuf,
    doc: Json<Document>,
) -> Result<CustomStatus<Json<ProofParameters>>, DIDError> {
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
        .map(CustomStatus::Created)
}

/// Updates a DID Document if the identity is authorized to perform this operation.
///
/// * `presentation` - verifable presentation that holds the updated DID Document
///
/// # TODO
///
/// * implement .well-known support
#[put("/<id..>", data = "<presentation>")]
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

    // if there's no valid presentation by the user, see if the owner signed the presentation
    println!(
        "proof parameters, challenge: {}",
        proof_parameters.challenge
    );
    println!("proof parameters, domain: {}", proof_parameters.domain);
    let opts = LinkedDataProofOptions {
        challenge: Some(proof_parameters.challenge.to_string()),
        domain: Some(proof_parameters.domain.to_string()),
        proof_purpose: Some(ssi::vc::ProofPurpose::Authentication),
        // created: xx; // this is set to now_ms, not sure if that's correct .. I guess that is should have be created max a minute ago
        ..Default::default()
    };

    let resolver = config.reslover_options.get_resolver();
    #[cfg(test)]
    let std_resolvers = resolver;
    #[cfg(test)]
    let test_resolver = DIDWebTestResolver {
        store: Some(&config.store),
        ..DIDWebTestResolver::default()
    };
    #[cfg(test)]
    let resolver = SeriesResolver {
        resolvers: vec![&test_resolver, &std_resolvers],
    };

    // TODO: test if all the containing credentials are also fully verified
    // - [x] ensure that signatures are correct
    // - [-] are the options also applied to every single credential or do they need to be reapplied?
    let result = presentation.verify(Some(opts), &resolver).await;
    println!("checks {}", result.checks.len());
    println!("warn {}", result.warnings.len());
    println!("errors {}", result.errors.len());
    println!("errors: {}", result.errors.join("; "));

    if !result.errors.is_empty() {
        return Err(DIDError::PresentationInvalid(
            "Presentation invalid, verification failed".to_string(),
        ));
    }

    // - [ ] find the credential that was issued by the DID itself .. or the controlling DID?
    // - [ ] ensure that the DID is the subject of the DID Document

    // ensure there's A valid did document in the credentials
    let (vc, diddoc) = presentation
        .verifiable_credential
        .as_ref()
        .and_then(|vcs| {
            println!("evaluating");
            vcs.into_iter()
                .map(|credential| match credential {
                    CredentialOrJWT::Credential(credential) => {
                        credential.credential_subject.clone().into_iter().fold(
                            None,
                            |acc, credential_subject| {
                                // - [ ] wie stelle ich den Typ eines DidDocs fest? as fehlt irgendwie in der DID Doc definition
                                // - [x] und prüfen, ob die ID die eigene ID ist
                                let id_equals_proof_parameter_did =
                                    credential_subject.id.as_ref().and_then(|id| {
                                        println!("credential subject: {:?}", id.to_string());
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
                                    Some((credential, credential_subject))
                                } else {
                                    acc
                                }
                            },
                        )
                    }
                    CredentialOrJWT::JWT(_) => {
                        println!("credential jwt");
                        // ignore JWT credentials
                        None
                    }
                })
                .fold(
                    None,
                    |acc, credential| if acc.is_none() { credential } else { acc },
                )
        })
        .ok_or_else(|| DIDError::DIDDocMissing("No valid DID Doc credential found".to_string()))?;

    // ensure that inssuance_date is not in the future
    match &vc.issuance_date {
        Some(issuance_date) => {
            let issuance_date = issuance_date.clone();
            match DateTime::parse_from_rfc3339(&String::from(issuance_date)) {
                Ok(issuance_date) => {
                    let now = Utc::now();
                    if issuance_date < now {
                        Ok(true)
                    } else {
                        Err(DIDError::PresentationInvalid(
                            "Presentation invalid, DID Doc credential has been issued in the future".to_string(),
                        ))
                    }
                }
                _ => Err(DIDError::PresentationInvalid(
                    "Presentation invalid, DID Doc credential has been issued in the future"
                        .to_string(),
                )),
            }
        }
        None => Ok(true),
    }?;

    // verify expiration_date as it's not verified by the verify call https://github.com/spruceid/ssi/issues/470
    match &vc.expiration_date {
        Some(expiration_date) => {
            let expiration_date = expiration_date.clone();
            match DateTime::parse_from_rfc3339(&String::from(expiration_date)) {
                Ok(expiration_date) => {
                    let now = Utc::now();
                    if expiration_date > now {
                        Ok(true)
                    } else {
                        Err(DIDError::PresentationInvalid(
                            "Presentation invalid, DID Doc credential expired".to_string(),
                        ))
                    }
                }
                _ => Err(DIDError::PresentationInvalid(
                    "Presentation invalid, DID Doc credential expired".to_string(),
                )),
            }
        }
        None => Ok(true),
    }?;
    // TODO: verify not before use - applies only to JWT claims

    // unsure how to easily convert a CredentialSubject into a Document. Via json encoding? - not
    // beautiful!!
    let diddoc = serde_json::to_string(&diddoc)
        .ok()
        // .map(log("json"))
        .and_then(|s| serde_json::from_str::<Document>(&s).ok());
    match diddoc {
        Some(document) => config
            .store
            .update(&id, document)
            .and_then(|doc| ProofParameters::new(config, &doc))
            .map_err(log("post, got error:"))
            .map(Json),
        None => Err(DIDError::DIDDocMissing("DID Doc invalid".to_string())),
    }
}

/// Deletes a DID Document if the identity is authorized to perform this operation.
///
/// * `presentation` - verifable presentation that holds the updated DID Document
///
/// # TODO
///
/// * Implement authorization - admin + user who can delete their own id .. good?
#[delete("/<id..>")]
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
    ship(Config::load_env_or_panic(Config::default()))
}

// Workaround for tests to start with a different configuration not derived from environment
// variables
fn ship(config: Config) -> rocket::Rocket<rocket::Build> {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(rocket::Config::default()))
        // .merge(Toml::file("Didwebserver.toml").nested())
        .merge(Env::prefixed("DID_SERVER_").global())
        .select(Profile::from_env_or("DID_SERVER_PROFILE", "default"));
    rocket::custom(figment).manage(config).mount(
        "/",
        routes![
            create,
            delete,
            get,
            get_proof_parameters,
            get_proof_parameters_wellknown,
            get_wellknown,
            update,
        ],
    )
}
