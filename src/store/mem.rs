use dashmap::DashMap;
use ssi::did::Document;
use std::path::Path;

use crate::error::DIDError;

use super::DIDWebStore;
use crate::utils::path_to_string;

#[derive(Debug)]
pub struct MemStore {
    // store: HashMap<String, Document>,
    store: DashMap<String, Document>,
}

impl MemStore {
    pub fn new() -> Self {
        MemStore {
            ..MemStore::default()
        }
    }
}

impl Default for MemStore {
    fn default() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}

impl MemStore {
    fn id_to_string(id: &Path) -> String {
        path_to_string(id, ":")
    }
}

impl DIDWebStore for MemStore {
    // fn exists(&self, id: &Path) -> bool {
    //     self.store.get(&MemStore::id_to_string(id)).is_some()
    // }

    fn get(&self, id: &Path) -> Result<Document, DIDError> {
        self.store
            .get(&MemStore::id_to_string(id))
            .map(|doc| doc.to_owned())
            .ok_or_else(|| DIDError::DIDNotFound("DID not found".to_string()))
    }

    fn create(&self, id: &Path, doc: Document) -> Result<Document, DIDError> {
        if self.store.get(&MemStore::id_to_string(id)).is_some() {
            Err(DIDError::DIDExists(format!(
                "DID already exists: {}",
                doc.id
            )))
        } else {
            self.store.insert(MemStore::id_to_string(id), doc);
            self.get(id)
        }
    }

    fn update(&self, id: &Path, doc: Document) -> Result<Document, DIDError> {
        if self.store.get(&MemStore::id_to_string(id)).is_none() {
            Err(DIDError::DIDNotFound("DID not found".to_string()))
        } else {
            self.store
                .insert(MemStore::id_to_string(id), doc)
                .ok_or_else(
                    // TODO: this should never be reached
                    || DIDError::DIDNotFound("DID not found".to_string()),
                )
        }
    }

    fn remove(&self, id: &Path) -> Result<Document, DIDError> {
        self.store
            .remove(&MemStore::id_to_string(id))
            .map(|(_, v)| v)
            .ok_or_else(|| DIDError::DIDNotFound("DID not found".to_string()))
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use ssi::did::Context;

    use crate::store::mem::*;

    #[test]
    fn test_id_not_in_store() {
        let store = MemStore::default();
        let id = PathBuf::from("an/id");
        let result = store.get(&id);
        assert!(
            result.is_err(),
            "When <id> is not in store, then an error is returned"
        );
    }

    #[test]
    fn test_add_id_to_store() {
        let store = MemStore::default();
        let id = PathBuf::from("an/id");
        let result = store.create(
            &id,
            Document {
                context: ssi::did::Contexts::One(Context::URI(
                    iref::IriRefBuf::new("https://example.com/my/context").unwrap(),
                )),
                id: "did:my:did".to_string(),
                also_known_as: None,
                controller: None,
                verification_method: None,
                authentication: None,
                assertion_method: None,
                key_agreement: None,
                capability_invocation: None,
                capability_delegation: None,
                public_key: None,
                service: None,
                proof: None,
                property_set: None,
            },
        );
        assert!(
            result.is_ok(),
            "When an <id> document is put in the store and <id> isn't present in store, then the document is stored at id"
        );

        let result = store.get(&id);
        assert!(
            result.is_ok(),
            "When document is present at <id> and <id> is accessed, then the document is retrieved"
        );

        let result = store.create(
            &id,
            Document {
                context: ssi::did::Contexts::One(Context::URI(
                    iref::IriRefBuf::new("https://example.com/my/context").unwrap(),
                )),
                id: "did:my:did".to_string(),
                also_known_as: None,
                controller: None,
                verification_method: None,
                authentication: None,
                assertion_method: None,
                key_agreement: None,
                capability_invocation: None,
                capability_delegation: None,
                public_key: None,
                service: None,
                proof: None,
                property_set: None,
            },
        );
        assert!(
            result.is_err(),
            "When document is present at <id> and <id> is tried to be created again, then an error is returned"
        );
    }

    #[test]
    fn test_update_id_in_store() {
        let store = MemStore::default();
        let id = PathBuf::from("an/id");
        let result = store.update(
            &id,
            Document {
                context: ssi::did::Contexts::One(Context::URI(
                    iref::IriRefBuf::new("https://example.com/my/context").unwrap(),
                )),
                id: "did:my:did".to_string(),
                also_known_as: None,
                controller: None,
                verification_method: None,
                authentication: None,
                assertion_method: None,
                key_agreement: None,
                capability_invocation: None,
                capability_delegation: None,
                public_key: None,
                service: None,
                proof: None,
                property_set: None,
            },
        );
        assert!(
            result.is_err(),
            "When <id> isn't present in store and an update is attempted, then an error is returned"
        );

        let result = store.create(
            &id,
            Document {
                context: ssi::did::Contexts::One(Context::URI(
                    iref::IriRefBuf::new("https://example.com/my/context").unwrap(),
                )),
                id: "did:my:did".to_string(),
                also_known_as: None,
                controller: None,
                verification_method: None,
                authentication: None,
                assertion_method: None,
                key_agreement: None,
                capability_invocation: None,
                capability_delegation: None,
                public_key: None,
                service: None,
                proof: None,
                property_set: None,
            },
        );
        assert!(
            result.is_ok(),
            "When an <id> document is put in the store and <id> isn't present in store, then the document is stored at id"
        );

        let result = store.update(
            &id,
            Document {
                context: ssi::did::Contexts::One(Context::URI(
                    iref::IriRefBuf::new("https://example.com/my/context").unwrap(),
                )),
                id: "did:my:did".to_string(),
                also_known_as: None,
                controller: None,
                verification_method: None,
                authentication: None,
                assertion_method: None,
                key_agreement: None,
                capability_invocation: None,
                capability_delegation: None,
                public_key: None,
                service: None,
                proof: None,
                property_set: None,
            },
        );
        assert!(
            result.is_ok(),
            "When <id> is present in store and an update is attempted, then the update succeeds"
        );
    }

    #[test]
    fn test_remove_id_from_store() {
        let store = MemStore::default();
        let id = PathBuf::from("an/id");
        let result = store.remove(&id);
        assert!(
            result.is_err(),
            "When <id> isn't present in store and a remove is attempted, then an error is returned"
        );

        // TOOD: continue here to fix the result type
        let result = store.create(
            &id,
            Document {
                context: ssi::did::Contexts::One(Context::URI(
                    iref::IriRefBuf::new("https://example.com/my/context").unwrap(),
                )),
                id: "did:my:did".to_string(),
                also_known_as: None,
                controller: None,
                verification_method: None,
                authentication: None,
                assertion_method: None,
                key_agreement: None,
                capability_invocation: None,
                capability_delegation: None,
                public_key: None,
                service: None,
                proof: None,
                property_set: None,
            },
        );
        assert!(
            result.is_ok(),
            "When an <id> document is put in the store and <id> isn't present in store, then the document is stored at id"
        );

        let result = store.remove(&id);
        assert!(
            result.is_ok(),
            "When <id> is present in store and a remove is attempted, then the operation succeeds"
        );

        let result = store.get(&id);
        assert!(
            result.is_err(),
            "When <id> is not in store, then an error is returned"
        );
    }
}
