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
use crate::utils::{
    compare_date, ensure_proof_matches_verification_method, get_did_doc_from_presentation, log,
};
use chrono::Utc;
use rocket::figment::providers::{Env, Serialized};
use rocket::figment::{Figment, Profile};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use ssi::did::{Document, VerificationRelationship};
use ssi::vc::{LinkedDataProofOptions, Presentation};
use std::cmp::Ordering;
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
// #[post("/<id..>", data = "<presentation>")]
fn create(
    config: &rocket::State<Config>,
    id: PathBuf,
    doc: Json<Document>,
    // presentation: Json<Presentation>,
) -> Result<CustomStatus<Json<ProofParameters>>, DIDError> {
    let _proof_parameters = ProofParameters::without_challenge(config, config.owner.clone());
    // TODO the presentation must be signed by the owner's DID / holder
    // TODO the VC must be a valid DID doc for the ID + it must also be signed by the owner

    // guard to ensure that the DID is manageable
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
/// * [ ] implement .well-known support
#[put("/<id..>", data = "<presentation>")]
async fn update(
    config: &rocket::State<Config>,
    id: PathBuf,
    presentation: Json<Presentation>,
) -> Result<Json<ProofParameters>, DIDError> {
    // retrieve proof parameters required to verify the correctness of the presentation
    let did_doc = config.store.get(&id)?;
    let proof_parameters = ProofParameters::new(config, &did_doc)?;

    ensure_proof_matches_verification_method(
        &did_doc,
        VerificationRelationship::AssertionMethod,
        &presentation.proof,
    )?;

    // TODO: maybe: if there's no valid presentation by the user, see if the owner signed the presentation

    // TODO: remove debug output
    match proof_parameters.challenge {
        Some(ref v) => println!("proof parameters, challenge: {}", v),
        None => println!("proof parameters, challenge: no challenge present"),
    }

    let opts = LinkedDataProofOptions {
        challenge: Some(proof_parameters.challenge.unwrap()), // fail if challenge is not present
        domain: Some(proof_parameters.domain.to_string()),
        proof_purpose: Some(ssi::vc::ProofPurpose::Authentication),
        // created: xx; // TODO this is set to now_ms, not sure if that's correct .. I guess that is should have be created max a minute ago
        ..LinkedDataProofOptions::default()
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

    // TODO: test if all the containing credentials are also fully verified - is this poperly done by the verify call?
    // - [x] ensure that signatures are correct
    // - [-] are the options also applied to every single credential or do they need to be reapplied?
    let result = presentation.verify(Some(opts), &resolver).await;

    // TODO: remove debug output
    println!("checks {}", result.checks.len());
    println!("warn {}", result.warnings.len());
    println!("errors {}", result.errors.len());
    println!("errors: {}", result.errors.join(", "));

    if !result.errors.is_empty() {
        return Err(DIDError::PresentationInvalid(
            "Presentation invalid, verification failed".to_string(),
        ));
    }

    // TODO: ?
    // - [ ] find the credential that was issued by the DID itself .. or the controlling DID?
    // - [ ] ensure that the DID is the subject of the DID Document

    let presentation = presentation.into_inner();
    let (vc, did_doc) = get_did_doc_from_presentation(&presentation, proof_parameters.did)?;

    // ensure that inssuance_date is not in the future
    compare_date(&vc.issuance_date, Ordering::Less, Utc::now()).ok_or_else(|| {
        DIDError::PresentationInvalid(
            "Presentation invalid, DID Doc credential has been issued in the future or has no issuance date".to_string(),
            ) })?;

    // verify expiration_date as it's not verified by verify() https://github.com/spruceid/ssi/issues/470
    match &vc.expiration_date {
        Some(expiration_date) => compare_date(
            &Some(expiration_date.clone()),
            Ordering::Greater,
            Utc::now(),
        )
        .ok_or_else(|| {
            DIDError::PresentationInvalid(
                "Presentation invalid, DID Doc credential has expired".to_string(),
            )
        }),
        _ => Ok(Ordering::Greater),
    }?;

    // TODO: verify "not before use" date - applies only to JWT claims

    // FIXME: unsure how to easily convert a CredentialSubject into a Document. Via json encoding? - not beautiful!!
    let did_doc = serde_json::to_string(&did_doc)
        .ok()
        // .map(log("json"))
        .and_then(|s| serde_json::from_str::<Document>(&s).ok());
    match did_doc {
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
