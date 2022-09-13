use std::io::prelude::*;
use std::path::{Path, PathBuf};

use ssi::did::Document;

use crate::{config::Config, error::DIDError};

/// Creates a DID Document
///
/// - `config` - Global configuration.
/// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
/// - `doc` - DID Document.
/// - returns The stored document or an error
pub fn create_diddoc(
    config: &rocket::State<Config>,
    id: PathBuf,
    doc: &Document,
) -> Result<usize, DIDError> {
    store_diddoc(config, id, doc, |filename| {
        if filename.exists() {
            // Err(DIDError::DIDExists(format!(
            //     "DID already exists: {}",
            //     computed_did
            // )))
            Err(DIDError::DIDExists("DID already exists".to_string()))
        } else {
            Ok(filename)
        }
    })
}

/// Updates a DID Document
///
/// - `config` - Global configuration.
/// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
/// - `doc` - DID Document.
/// - returns The stored document or an error
pub fn update_diddoc(
    config: &rocket::State<Config>,
    id: PathBuf,
    doc: &Document,
) -> Result<usize, DIDError> {
    store_diddoc(config, id, doc, |filename| Ok(filename))
}

/// Persisently stores DID Document
///
/// - `config` - Global configuration.
/// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
/// - `doc` - DID Document.
/// - `contraints_op` - Call contraints_op with the computed file name. contraints_op returns Ok if the update / creation can continue.
/// - returns size of bytes written or an error
fn store_diddoc<F: FnOnce(&PathBuf) -> Result<&PathBuf, DIDError>>(
    config: &rocket::State<Config>,
    id: PathBuf,
    doc: &Document,
    contraints_op: F,
) -> Result<usize, DIDError> {
    get_filename_from_id(&config.didstore, &id)
        .map_err(|e| DIDError::NoFileName(e.to_string()))
        .and_then(|filename| match contraints_op(&filename) {
            Ok(_) => Ok(filename),
            Err(e) => Err(e),
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
                    serde_json::to_string(&doc)
                        .map_err(|e| DIDError::ContentConversion(e.to_string()))
                        .and_then(|s| {
                            f.write(s.as_bytes())
                                .map_err(|e| DIDError::NoFileWrite(e.to_string()))
                        })
                })
        })
}

/// Computes the absolute path to a file with json extension in a base
/// direcotory and an ID.
pub fn get_filename_from_id<'a>(base_dir: &Path, id: &Path) -> Result<PathBuf, &'a str> {
    // Path::new(id)
    //     .file_name()
    //     // DEBUG
    //     // .map(|x| {
    //     //     let y = x.to_str();
    //     //     match y {
    //     //         Some(s) => println!("res {}, {}", s, id),
    //     //         None => println!("nothing"),
    //     //     };
    //     //     x
    //     // })
    //     //  bail out if id contains more information than than the plain file name
    //     .and_then(|id_file| if id_file == id { Some(id_file) } else { None })
    //     .ok_or("id is not a file") // TODO: place error messages in a constant somewhere
    //     .and_then(|id_file| {
    //         let p = Path::new(base_dir).join(id_file).with_extension("json");
    //         if p.is_absolute() {
    //             Ok(p)
    //         } else {
    //             Err("Path not absolute")
    //         }
    //     })
    id.file_name()
        // .map(|x| {
        //     match x.to_str() {
        //         Some(s) => println!("res {}, {}", s, "id"),
        //         None => println!("nothing"),
        //     };
        //     x
        // })
        .and_then(|filename| {
            filename
                .to_str()
                .and_then(|_f| if _f == "did.json" { id.parent() } else { None })
        })
        .and_then(|id_file| {
            if id_file.is_absolute() {
                id_file.strip_prefix("/").ok() // when id is an absolute path, joining it with
                                               // base_dir will overwrite base_dir's value.
                                               // Therefore, ensure that path is releative
            } else {
                Some(id_file)
            }
        })
        // .map(|x| {
        //     match x.to_str() {
        //         Some(s) => println!("parent path res {}, {}", s, "id"),
        //         None => println!("partent path nothing"),
        //     };
        //     x
        // })
        .ok_or("id not found, path doesn't end in /did.json")
        .and_then(|id_file| {
            let p = base_dir.join(id_file).with_extension("json");
            if p.is_absolute() {
                Ok(p)
            } else {
                Err("Path not absolute")
            }
        })
}

#[cfg(test)]
mod test {
    use crate::store::*;

    #[test]
    fn test_get_filename_from_id() {
        let id = PathBuf::from("");
        let base_dir = PathBuf::from(".");
        let result = get_filename_from_id(&base_dir, &id);
        assert_eq!(
            result,
            Err("id not found, path doesn't end in /did.json"),
            "When <id> is empty, then an error is returned"
        );

        let id = PathBuf::from("abc/did.json");
        let base_dir = PathBuf::from(".");
        let result = get_filename_from_id(&base_dir, &id);
        assert_eq!(
            result,
            Err("Path not absolute"),
            "When resulting path is not absolute, then an error is returned"
        );

        // let id = PathBuf::from("../abc/did.json");
        // let base_dir = PathBuf::from(&format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/dids"));
        // let result = get_filename_from_id(&base_dir, &id);
        // assert_eq!(
        //     result,
        //     Err("id not found, path doesn't end in /did.json"),
        //     "When <id> contains additional characters that are not part of the filename, e.g. a relative path, then return an error"
        // );

        let id = PathBuf::from("abc/did.json");
        let base_dir = PathBuf::from(&format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/dids"));
        let id_with_extension = "abc.json";
        let result = get_filename_from_id(&base_dir, &id);
        match result {
            Ok(r) => assert_eq!(
                r,
                base_dir.join(id_with_extension),
                "When <id> and <base_dir> can be combined to an absolute path, then succeed"
            ),
            Err(_) => {
                panic!("When <id> and <base_dir> can be combined to an absolute path, then succeed")
            }
        }

        let id = PathBuf::from(".well-known/did.json");
        let base_dir = PathBuf::from(&format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/dids"));
        let id_with_extension = ".well-known.json";
        let result = get_filename_from_id(&base_dir, &id);
        match result {
            Ok(r) => assert_eq!(
                r,
                base_dir.join(id_with_extension),
                "When <id> is .well-known/did.json and <base_dir> can be combined to an absolute path, then succeed"
            ),
            Err(_) => {
                panic!("When <id> is .well-known/did.json and <base_dir> can be combined to an absolute path, then succeed")
            }
        }
    }
}
