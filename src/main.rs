// use serde::{Deserialize, Serialize};
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str;

#[macro_use]
extern crate rocket;

#[derive(Debug, Responder)]
enum DocumentErrors {
    #[response(status = 500)] // InternalServerError
    ConversionError(String),
    #[response(status = 399)] // TODO: return a default value instead of an error code
    FileError(String),
    #[response(status = 400)] // BadRequest
    FileNameError(String),
}

impl fmt::Display for DocumentErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentErrors::ConversionError(e) => write!(f, "{}", e),
            DocumentErrors::FileError(e) => write!(f, "{}", e),
            DocumentErrors::FileNameError(e) => write!(f, "{}", e),
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

#[derive(Responder)]
enum DIDResult<T, E> {
    #[response(status = 200)]
    Ok(T),
    Err(E),
}

// TODO: implent tests

/// Retrieve DID documents from the file system.
///
/// * `id` - requested id, e.g. `alice`
///
/// # TODO
///
/// * implement subdirectories
#[get("/<id>/did.json")]
fn get(config: &rocket::State<Config>, id: &str) -> DIDResult<Json<DIDDoc>, DocumentErrors> {
    println!("domainname {}", &config.domainname);
    println!("subpath {}", &config.subpath);
    println!("didstore {}", &config.didstore);
    println!(
        "did did:web:{}:{}/{}/did.json",
        &config.domainname, &config.subpath, id
    );

    // std::env::var("PWD"); // TODO: retrieve directory where to store the DIDs dynamically; default
    //                       // is the current directory's `dids` subdirectory
    let result = compute_filename(&config.didstore, id)
        .or_else(|e| Err(DocumentErrors::FileNameError(e.to_string())))
        // debugging:
        // .map(|f| {
        //     f.to_str().map(|ff| println!("f {}", ff));
        //     f
        // })
        .and_then(|filename| {
            fs::read(filename).or_else(|e| {
                // TODO: return default value
                Err(DocumentErrors::FileError(e.to_string()))
            })
        })
        .and_then(|b| {
            // str::from_utf8(&b)
            //     .map(|s| s.to_string()) // only here &str needs to be converted to String to ensure that the result is available beyond the end of the function
            // okay, I managed to understand what's going on. I recevive an owned piece of data so
            // I need to ensure that I transform it into another owned piece of data. Yes, this is
            // possible with a function that transforms it accordingly and consumes the owned data
            String::from_utf8(b).or_else(|e| Err(DocumentErrors::ConversionError(e.to_string())))
        })
        .and_then(|ref s| {
            serde_json::from_str::<DIDDoc>(s)
                .or_else(|e| Err(DocumentErrors::ConversionError(e.to_string())))
        })
        // .and_then(|ref d: DIDDoc| {
        //     serde_json::to_string(d).map_err(|e| MyErrors::ConversionError(e.to_string()))1. [x] identinet: Work on did:web based file hosting service - get the service going with the integration of the SSI library
        // })
        .map_err(|e| {
            println!("get error: {}", e);
            e
        })
        .map(Json);
    match result {
        Ok(r) => DIDResult::Ok(r),
        Err(e) => DIDResult::Err(e),
    }
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
#[post("/v1/web/<_id>/did.json", data = "<_doc>")]
fn create(config: &rocket::State<Config>, _id: &str, _doc: Json<DIDDoc>) -> String {
    // 1. let's retrieve the document via Post

    // let result = compute_filename(&config.didstore, id)
    //     .or_else(|e| Err(DocumentErrors::FileNameError(e.to_string())));

    format!("Did doc: did:web:identinet.io:vc/xx/did.json")
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
                    .unwrap_or(".".to_string()),
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
