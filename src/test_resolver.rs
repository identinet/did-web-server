// SPDX-License-Identifier: Apache-2.0
// Copyright 2021, Spruce Systems, Inc. All rights reserved.
// Copyright 2022, identinet. All rights reserved.
// Source: https://github.com/spruceid/ssi/blob/main/did-web/src/lib.rs

///
/// Custom Resolvers that use the did:web Server endpoint methods either directly or through the
/// rocket client
///
use crate::store::DIDWebStore;
use crate::utils::path_to_string;
use async_trait::async_trait;
use either::{Either, Left, Right};
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use ssi::did::{DIDMethod, Document};
use ssi::did_resolve::{
    DIDResolver, DocumentMetadata, ResolutionInputMetadata, ResolutionMetadata,
    ERROR_METHOD_NOT_SUPPORTED, ERROR_NOT_FOUND, TYPE_DID_LD_JSON,
};
use std::path::PathBuf;

fn did_web_id(did: &str) -> Result<Either<PathBuf, PathBuf>, ResolutionMetadata> {
    let mut parts = did.split(':').peekable();
    let _domain_name = match (parts.next(), parts.next(), parts.next()) {
        (Some("did"), Some("web"), Some(domain_name)) => domain_name,
        _ => {
            return Err(ResolutionMetadata::from_error(ERROR_METHOD_NOT_SUPPORTED));
        }
    };
    let path = match parts.peek() {
        Some(_) => Right(parts.collect::<Vec<&str>>().join("/")),
        None => Left(".well-known".to_string()),
    };
    let to_did_path = |s| PathBuf::from(format!("{}/did.json", s));
    Ok(path.map_right(to_did_path).map_left(to_did_path))
}

/// did:web Method
///
/// [Specification](https://w3c-ccg.github.io/did-method-web/)
pub struct DIDWebTestResolver<'a> {
    pub store: Option<&'a Box<dyn DIDWebStore + Sync + Send>>,
    pub client: Option<&'a Client>,
}

impl Default for DIDWebTestResolver<'_> {
    fn default() -> Self {
        DIDWebTestResolver {
            store: None,
            client: None,
        }
    }
}

/// <https://w3c-ccg.github.io/did-method-web/#read-resolve>
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DIDResolver for DIDWebTestResolver<'_> {
    async fn resolve(
        &self,
        did: &str,
        input_metadata: &ResolutionInputMetadata,
    ) -> (
        ResolutionMetadata,
        Option<Document>,
        Option<DocumentMetadata>,
    ) {
        let (mut res_meta, doc_data, doc_meta_opt) =
            self.resolve_representation(did, input_metadata).await;
        let doc_opt = if doc_data.is_empty() {
            None
        } else {
            match serde_json::from_slice(&doc_data) {
                Ok(doc) => doc,
                Err(err) => {
                    return (
                        ResolutionMetadata::from_error(
                            &("JSON Error: ".to_string() + &err.to_string()),
                        ),
                        None,
                        None,
                    )
                }
            }
        };
        // https://www.w3.org/TR/did-core/#did-resolution-metadata
        // contentType - "MUST NOT be present if the resolve function was called"
        res_meta.content_type = None;
        (res_meta, doc_opt, doc_meta_opt)
    }

    async fn resolve_representation(
        &self,
        did: &str,
        _input_metadata: &ResolutionInputMetadata,
    ) -> (ResolutionMetadata, Vec<u8>, Option<DocumentMetadata>) {
        let id = match did_web_id(did) {
            Err(meta) => return (meta, Vec::new(), None),
            Ok(id) => id,
        };

        if let Some(store) = self.store {
            // ist die ID falsch?
            let get_from_store = |id: PathBuf| store.get(&id);
            let resp = id.either(get_from_store, get_from_store);

            match resp {
                Ok(doc) => {
                    let doc_string = serde_json::to_string(&doc).unwrap();
                    (
                        ResolutionMetadata {
                            error: None,
                            content_type: Some(TYPE_DID_LD_JSON.to_string()),
                            property_set: None,
                        },
                        doc_string.into_bytes(),
                        Some(DocumentMetadata::default()),
                    )
                }
                Err(_e) => (
                    ResolutionMetadata::from_error(ERROR_NOT_FOUND),
                    Vec::new(),
                    Some(DocumentMetadata::default()),
                ),
            }
        } else if let Some(client) = self.client {
            let id_to_url = |id: PathBuf| {
                let path = path_to_string(&id, "/");
                format!("/{}", path)
            };
            let url = id.map_right(id_to_url).map_left(id_to_url);
            let resp = match url {
                Right(url) => client.get(uri!(crate::get(id = PathBuf::from(url)))),
                Left(_url) => client.get(uri!(crate::get_wellknown())),
            }
            .dispatch()
            .await;

            if resp.status() == Status::Ok {
                let data = resp.into_bytes().await.unwrap();
                (
                    ResolutionMetadata {
                        error: None,
                        content_type: Some(TYPE_DID_LD_JSON.to_string()),
                        property_set: None,
                    },
                    data,
                    Some(DocumentMetadata::default()),
                )
            } else {
                (
                    ResolutionMetadata::from_error(ERROR_NOT_FOUND),
                    Vec::new(),
                    Some(DocumentMetadata::default()),
                )
            }
        } else {
            (
                ResolutionMetadata::from_error(ERROR_NOT_FOUND),
                Vec::new(),
                Some(DocumentMetadata::default()),
            )
        }
    }
}

impl DIDMethod for DIDWebTestResolver<'_> {
    fn name(&self) -> &'static str {
        "web"
    }

    fn to_resolver(&self) -> &dyn DIDResolver {
        self
    }
}
