// use serde::{Deserialize, Serialize};
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str;

#[macro_use]
extern crate rocket;

#[derive(Debug)]
enum MyErrors {
    ConversionError(String),
    FileError(String),
    FileNameError(String),
}

impl fmt::Display for MyErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyErrors::ConversionError(e) => write!(f, "{}", e),
            MyErrors::FileError(e) => write!(f, "{}", e),
            MyErrors::FileNameError(e) => write!(f, "{}", e),
        }
    }
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

fn compute_filename<'a>(base_dir: &str, id: &str) -> Result<PathBuf, &'a str> {
    Path::new(id)
        .file_name()
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
            let p = Path::new(base_dir)
                .join("dids")
                .join(id_file)
                .with_extension("json");
            if p.is_absolute() {
                Ok(p)
            } else {
                Err("Path not absolute")
            }
        })
}

// TODO: can we also represent sub directories somehow? <..id>?
// TODO: how about passing .. or some other weird construct, is it safe against that?
// TODO: implent tests
// Automatically determine the appropriate DIDdoc derived from the ID .. if that makes sense .. or
// no?
// Retrieve DID documents from the file system
#[get("/v1/web/<id>/did.json")]
fn get(id: &str) -> Option<Json<DIDDoc>> {
    // TODO: figure out where to get the runtime directory from
    compute_filename(env!("CARGO_MANIFEST_DIR"), id)
        .or_else(|e| Err(MyErrors::FileNameError(e.to_string())))
        // debugging:
        // .map(|f| {
        //     f.to_str().map(|ff| println!("f {}", ff));
        //     f
        // })
        .and_then(|filename| {
            fs::read(filename).or_else(|e| Err(MyErrors::FileError(e.to_string())))
        })
        .and_then(|b| {
            // str::from_utf8(&b)
            //     .map(|s| s.to_string()) // only here &str needs to be converted to String to ensure that the result is available beyond the end of the function
            // okay, I managed to understand what's going on. I recevive an owned piece of data so
            // I need to ensure that I transform it into another owned piece of data. Yes, this is
            // possible with a function that transforms it accordingly and consumes the owned data
            String::from_utf8(b).or_else(|e| Err(MyErrors::ConversionError(e.to_string())))
        })
        .and_then(|ref s| {
            serde_json::from_str::<DIDDoc>(s)
                .or_else(|e| Err(MyErrors::ConversionError(e.to_string())))
        })
        // .and_then(|ref d: DIDDoc| {
        //     serde_json::to_string(d).map_err(|e| MyErrors::ConversionError(e.to_string()))1. [x] identinet: Work on did:web based file hosting service - get the service going with the integration of the SSI library
        // })
        .map_err(|e| {
            println!("get error: {}", e);
            e
        })
        .map(Json)
        .ok()
}

// Post a DID document to the post endpoint.. I guess I somehow need to autodetermine the
// correctness of the key, then validate the request and allow the post to happen
#[post("/v1/web/<_id>/did.json", data = "<_doc>")]
fn create(_id: &str, _doc: Json<DIDDoc>) -> String {
    // 1. let's retrieve the document via Post
    format!("Did doc: did:web:identinet.io:vc/xx/did.json")
}

#[put("/v1/web/<id>/did.json")]
fn update(id: &str) -> String {
    format!("Did doc: did:web:identinet.io:vc/{}/did.json", id)
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
    rocket::build().mount("/", routes![hello, get, create, update, delete])
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_compute_filename() {
        let id = "abc";
        let base_dir = ".";
        let result = compute_filename(base_dir, id);
        assert_eq!(
            result,
            Err("Path not absolute"),
            "When path is not absolute, then an error is returned"
        );

        let id = "../abc";
        let base_dir = env!("CARGO_MANIFEST_DIR");
        let result = compute_filename(base_dir, id);
        assert_eq!(
            result,
            Err("id is not a file"),
            "When <id> contains additional characters that are not part of the filename, e.g. a relative path, then return an error"
        );

        let id = "abc";
        let base_dir = env!("CARGO_MANIFEST_DIR");
        let id_with_extension = "abc.json";
        let result = compute_filename(base_dir, id);
        match result {
            Ok(r) => assert_eq!(
                r,
                Path::new(base_dir).join("dids").join(id_with_extension),
                "When <id> and <base_dir> can be combined to an absolute path, then succeed"
            ),
            Err(_) => {
                panic!("When <id> and <base_dir> can be combined to an absolute path, then succeed")
            }
        }
    }
}
