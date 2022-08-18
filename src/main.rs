// TODO: implement file writing

// use serde::{Deserialize, Serialize};
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str;

#[macro_use]
extern crate rocket;

// TODO: change String to &'static str to avoid reallocation of the error on the heap
#[derive(Debug, Responder)]
enum DIDError {
    #[response(status = 500)] // InternalServerError
    ContenctConversion(String),
    #[response(status = 399)] // TODO: return a default value instead of an error code
    NoFileRead(String),
    #[response(status = 400)] // BadRequest
    NoFileName(String),
    #[response(status = 400)] // BadRequest
    DIDExists(String),
}

impl std::error::Error for DIDError {}

impl fmt::Display for DIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DIDError::ContenctConversion(e) => write!(f, "{}", e),
            DIDError::NoFileRead(e) => write!(f, "{}", e),
            DIDError::NoFileName(e) => write!(f, "{}", e),
            DIDError::DIDExists(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug)]
struct Config {
    domainname: String,
    subpath: String,
    didstore: String,
}

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct DIDDoc {
    id: String,
    #[serde(rename = "@context")]
    context: String, // how to add special characters like @context to the document
}

// impl fmt::Display for DIDDoc {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.id)
//     }
// }

// fn get(id: &str) -> String {
//     format!("{{y {}}}", DIDDoc {
//         id: id.to_string()
//     })
// }

fn get_env(varname: &str, default: &str) -> String {
    match std::env::var(varname) {
        Ok(value) => value,
        Err(_) => default.to_string(),
    }
}

/// Computes the absolute path to a file with json extension in a base
/// direcotory and an ID.
fn compute_filename<'a>(base_dir: &str, id: &str) -> Result<PathBuf, &'a str> {
    Path::new(id)
        .file_name()
        // DEBUG
        // .map(|x| {
        //     let y = x.to_str();
        //     match y {
        //         Some(s) => println!("res {}, {}", s, id),
        //         None => println!("nothing"),
        //     };
        //     x
        // })
        //  bail out if id contains more information than than the plain file name
        .and_then(|id_file| if id_file == id { Some(id_file) } else { None })
        .ok_or("id is not a file") // TODO: place error messages in a constant somewhere
        .and_then(|id_file| {
            let p = Path::new(base_dir).join(id_file).with_extension("json");
            if p.is_absolute() {
                Ok(p)
            } else {
                Err("Path not absolute")
            }
        })
}

/// Retrieve DID documents from the file system.
///
/// * `id` - requested id, e.g. `alice`
///
/// # TODO
///
/// * implement subdirectories
#[get("/<id>/did.json")]
fn get(config: &rocket::State<Config>, id: &str) -> Result<Json<DIDDoc>, DIDError> {
    // TODO: remove debug output
    println!("domainname {}", &config.domainname);
    println!("subpath {}", &config.subpath);
    println!("didstore {}", &config.didstore);
    println!(
        "did did:web:{}:{}/{}/did.json",
        &config.domainname, &config.subpath, id
    );

    compute_filename(&config.didstore, id)
        .map_err(|e| DIDError::NoFileName(e.to_string()))
        // debugging:
        // .map(|f| {
        //     f.to_str().map(|ff| println!("f {}", ff));
        //     f
        // })
        .and_then(|filename| {
            fs::read(filename).map_err(|e| {
                // TODO: return default value
                DIDError::NoFileRead(e.to_string())
            })
        })
        .and_then(|b| {
            // str::from_utf8(&b)
            //     .map(|s| s.to_string()) // only here &str needs to be converted to String to ensure that the result is available beyond the end of the function
            // okay, I managed to understand what's going on. I recevive an owned piece of data so
            // I need to ensure that I transform it into another owned piece of data. Yes, this is
            // possible with a function that transforms it accordingly and consumes the owned data
            String::from_utf8(b).map_err(|e| DIDError::ContenctConversion(e.to_string()))
        })
        .and_then(|ref s| {
            serde_json::from_str::<DIDDoc>(s)
                .map_err(|e| DIDError::ContenctConversion(e.to_string()))
        })
        // .and_then(|ref d: DIDDoc| {
        //     serde_json::to_string(d).map_err(|e| MyErrors::ConversionError(e.to_string()))1. [x] identinet: Work on did:web based file hosting service - get the service going with the integration of the SSI library
        // })
        .map_err(|e| {
            println!("get error: {}", e);
            e
        })
        .map(Json)
}

/// Creates a DID document .. I guess I somehow need to autodetermine the
/// correctness of the key, then validate the request and allow the post to happen. This is a
/// method for the administrator to create the DIDs that are allowed to exist on the server. It
/// requires authentication, e.g. via a signed request or just a JWT.
///
/// # TODO
///
/// * Implement authentication via some fitting method, JWT or actual signed requests via a private
///   key
/// * Create the DID document with the provided data
#[post("/v1/web/<id>/did.json", data = "<_doc>")]
fn create(
    config: &rocket::State<Config>,
    id: &str,
    _doc: Json<DIDDoc>,
) -> Result<String, DIDError> {
    // 1. let's retrieve the document via Post

    compute_filename(&config.didstore, id)
        .map_err(|e| DIDError::NoFileName(e.to_string()))
        .and_then(|filename| {
            if filename.exists() {
                Err(DIDError::DIDExists(format!(
                    "DID already exists: {}",
                    "todo did"
                )))
            } else {
                Ok(filename)
            }
        })
        .map(|_| format!("Did doc: did:web:identinet.io:vc/xx/did.json"))
}

/// Updates a DID Document if the identity is authorized to perform this operation.
///
/// * `presentation` - verifable presentation that holds the updated DID Document
///
/// # TODO
/// Automatically determine the appropriate DIDdoc derived from the ID .. if that makes sense .. or
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

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Config {
            domainname: get_env("DOMAINNAME", "localhost"),
            subpath: get_env("SUBPATH", ""),
            didstore: get_env(
                "DID_STORE",
                // by default store all files in $PWD/did_store/
                &std::env::current_dir()
                    .map(|val| val.join("did_store").to_str().unwrap_or(".").to_string())
                    .unwrap_or_else(|_| ".".to_string()),
            ),
        })
        .mount("/", routes![hello, get, create, update, delete])
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_compute_filename() {
        let id = "";
        let base_dir = ".";
        let result = compute_filename(base_dir, id);
        assert_eq!(
            result,
            Err("id is not a file"),
            "When <id> is empty, then an error is returned"
        );

        let id = "abc";
        let base_dir = ".";
        let result = compute_filename(base_dir, id);
        assert_eq!(
            result,
            Err("Path not absolute"),
            "When resulting path is not absolute, then an error is returned"
        );

        let id = "../abc";
        let base_dir = &format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/dids");
        let result = compute_filename(base_dir, id);
        assert_eq!(
            result,
            Err("id is not a file"),
            "When <id> contains additional characters that are not part of the filename, e.g. a relative path, then return an error"
        );

        let id = "abc";
        let base_dir = &format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/dids");
        let id_with_extension = "abc.json";
        let result = compute_filename(base_dir, id);
        match result {
            Ok(r) => assert_eq!(
                r,
                Path::new(base_dir).join(id_with_extension),
                "When <id> and <base_dir> can be combined to an absolute path, then succeed"
            ),
            Err(_) => {
                panic!("When <id> and <base_dir> can be combined to an absolute path, then succeed")
            }
        }
    }
}
