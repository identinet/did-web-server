mod config;
mod did;
mod error;
mod store;
mod util;

// use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::did::DIDWeb;
use crate::error::DIDError;
use crate::store::get_filename_from_id;
use crate::util::{get_env, log};
use rocket::serde::json::Json;
use ssi::did::Document;
use std::fs;
use std::io::prelude::*;
use std::str;

#[macro_use]
extern crate rocket;

// trait CustomAndThen<T, E> {
//     fn and_then2<U, E2, F: FnOnce(T) -> Result<U, E2>>(self, op: F) -> Result<U, E>
//     where
//         E: std::convert::From<E2>;
// }

// impl<T, E> CustomAndThen<T, E> for Result<T, E> {
//     fn and_then2<U, E2, F: FnOnce(T) -> Result<U, E2>>(self, op: F) -> Result<U, E>
//     where
//         E: std::convert::From<E2>,
//     {
//         match self {
//             Ok(t) => op(t).map_err(From::from),
//             Err(e) => Err(e),
//         }
//     }
// }

/// Retrieve DID documents.
///
/// * `id` - requested id, e.g. `alice`
///
/// # TODO
///
/// * implement access to subdirectories, e.g. sales/<id>, admin/<id> .. if that's necessary
#[get("/v1/web/<id>/did.json")]
fn get(config: &rocket::State<Config>, id: &str) -> Result<Json<Document>, DIDError> {
    get_filename_from_id(&config.didstore, id)
        .map_err(|e| DIDError::NoFileName(e.to_string()))
        // debugging:
        // .map(|f| {
        //     f.to_str().map(|ff| println!("f {}", ff));
        //     f
        // })
        .and_then(|filename| {
            fs::read(filename).map_err(|e| {
                // TODO: return default value, maybe
                DIDError::NoFileRead(e.to_string())
            })
        })
        .and_then(|b| String::from_utf8(b).map_err(|e| DIDError::ContentConversion(e.to_string())))
        .and_then(|ref s| {
            serde_json::from_str::<Document>(s)
                .map_err(|e| DIDError::ContentConversion(e.to_string()))
        })
        // .and_then(|ref d: Document| {
        //     serde_json::to_string(d).map_err(|e| MyErrors::ConversionError(e.to_string()))1. [x] identinet: Work on did:web based file hosting service - get the service going with the integration of the SSI library
        // })
        .map_err(log("got error"))
        .map(Json)
}

#[get("/<id>/did.json")]
fn getroot(config: &rocket::State<Config>, id: &str) -> Result<Json<Document>, DIDError> {
    get(config, id)
}

/// Creates a DID document at the given position. The DID Document's id must match the DID the
/// computed DID at this position.
///
/// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
/// - `doc` - JSON-LD DID Document as specified in https://w3c.github.io/did-core/
///
/// # TODO
///
/// * Implement authentication via some fitting method, JWT or actual signed requests via a private
///   key
/// * Verify that the DID in the Document matches the computed DID
#[post("/v1/web/<id>/did.json", data = "<doc>")]
fn create(
    config: &rocket::State<Config>,
    id: &str,
    doc: Json<Document>,
) -> Result<Json<String>, DIDError> {
    // TODO: create a function that
    // 1. is an extension of DID Methods by SSI lib
    // 2. takes the method's name
    // 3. takes a hostname
    // 4. takes an id - in the case of did:web the id would be either a string that's then split
    //    at : or a vector of strings.
    // 5. the parts of the id a matches against allowed characters
    // 6. then the whole DID is generated

    // TODO: chcek if DID in document matches computed DID

    // TODO: did aufbauen und dann zurckgeben lassen
    // println!(
    //     "did:web:d_{}:p_{}/id_{}/did.json",
    //     &config.domainname,
    //     &config.path,
    //     // format!("{}:{}", &config.path, id)
    //     id
    // );
    let computed_did = match DIDWeb::new(&config.domainname, &config.path, id) {
        Ok(did) => did,
        Err(e) => return Err(e),
    };
    // if doc.
    get_filename_from_id(&config.didstore, id)
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
        .map(|_| Json(computed_did.to_string()))
}

/// Updates a DID Document if the identity is authorized to perform this operation.
///
/// * `presentation` - verifable presentation that holds the updated DID Document
///
/// # TODO
/// Automatically determine the appropriate DID Document derived from the ID .. if that makes sense .. or
/// no?
#[put("/v1/web/<id>/did.json", data = "<_presentation>")]
fn update(id: &str, _presentation: &str) -> String {
    format!("Did doc: did:web:identinet.io:vc/{}/did.json", id)
    // import VP from SSI lib
}

#[delete("/v1/web/<id>/did.json")]
fn delete(id: &str) -> String {
    format!("Did doc: did:web:identinet.io:vc/{}/did.json", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Config::new(
            get_env("DOMAINNAME", "localhost"),
            get_env("SUBPATH", ""),
            get_env(
                "DID_STORE",
                // by default store all files in $PWD/did_store/
                &std::env::current_dir()
                    .map(|val| val.join("did_store").to_str().unwrap_or(".").to_string())
                    .unwrap_or_else(|_| ".".to_string()),
            ),
        ))
        .mount("/", routes![get, getroot, create, update, delete])
}
