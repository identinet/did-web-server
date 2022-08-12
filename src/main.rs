// use rocket::tokio::fs::File;
// use serde::{Deserialize, Serialize};
use rocket::serde::{json::Json, Deserialize, Serialize};
// use serde::Deserialize;
// use rocket::serde::Serialize;
// use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str;

#[macro_use]
extern crate rocket;

#[derive(Debug)]
enum MyErrors<String> {
    FileError(String),
    ConversionError(String),
}

impl fmt::Display for MyErrors<String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyErrors::FileError(e) => write!(f, "{}", e),
            MyErrors::ConversionError(e) => write!(f, "{}", e),
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
pub struct DIDDoc {
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

// TODO: can we also represent sub directories somehow? <..id>?
// Retrieve DID documents from the file system
#[get("/v1/web/<id>/did.json")]
fn get(id: &str) -> Option<Json<DIDDoc>> {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "dids");
    let filename = Path::new(upload_dir).join(id);
    // File::open(&filename).await.ok();
    // der Fehler kommt von dem Problem, dass fs::read einen IO-Error erzeugt, von dem dann
    // erwartet wird, dass er weiter ausgeführt wird .. was nicht der Fall ist
    // fn x(ok: bool) -> Result<i32, dyn Error> {
    //     if ok {
    //         Ok(3)
    //     } else {
    //         Err(33)
    //     }
    // }
    // x(true)
    //     .and_then(|| fs::read(filename))

    // let filename2 = Path::new(upload_dir).join(id);
    // let x = fs::read(filename2).map_err(|e| MyErrors::FileError(e.to_string()));
    // let _z = match x {
    //     Ok(ref y) => {
    //         let aa = str::from_utf8(y).map_err(|e| MyErrors::ConversionError(e.to_string()));
    //         aa
    //     }
    //     Err(e) => Err(e),
    // };
    // None

    // // Das Problem scheint teilweise in den Closures zu liegen, die nicht ordentlich funktionieren.
    // // Eine Lösung ist es auf die Fehlermeldung zu verzichten, denn diese erzeugt
    // fn toStaticString<'a>(s: &'a String) -> &'a String {
    //     return s;
    // }

    // const fn from(b: Vec<u8>) -> Result<&'static str, String> {
    //     let y = str::from_utf8(&b)
    //         .map(|r| r.to_string())
    //         .map_err(|e| MyErrors::ConversionError(e.to_string()));
    //     // y
    //     Ok("moin")
    // }

    fs::read(filename)
        .map_err(|e| MyErrors::FileError(e.to_string()))
        .and_then(|b| {
            str::from_utf8(&b)
                .map(|s| s.to_string()) // only here &str needs to be converted to String to ensure that the result is available beyond the end of the function
                .map_err(|e| MyErrors::ConversionError(e.to_string()))
        })
        .and_then(|ref s| {
            serde_json::from_str::<DIDDoc>(s).map_err(|e| MyErrors::ConversionError(e.to_string()))
        })
        // .and_then(|ref d: DIDDoc| {
        //     serde_json::to_string(d).map_err(|e| MyErrors::ConversionError(e.to_string()))
        // })
        .map_err(|e| {
            println!("get error: {}", e);
            e
        })
        .map(Json)
        .ok()
}

#[post("/v1/web")]
fn create() -> String {
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
