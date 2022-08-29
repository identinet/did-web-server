// TODO: Things to consider:
//
// * [ ] compatibility with the did:web API .. not sure if there's much imposed: https://w3c-ccg.github.io/did-method-web/
// * [+] return content type application/did+json for get requests
// * [ ] support .well-known/did.json documents
// * [ ] support subpaths for storing DIDs
// * [ ] ensure that the web-did-resolver test suite passes: https://github.com/decentralized-identity/web-did-resolver/blob/master/src/__tests__/resolver.test.ts
// * [ ] rethink the API of the service, maybe reserve the admin operations under a specific API endpoint while keeping the user-focused operations at the root level so that the interaction directly happens as intended .. but maybe also the admin operations should happen there
// * [ ] support additional storage mechanims other than the file system
// * [ ] maybe add support for a history of documents, including a history of operations and authentications that were used for audit purposes
// * [ ] think about document integrity verification as proposed here: https://w3c-ccg.github.io/did-method-web/#did-document-integrity-verification

mod config;
mod did;
mod error;
mod store;
mod util;

// use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::did::{DIDContentTypes, DIDWeb};
use crate::error::DIDError;
use crate::store::get_filename_from_id;
use crate::util::{get_env, log};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use ssi::did::Document;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str;

#[macro_use]
extern crate rocket;

/// Retrieve DID document.
///
/// - `id` - requested id, e.g. `alice`
/// - returns JSON encoded DID document
///
/// # TODO
///
/// * implement access to subdirectories, e.g. sales/<id>, admin/<id> .. if that's necessary
#[get("/v1/web/<id..>")]
fn get(
    config: &rocket::State<Config>,
    id: PathBuf,
) -> (ContentType, Result<Json<Document>, DIDError>) {
    // Content Type for application/did+json
    // TODO: understand how to generate this content type at compile time rather than runtime
    // TODO: maybe return json_api for errors?

    let result = get_filename_from_id(&config.didstore, &id)
        // .map(|f| {
        //     f.to_str().map(log("path"));
        //     f
        // })
        .map_err(|e| DIDError::DIDNotFound(e.to_string()))
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
        // .and_then(|ref d: Document| {
        //     serde_json::to_string(d).map_err(|e| MyErrors::ConversionError(e.to_string()))1. [x] identinet: Work on did:web based file hosting service - get the service going with the integration of the SSI library
        // })
        .map_err(log("get, got error:"));

    // Apparently, DID documents in the ssi implementation require @context to be present while
    // it's optional in the spec, see https://w3c.github.io/did-core/#iana-considerations
    // See https://github.com/spruceid/ssi/issues/458
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

#[get("/.well-known/did.json")]
fn getwellknown(config: &rocket::State<Config>) -> (ContentType, Result<Json<Document>, DIDError>) {
    get(config, PathBuf::from("/.well-known/did.json"))
}

#[get("/<id..>")]
fn getroot(
    config: &rocket::State<Config>,
    id: PathBuf,
) -> (ContentType, Result<Json<Document>, DIDError>) {
    get(config, id)
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
#[post("/v1/web/<id..>", data = "<doc>")]
fn create(
    config: &rocket::State<Config>,
    id: PathBuf,
    doc: Json<Document>,
) -> Result<Json<String>, DIDError> {
    // guard to ensure that the DID is in general manageable
    println!("starting post");
    let computed_did =
        match DIDWeb::new(&config.hostname, &config.port, &config.did_method_path, &id) {
            Ok(did) => did,
            Err(e) => return Err(e),
        };
    println!("computed did {}", computed_did);
    if doc.id != format!("{}", computed_did) {
        return Err(DIDError::DIDMismatch(format!(
            "DIDs don't match - expected: {} received: {}",
            computed_did, doc.id
        )));
    }
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
                    serde_json::to_string(&doc.into_inner())
                        .map_err(|e| DIDError::ContentConversion(e.to_string()))
                        .and_then(|s| {
                            f.write(s.as_bytes())
                                .map_err(|e| DIDError::NoFileWrite(e.to_string()))
                        })
                })
        })
        .map_err(log("post, got error:"))
        // Return the DID
        .map(|_| Json(computed_did.to_string()))
}

/// Updates a DID Document if the identity is authorized to perform this operation.
///
/// * `presentation` - verifable presentation that holds the updated DID Document
///
/// # TODO
#[put("/v1/web/<id>/did.json", data = "<_presentation>")]
fn update(id: &str, _presentation: &str) -> String {
    format!("Did doc: did:web:identinet.io:vc/{}/did.json", id)
    // import VP from SSI lib
    // like create, partial updates aren't supported
    // This method requires that the DID already exists .. if the DID doesn't exist POST has to be
    // used instead
    // When using this endpoint, the user needs to authenticate with a signed DID document in a VP
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
    let computed_did =
        match DIDWeb::new(&config.hostname, &config.port, &config.did_method_path, &id) {
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
            get_env("EXTERNAL_PORT", "8080"),
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
            routes![get, getroot, getwellknown, create, update, delete],
        )
}
