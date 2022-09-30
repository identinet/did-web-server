mod config;
mod content_types;
mod did;
mod error;
mod resolver;
mod store;
mod util;

use crate::config::Config;
use crate::content_types::DIDContentTypes;
use crate::did::{DIDWeb, ProofParameters};
use crate::error::{CustomStatus, DIDError};
use crate::util::log;
use rocket::figment::providers::{Env, Serialized};
use rocket::figment::{Figment, Profile};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use ssi::did::Document;
use ssi::vc::{CredentialOrJWT, LinkedDataProofOptions, Presentation};
use std::path::PathBuf;

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

#[cfg(test)]
mod test {
    use super::ship;
    use crate::config::Config;
    use crate::did::ProofParameters;
    use crate::test_resolver::DIDWebTestResolver;
    use chrono::prelude::*;
    use lazy_static::lazy_static;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use ssi::did::Document;
    use ssi::jwk::{OctetParams, Params, JWK};
    use ssi::one_or_many::OneOrMany;
    use ssi::vc::VCDateTime;
    use ssi::vc::{Context, Contexts, Credential, LinkedDataProofOptions, Presentation, URI};
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    lazy_static! {
        static ref OWNER: &'static str = "did:key:z6MksRCeBVzFcsnR4Ao7YurYSJEVxNzUPnBNkXAcQdvwmwLR";
        static ref NOT_OWNER: &'static str =
            "did:key:z6MketjFUmQyWfJUjD21peHqsxreL8VCvwnKoCcVKRWqSWCm";
    }

    fn read_file(filename: &str) -> Result<String, String> {
        fs::read(PathBuf::from(filename))
            .map_err(|_| "file access failed".to_string())
            .and_then(|b| String::from_utf8(b).map_err(|_| "conversion failure".to_string()))
    }

    #[test]
    fn integration_get() {
        let client = Client::tracked(ship(Config {
            owner: OWNER.to_string(),
            ..Config::default()
        }))
        .expect("valid rocket instance");

        let response = client
            .get(uri!(super::get(id = PathBuf::from(".well-known/did.json"))))
            .dispatch();
        assert_eq!(
            response.status(),
            Status::NotFound,
            "When DID is not in the store, then return 404 - not found."
        );
    }

    #[test]
    fn integration_create() {
        let client = Client::tracked(ship(Config {
            owner: OWNER.to_string(),
            ..Config::default()
        }))
        .expect("valid rocket instance");

        // create
        // ------
        let filename = "./src/__fixtures__/valid-did.json";
        let doc = read_file(filename);
        assert!(
            doc.is_ok(),
            "When a fixture is read, then it's returned successfully."
        );
        let doc = doc.unwrap();
        let response = client
            .post(uri!(super::create(
                id = PathBuf::from("valid-did/did.json"),
            )))
            .body(doc)
            .dispatch();
        assert_eq!(
            response.status(),
            Status::Created,
            "When DID is created in store, then return 201 - created."
        );
        let res = response.into_json::<ProofParameters>();
        assert!(
            res.is_some(),
            "When DID is created in store, then ProofParameters are returned."
        );
        let res = res.unwrap();
        assert_eq!(
            res.domain, "localhost",
            "When DID is created in store, then the proof domain is 'localhost'."
        );

        // get
        // ---
        let filename = "./src/__fixtures__/valid-did.json";
        let doc = read_file(filename);
        assert!(
            doc.is_ok(),
            "When a fixture is read, then it's returned successfully."
        );
        let doc = doc.unwrap();
        let document = serde_json::from_str::<Document>(&doc).unwrap();
        let docstring = serde_json::to_string(&document).unwrap();
        let response = client
            .get(uri!(super::get(id = PathBuf::from("valid-did/did.json"))))
            .dispatch();
        assert_eq!(
            response.status(),
            Status::Ok,
            "When DID exists in the store, then return 200 - ok."
        );
        let res = response.into_json::<Document>();
        assert!(
            res.is_some(),
            "When DID exists in the store, then a DID Document is returned."
        );
        let res = res.unwrap();
        let res = serde_json::to_string(&res).unwrap();
        assert_eq!(
            res, docstring,
            "When DID was created in store, then the same document is returned as stored in the document."
        );

        // double create
        // -------------
        let filename = "./src/__fixtures__/valid-did.json";
        let doc = read_file(filename);
        assert!(
            doc.is_ok(),
            "When a fixture is read, then it's returned successfully."
        );
        let doc = doc.unwrap();
        let response = client
            .post(uri!(super::create(
                id = PathBuf::from("valid-did/did.json"),
            )))
            .body(doc)
            .dispatch();
        assert_eq!(
            response.status(),
            Status::BadRequest,
            "When DID exists in store and is created again, then return 400 - bad request."
        );

        // create with invalid id
        // ----------------------
        let filename = "./src/__fixtures__/invalid-diddoc.json";
        let doc = read_file(filename);
        assert!(
            doc.is_ok(),
            "When a fixture is read, then it's returned successfully."
        );
        let doc = doc.unwrap();
        let response = client
            .post(uri!(super::create(
                id = PathBuf::from("invalid-diddoc/did.json"),
            )))
            .body(doc)
            .dispatch();
        assert_eq!(
            response.status(),
            Status::BadRequest,
            "When DID doesn't exist in store but the DID doesn't match the expected did, then return 400 - bad request."
        );
    }

    #[rocket::async_test]
    async fn integration_update() {
        use rocket::local::asynchronous::Client;
        let config = Config {
            owner: OWNER.to_string(),
            ..Config::default()
        };
        let client = Client::tracked(ship(config))
            .await
            .expect("valid rocket instance");

        // create
        // ------
        let filename = "./src/__fixtures__/valid-did.json";
        let doc = read_file(filename);
        assert!(
            doc.is_ok(),
            "When a fixture is read, then it's returned successfully."
        );
        let doc = doc.unwrap();
        let response = client
            .post(uri!(super::create(
                id = PathBuf::from("valid-did/did.json"),
            )))
            .body(doc)
            .dispatch()
            .await;
        assert_eq!(
            response.status(),
            Status::Created,
            "When DID is created in store, then return 201 - created."
        );
        let proof_parameters = response.into_json::<ProofParameters>().await;
        assert!(
            proof_parameters.is_some(),
            "When DID is created in store, then ProofParameters are returned."
        );
        let proof_parameters = proof_parameters.unwrap();
        assert_eq!(
            proof_parameters.domain, "localhost",
            "When DID is created in store, then the proof domain is 'localhost'."
        );
        assert_eq!(
            proof_parameters.challenge, "d992a52400965351e261fdcfa47469cb3e0fa06cc658208c3c95bddf577dc29a",
            "When DID is created in store, then the challenge is set to a unique but deterministic value."
        );

        // update
        // ------
        let filename = "./src/__fixtures__/valid-did.jwk";
        let key = read_file(filename);
        assert!(
            key.is_ok(),
            "When a fixture is read, then it's returned successfully."
        );
        let key = key.unwrap();
        let key = JWK::from(Params::OKP(
            serde_json::from_str::<OctetParams>(&key).unwrap(),
        ));

        // build a credential from the did document
        let filename = "./src/__fixtures__/valid-did_update.json";
        let doc = read_file(filename);
        assert!(
            doc.is_ok(),
            "When a fixture is read, then it's returned successfully."
        );
        let doc = doc.unwrap();
        let mut attributes =
            serde_json::from_str::<HashMap<String, rocket::serde::json::serde_json::Value>>(&doc)
                .unwrap();
        let id = match attributes.remove("id").unwrap() {
            rocket::serde::json::serde_json::Value::String(id) => Some(id),
            _ => None,
        }
        .unwrap();
        let mut credential = Credential {
            context: Contexts::One(Context::URI(URI::String(
                "https://www.w3.org/2018/credentials/v1".to_string(),
            ))),
            id: Some(URI::String("https://example.com/vc/123".to_string())),
            type_: OneOrMany::One("VerifiableCredential".to_string()),
            credential_subject: OneOrMany::One(ssi::vc::CredentialSubject {
                id: Some(URI::String(id.to_string())),
                property_set: Some(attributes),
            }),
            issuer: Some(ssi::vc::Issuer::URI(URI::String(
                proof_parameters.did.to_owned(),
            ))),
            issuance_date: Some(VCDateTime::from(Utc::now())),
            proof: None,           // added later
            expiration_date: None, // TODO: test expired credential
            credential_status: None,
            terms_of_use: None,
            evidence: None,
            credential_schema: None,
            refresh_service: None,
            property_set: None,
        };
        let resolver = DIDWebTestResolver {
            client: Some(&client),
            ..DIDWebTestResolver::default()
        };
        let proof = match credential
            .generate_proof(
                &key,
                &LinkedDataProofOptions {
                    type_: Some("Ed25519Signature2018".to_string()),
                    proof_purpose: Some(ssi::vc::ProofPurpose::AssertionMethod),
                    verification_method: Some(URI::String(
                        "did:web:localhost%3A8000:valid-did#controller".to_string(),
                    )),
                    ..LinkedDataProofOptions::default()
                },
                &resolver,
            )
            .await
        {
            Ok(proof) => Ok(proof),

            Err(e) => {
                eprintln!("error, {}", e);
                Err(e)
            }
        }
        .unwrap();
        credential.add_proof(proof);
        // build a presentation from the credential
        let mut presentation = Presentation {
            holder: Some(URI::String(proof_parameters.did.to_owned())), // holder must be present, otherwise the presentation can't be verified
            verifiable_credential: Some(OneOrMany::One(ssi::vc::CredentialOrJWT::Credential(
                credential,
            ))),
            ..Presentation::default()
        };
        let proof = match presentation
            .generate_proof(
                &key,
                &LinkedDataProofOptions {
                    type_: Some("Ed25519Signature2018".to_string()),
                    domain: Some(proof_parameters.domain),
                    challenge: Some(proof_parameters.challenge),
                    proof_purpose: Some(proof_parameters.proof_purpose.to_owned()),
                    verification_method: Some(URI::String(
                        "did:web:localhost%3A8000:valid-did#controller".to_string(),
                    )),
                    ..LinkedDataProofOptions::default()
                },
                &resolver,
            )
            .await
        {
            Ok(proof) => Ok(proof),

            Err(e) => {
                eprintln!("error, {}", e);
                Err(e)
            }
        }
        .unwrap();
        presentation.add_proof(proof);
        let presentation_string = serde_json::to_string(&presentation).unwrap();
        // update did doc via presentation
        let response = client
            .put(uri!(super::update(
                id = PathBuf::from("valid-did/did.json"),
            )))
            .body(presentation_string)
            .dispatch()
            .await;
        assert_eq!(
            response.status(),
            Status::Ok,
            "When Presentation with updated DID document is sent to store, then the document is updated and 200 - ok is returned."
        );
    }
}
