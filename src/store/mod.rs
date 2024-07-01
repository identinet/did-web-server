// SPDX-License-Identifier: AGPL-3.0

pub mod file;
pub mod mem;

use ssi::did::Document;
use std::path::Path;

use crate::error::DIDError;

pub trait DIDWebStore {
    // /// Tests existence of DID in store.
    // ///
    // /// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
    // fn exists(&self, id: &Path) -> bool;

    /// Get DID from store. The operation fails if the DID doesn't exist.
    ///
    /// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
    fn get(&self, id: &Path) -> Result<Document, DIDError>;

    /// Create DID in store. The operation fails if the DID already exists.
    ///
    /// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
    /// - `doc` - DID Document.
    ///
    /// @returns The new version of the DID Document
    fn create(&self, id: &Path, doc: Document) -> Result<Document, DIDError>;

    /// Update DID in store. The operation fails if the DID doesn't exist.
    ///
    /// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
    /// - `doc` - DID Document.
    ///
    /// @returns The old version of the DID Document
    fn update(&self, id: &Path, doc: Document) -> Result<Document, DIDError>;

    /// Remove DID from store. The operation fails if the DID doesn't exist.
    ///
    /// - `id` - id part of the did:web method as specified in https://w3c-ccg.github.io/did-method-web/
    ///
    /// @returns The old version of the DID Document
    fn remove(&self, id: &Path) -> Result<Document, DIDError>;
}
