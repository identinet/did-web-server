mod utils;

use super::ship;
use crate::config::Config;
use crate::did::ProofParameters;
use crate::test_resolver::DIDWebTestResolver;
use lazy_static::lazy_static;
use rocket::http::Status;
use rocket::local::blocking::Client;
use ssi::did::Document;
use ssi::did_resolve::SeriesResolver;
use ssi::jwk::{OctetParams, Params, JWK};
use ssi::ldp::ProofSuiteType;
use ssi::one_or_many::OneOrMany;
use ssi::vc::{LinkedDataProofOptions, VCDateTime, URI};
use std::path::PathBuf;
use std::str::FromStr;

lazy_static! {
    static ref OWNER: &'static str = "did:key:z6MksRCeBVzFcsnR4Ao7YurYSJEVxNzUPnBNkXAcQdvwmwLR";
    static ref NOT_OWNER: &'static str = "did:key:z6MketjFUmQyWfJUjD21peHqsxreL8VCvwnKoCcVKRWqSWCm";
    static ref NOT_OWNER_VERIFICATION_METHOD: &'static str = "did:key:z6MketjFUmQyWfJUjD21peHqsxreL8VCvwnKoCcVKRWqSWCm#z6MketjFUmQyWfJUjD21peHqsxreL8VCvwnKoCcVKRWqSWCm";
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
    // TODO: create a credential and a presentation for the DID
    // TODO: pass the owner to the server
    let filename = "./src/__fixtures__/valid-did.json";
    let doc = utils::read_file(filename);
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
    let doc = utils::read_file(filename);
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
    let doc = utils::read_file(filename);
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
        Status::Forbidden,
        "When DID exists in store and is created again, then return 403 - forbidden."
    );

    // TODO: create as non-owner: When create is attempted by a DID that's not the owner, then return 401 - unauthorized.
    // TODO: create a did document in the wrong location: When a valid presentation and DID document is sent by the owner but the location of the document is wrong, then return 400 - bad request.
}

#[test]
fn integration_create_invalid_id() {
    let client = Client::tracked(ship(Config {
        owner: OWNER.to_string(),
        ..Config::default()
    }))
    .expect("valid rocket instance");

    // create with invalid id
    // ----------------------
    let filename = "./src/__fixtures__/invalid-diddoc.json";
    let doc = utils::read_file(filename);
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

    let resolver_config = Config {
        owner: OWNER.to_string(),
        ..Config::default()
    };
    let std_resolvers = resolver_config.reslover_options.get_resolver();
    let test_resolver = DIDWebTestResolver {
        client: Some(&client),
        ..DIDWebTestResolver::default()
    };
    let resolver = SeriesResolver {
        resolvers: vec![&test_resolver, &std_resolvers],
    };

    let filename = "./src/__fixtures__/not-owner.jwk";
    let key_not_owner = utils::read_file(filename);
    assert!(
        key_not_owner.is_ok(),
        "When a fixture is read, then it's returned successfully."
    );
    let key_not_owner = key_not_owner.unwrap();
    let key_not_owner = JWK::from(Params::OKP(
        serde_json::from_str::<OctetParams>(&key_not_owner).unwrap(),
    ));
    // create
    // ------
    let filename = "./src/__fixtures__/valid-did.json";
    let doc = utils::read_file(filename);
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
            proof_parameters.challenge.unwrap(), "d992a52400965351e261fdcfa47469cb3e0fa06cc658208c3c95bddf577dc29a",
            "When DID is created in store, then the challenge is set to a unique and deterministic value."
        );

    // update
    // ------
    // Fetch new proof parameters
    let response = client
        .get(uri!(super::get_proof_parameters(
            id = PathBuf::from("valid-did/did.json"),
        )))
        .dispatch()
        .await;
    let proof_parameters = response.into_json::<ProofParameters>().await.unwrap();
    // build a credential from the did document
    let filename = "./src/__fixtures__/valid-did.jwk";
    let key = utils::read_file(filename);
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
    let mut attributes = utils::json_file_to_attributes_or_panic(filename);
    let id = match attributes.remove("id").unwrap() {
        rocket::serde::json::serde_json::Value::String(id) => Some(id),
        _ => None,
    }
    .unwrap();
    let credential = utils::create_credential_or_panic(
        &proof_parameters.did,
        &id,
        "https://example.com/vc/123",
        Some(attributes),
        None,
        None,
        &resolver,
        "did:web:localhost%3A8000:valid-did#controller",
        &key,
    )
    .await;
    // build a presentation from the credential
    let presentation = utils::create_presentation_or_panic(
        &proof_parameters.did,
        OneOrMany::One(ssi::vc::CredentialOrJWT::Credential(credential)),
        &LinkedDataProofOptions {
            type_: Some(ProofSuiteType::Ed25519Signature2020),
            domain: Some(proof_parameters.domain.to_string()),
            challenge: Some(proof_parameters.challenge.unwrap()),
            proof_purpose: Some(proof_parameters.proof_purpose.to_owned()),
            verification_method: Some(URI::String(
                "did:web:localhost%3A8000:valid-did#controller".to_string(),
            )),
            ..LinkedDataProofOptions::default()
        },
        &resolver,
        &key,
    )
    .await;
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

    // Test expired DID Doc
    // --------------------
    // Fetch new proof parameters
    let response = client
        .get(uri!(super::get_proof_parameters(
            id = PathBuf::from("valid-did/did.json"),
        )))
        .dispatch()
        .await;
    let proof_parameters = response.into_json::<ProofParameters>().await.unwrap();
    // build a credential from the did document
    let filename = "./src/__fixtures__/valid-did_update.json";
    let mut attributes = utils::json_file_to_attributes_or_panic(filename);
    let id = match attributes.remove("id").unwrap() {
        rocket::serde::json::serde_json::Value::String(id) => Some(id),
        _ => None,
    }
    .unwrap();
    let credential = utils::create_credential_or_panic(
        &proof_parameters.did,
        &id,
        "https://example.com/vc/123",
        Some(attributes),
        Some(VCDateTime::from_str("2019-12-31T01:01:00Z").unwrap()),
        Some(VCDateTime::from_str("2020-01-01T01:01:00Z").unwrap()),
        &resolver,
        "did:web:localhost%3A8000:valid-did#controller",
        &key,
    )
    .await;
    // build a presentation from the credential
    let presentation = utils::create_presentation_or_panic(
        &proof_parameters.did,
        OneOrMany::One(ssi::vc::CredentialOrJWT::Credential(credential)),
        &LinkedDataProofOptions {
            type_: Some(ProofSuiteType::Ed25519Signature2020),
            domain: Some(proof_parameters.domain.to_string()),
            challenge: Some(proof_parameters.challenge.unwrap()),
            proof_purpose: Some(proof_parameters.proof_purpose.to_owned()),
            verification_method: Some(URI::String(
                "did:web:localhost%3A8000:valid-did#controller".to_string(),
            )),
            ..LinkedDataProofOptions::default()
        },
        &resolver,
        &key,
    )
    .await;
    let presentation_string = serde_json::to_string(&presentation).unwrap();
    // println!("presentation: {}", presentation_string);
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
        Status::Unauthorized,
        "When Presentation with expired VC is sent, then 401 - Unauthorized is returned."
    );

    // update without DIDDoc VC
    // ------------------------
    // Fetch new proof parameters
    let response = client
        .get(uri!(super::get_proof_parameters(
            id = PathBuf::from("valid-did/did.json"),
        )))
        .dispatch()
        .await;
    let proof_parameters = response.into_json::<ProofParameters>().await.unwrap();
    // build a credential from the did document
    let credential = utils::create_credential_or_panic(
        &proof_parameters.did,
        &id,
        "https://example.com/vc/123",
        None,
        None,
        None,
        &resolver,
        "did:web:localhost%3A8000:valid-did#controller",
        &key,
    )
    .await;
    // build a presentation from the credential
    let presentation = utils::create_presentation_or_panic(
        &proof_parameters.did,
        OneOrMany::One(ssi::vc::CredentialOrJWT::Credential(credential)),
        &LinkedDataProofOptions {
            type_: Some(ProofSuiteType::Ed25519Signature2020),
            domain: Some(proof_parameters.domain.to_string()),
            challenge: Some(proof_parameters.challenge.unwrap()),
            proof_purpose: Some(proof_parameters.proof_purpose.to_owned()),
            verification_method: Some(URI::String(
                "did:web:localhost%3A8000:valid-did#controller".to_string(),
            )),
            ..LinkedDataProofOptions::default()
        },
        &resolver,
        &key,
    )
    .await;
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
            Status::BadRequest,
            "When a valid Presentation without a DIDDoc VC is sent, then 401 - Unauthorized is returned."
        );

    // update with non-matching ID in DID document
    // -------------------------------------------
    // Fetch new proof parameters
    let response = client
        .get(uri!(super::get_proof_parameters(
            id = PathBuf::from("valid-did/did.json"),
        )))
        .dispatch()
        .await;
    let proof_parameters = response.into_json::<ProofParameters>().await.unwrap();
    // build a credential from the did document
    let filename = "./src/__fixtures__/valid-did_update-invalid.json";
    let mut attributes = utils::json_file_to_attributes_or_panic(filename);
    let id = match attributes.remove("id").unwrap() {
        rocket::serde::json::serde_json::Value::String(id) => Some(id),
        _ => None,
    }
    .unwrap();
    let credential = utils::create_credential_or_panic(
        &proof_parameters.did,
        &id,
        "https://example.com/vc/123",
        Some(attributes),
        None,
        None,
        &resolver,
        "did:web:localhost%3A8000:valid-did#controller",
        &key,
    )
    .await;
    println!("cred: {}", id);
    // build a presentation from the credential
    let presentation = utils::create_presentation_or_panic(
        &proof_parameters.did,
        OneOrMany::One(ssi::vc::CredentialOrJWT::Credential(credential)),
        &LinkedDataProofOptions {
            type_: Some(ProofSuiteType::Ed25519Signature2020),
            domain: Some(proof_parameters.domain.to_string()),
            challenge: Some(proof_parameters.challenge.unwrap()),
            proof_purpose: Some(proof_parameters.proof_purpose.to_owned()),
            verification_method: Some(URI::String(
                "did:web:localhost%3A8000:valid-did#controller".to_string(),
            )),
            ..LinkedDataProofOptions::default()
        },
        &resolver,
        &key,
    )
    .await;
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
            Status::BadRequest,
            "When a valid Presentation with a non-matching subject in the DID Doc credential is sent, then 400 - Bad Request is returned."
        );

    // Attempted update with holder not matching DID Doc ID
    // -------------------------------------------
    // Fetch new proof parameters
    let response = client
        .get(uri!(super::get_proof_parameters(
            id = PathBuf::from("valid-did/did.json"),
        )))
        .dispatch()
        .await;
    let proof_parameters = response.into_json::<ProofParameters>().await.unwrap();
    // build a credential from the did document
    let filename = "./src/__fixtures__/valid-did_update.json";
    let mut attributes = utils::json_file_to_attributes_or_panic(filename);
    let id = match attributes.remove("id").unwrap() {
        rocket::serde::json::serde_json::Value::String(id) => Some(id),
        _ => None,
    }
    .unwrap();
    let credential = utils::create_credential_or_panic(
        &proof_parameters.did,
        &id,
        "https://example.com/vc/123",
        Some(attributes),
        None,
        None,
        &resolver,
        "did:web:localhost%3A8000:valid-did#controller",
        &key,
    )
    .await;
    // build a presentation from the credential
    let presentation = utils::create_presentation_or_panic(
        &NOT_OWNER,
        OneOrMany::One(ssi::vc::CredentialOrJWT::Credential(credential)),
        &LinkedDataProofOptions {
            type_: Some(ProofSuiteType::Ed25519Signature2020),
            domain: Some(proof_parameters.domain.to_string()),
            challenge: Some(proof_parameters.challenge.unwrap()),
            proof_purpose: Some(proof_parameters.proof_purpose.to_owned()),
            verification_method: Some(URI::String(NOT_OWNER_VERIFICATION_METHOD.to_string())),
            ..LinkedDataProofOptions::default()
        },
        &resolver,
        &key_not_owner,
    )
    .await;
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
            Status::Unauthorized,
            "When a valid Presentation and DID Doc is sent but the Presentation holder doesn't match the DID Doc id, then 403 - Unauthorized is returned."
        );

    // TODO Test When the owner of the server sends a valid update of a DID doc, then the DID Doc is successfully updated.
    // TODO Test When the owner of the server sends a valid Presentation but the DID Doc id doesn't match the actual DID, then 400 - Bad Request is returned.
}
