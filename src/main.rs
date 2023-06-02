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
use crate::utils::{log, verify_issuer};
use rocket::figment::providers::{Env, Serialized};
use rocket::figment::{Figment, Profile};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use ssi::did::{Document, VerificationRelationship};
use ssi::vc::Presentation;
use std::path::PathBuf;
use utils::verify_presentation;

#[cfg(test)]
mod test;
#[cfg(test)]
mod test_resolver;

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
    ProofParameters::new(config, &id).map(Json)
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

/// Creates a DID document at the given position. The DID Document's id must match the DID of
/// computed DID at this position otherwise the DID wouldn't be manageable. Only the server's owner is allowed to
/// create / register new DID documents.
///
/// * `config` - the server configuration.
/// * `id` - path to the identity.
/// * `presentation` - verifable presentation that holds the updated DID Document.
/// - returns DID as JSON string
///
/// # TODO
///
/// * Implement authentication via some fitting method, JWT or actual signed requests via a private
///   key
/// * Support subfolder so that the DIDs don't only have to live in the top-level folder
/// * implement .well-known support
#[post("/<id..>", data = "<presentation>")]
// #[post("/<id..>", data = "<presentation>")]
async fn create(
    config: &rocket::State<Config>,
    id: PathBuf,
    presentation: Json<Presentation>,
) -> Result<CustomStatus<Json<ProofParameters>>, DIDError> {
    // only the server's owner is allowed to create DIDs
    let controlling_did = &config.owner;
    verify_issuer(
        config,
        controlling_did,
        VerificationRelationship::AssertionMethod,
        &presentation,
    )
    .await?;
    let proof_parameters = ProofParameters::new(config, &id)?;
    let (_result, _vc, did_doc) =
        verify_presentation(config, proof_parameters, presentation).await?;

    // INFO: unsure how to easily convert a CredentialSubject into a Document. Via json encoding? - not beautiful!!
    let did_doc = serde_json::to_string(&did_doc)
        .ok()
        // .map(log("json"))
        .and_then(|s| serde_json::from_str::<Document>(&s).ok());
    match did_doc {
        Some(document) => config
            .store
            .create(&id, document)
            .and_then(|_| ProofParameters::new(config, &id))
            .map_err(log("post, got error:"))
            .map(Json)
            .map(CustomStatus::Created),
        None => Err(DIDError::DIDDocMissing("DID Doc invalid".to_string())),
    }
}

/// Updates a DID Document if the identity is authorized to perform this operation.
///
/// * `config` - the server configuration.
/// * `id` - path to the identity.
/// * `presentation` - verifable presentation that holds the updated DID Document.
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
    // The user is the only one allowed to update the personal DID document
    let controlling_did = DIDWeb::from_config(&config, &id)?.to_string();
    verify_issuer(
        config,
        &controlling_did,
        VerificationRelationship::AssertionMethod,
        &presentation,
    )
    .await?;

    // retrieve proof parameters required to verify the correctness of the presentation
    let proof_parameters = ProofParameters::new(config, &id)?;
    let (_result, _vc, did_doc) =
        verify_presentation(config, proof_parameters, presentation).await?;

    // INFO: unsure how to easily convert a CredentialSubject into a Document. Via json encoding? - not beautiful!!
    let did_doc = serde_json::to_string(&did_doc)
        .ok()
        // .map(log("json"))
        .and_then(|s| serde_json::from_str::<Document>(&s).ok());
    match did_doc {
        Some(document) => config
            .store
            .update(&id, document)
            .and_then(|_| ProofParameters::new(config, &id))
            .map_err(log("post, got error:"))
            .map(Json),
        None => Err(DIDError::DIDDocMissing("DID Doc invalid".to_string())),
    }
}

/// Deletes a DID Document if the identity is authorized to perform this operation. Currently, only the owner of the
/// server is allowed to delete DID Documents.
///
/// # Arguments
///
/// * `config` - the server configuration.
/// * `id` - path to the identity.
/// * `presentation` - verifable presentation that holds the updated DID Document.
#[delete("/<id..>", data = "<presentation>")]
async fn delete(
    config: &rocket::State<Config>,
    id: PathBuf,
    presentation: Json<Presentation>,
) -> Result<Json<ProofParameters>, DIDError> {
    // only the server's owner is allowed to create DIDs
    let controlling_did = &config.owner;
    verify_issuer(
        config,
        controlling_did,
        VerificationRelationship::AssertionMethod,
        &presentation,
    )
    .await?;
    let proof_parameters = ProofParameters::new(config, &id)?;
    verify_presentation(config, proof_parameters, presentation).await?;
    config
        .store
        .remove(&id)
        .and_then(|_| ProofParameters::new(config, &id))
        .map_err(log("delete, got error:"))
        .map(Json)
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
