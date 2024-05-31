use std::io::prelude::*;

use ssi::did::Document;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{error::DIDError, store::DIDWebStore};

#[derive(Debug)]
pub struct FileStore {
    directory: PathBuf,
}

impl FileStore {
    pub fn new(directory: String) -> Self {
        FileStore {
            directory: PathBuf::from(directory),
            // ..FileStore::default()
        }
    }
}

impl Default for FileStore {
    fn default() -> Self {
        Self {
            directory: PathBuf::from(
                // by default store all files in $PWD/did_store/
                &std::env::current_dir()
                    .map(|val| val.join("did_store").to_str().unwrap_or(".").to_string())
                    .unwrap_or_else(|_| ".".to_string()),
            ),
        }
    }
}

impl DIDWebStore for FileStore {
    // fn exists(&self, id: &Path) -> bool {
    //     match id2filename(&self.directory, id) {
    //         Ok(filename) => filename.exists(),
    //         Err(_) => false,
    //     }
    // }

    fn get(&self, id: &Path) -> Result<Document, DIDError> {
        id2filename(&self.directory, id)
            // .map(|f| {
            //     f.to_str().map(log("path"));
            //     f
            // })
            .map_err(|e| DIDError::DIDNotFound(e.to_string()))
            // TODO: use let doc = match Document::from_json(include_str!("../tests/did-example-foo.json")) {
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
            .and_then(|filename| {
                fs::read(filename).map_err(|e| DIDError::NoFileRead(e.to_string()))
            })
            .and_then(|b| {
                String::from_utf8(b).map_err(|e| DIDError::ContentConversion(e.to_string()))
            })
            .and_then(|ref s| {
                serde_json::from_str::<Document>(s)
                    .map_err(|e| DIDError::ContentConversion(e.to_string()))
            })
    }

    fn create(&self, id: &Path, doc: Document) -> Result<Document, DIDError> {
        let did = doc.id.to_string();
        self.store_diddoc(id, doc, |filename| {
            if filename.exists() {
                Err(DIDError::DIDExists(format!("DID already exists: {}", did)))
            } else {
                Ok(filename)
            }
        })
    }

    fn update(&self, id: &Path, doc: Document) -> Result<Document, DIDError> {
        self.store_diddoc(id, doc, |filename| Ok(filename))
    }

    fn remove(&self, id: &Path) -> Result<Document, DIDError> {
        let document = self.get(id)?; // WARNING: potential early return!
        id2filename(&self.directory, id)
            .map_err(|e| DIDError::NoFileName(e.to_string()))
            .and_then(|filename| {
                if filename.exists() {
                    Ok(filename)
                } else {
                    Err(DIDError::DIDNotFound("DID doesn't exist".to_string()))
                }
            })
            // Delete file that stores DID doc
            .and_then(|filename| {
                std::fs::remove_file(filename).map_err(|e| DIDError::NoFileWrite(e.to_string()))
            })
            .map(|_| document.to_owned())
    }
}

impl FileStore {
    /// Persisently stores DID Document
    ///
    /// - `config` - Global configuration.
    /// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
    /// - `doc` - DID Document.
    /// - `contraints_op` - Call contraints_op with the computed file name. contraints_op returns Ok if the update / creation can continue.
    /// - returns size of bytes written or an error
    fn store_diddoc<F: FnOnce(&PathBuf) -> Result<&PathBuf, DIDError>>(
        &self,
        id: &Path,
        doc: Document,
        contraints_op: F,
    ) -> Result<Document, DIDError> {
        id2filename(&self.directory, id)
            .map_err(|e| DIDError::NoFileName(e.to_string()))
            .and_then(|filename| match contraints_op(&filename) {
                Ok(_) => Ok(filename),
                Err(e) => Err(e),
            })
            // Store DID doc in file
            // Store DID document in file
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
                            .map(|_| doc)
                    })
            })
    }
}

/// Computes the absolute path to a file with json extension in a base
/// direcotory and an ID.
///
/// @param base_dir - Base directory that `id` will be appended to
/// @param id - Path to store DID document at
fn id2filename<'a>(base_dir: &Path, id: &Path) -> Result<PathBuf, &'a str> {
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
            filename.to_str().and_then(|_f| {
                if _f == "did.json" {
                    Some(id)
                    // .parent()
                } else {
                    None
                }
            })
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
    use crate::store::file::*;

    #[test]
    fn test_get_filename_from_id() {
        let id = PathBuf::from("");
        let base_dir = PathBuf::from(".");
        let result = id2filename(&base_dir, &id);
        assert_eq!(
            result,
            Err("id not found, path doesn't end in /did.json"),
            "When <id> is empty, then an error is returned"
        );

        let id = PathBuf::from("abc/did.json");
        let base_dir = PathBuf::from(".");
        let result = id2filename(&base_dir, &id);
        assert_eq!(
            result,
            Err("Path not absolute"),
            "When resulting path is not absolute, then an error is returned"
        );

        // let id = PathBuf::from("../abc/did.json");
        // let base_dir = PathBuf::from(&format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/dids"));
        // let result = id2filename(&base_dir, &id);
        // assert_eq!(
        //     result,
        //     Err("id not found, path doesn't end in /did.json"),
        //     "When <id> contains additional characters that are not part of the filename, e.g. a relative path, then return an error"
        // );

        let id = PathBuf::from("abc/did.json");
        let base_dir = PathBuf::from(&format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/dids"));
        let id_with_extension = "abc/did.json";
        let result = id2filename(&base_dir, &id);
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
        let id_with_extension = ".well-known/did.json";
        let result = id2filename(&base_dir, &id);
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
