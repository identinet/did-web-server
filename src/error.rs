// SPDX-License-Identifier: AGPL-3.0

use rocket::response::Responder;

/// Custom response status
#[derive(Responder)]
pub enum CustomStatus<T> {
    #[response(status = 201)] // Created
    Created(T),
}

// TODO: change String to &'static str to avoid reallocation of the error on the heap
#[derive(Debug, Responder)]
pub enum DIDError {
    #[response(status = 500)] // InternalServerError
    ContentConversion(String),
    #[response(status = 500)] // TODO: return a default value instead of an error code, maybe
    NoFileRead(String),
    #[response(status = 500)] // InternalServerError
    NoFileWrite(String),
    #[response(status = 400)] // BadRequest
    NoFileName(String),
    #[response(status = 409)] // Conflict
    DIDExists(String),
    #[response(status = 400)] // BadRequest
    IllegalCharacter(String),
    #[response(status = 400)] // BadRequest
    DIDDocMissing(String),
    #[response(status = 400)] // BadRequest
    DIDMismatch(String),
    #[response(status = 404)] // NotFound
    DIDNotFound(String),
    #[response(status = 400)] // Bad Request
    DIDPortNotAllowed(String),
    #[response(status = 401)] // Unauthorized
    PresentationInvalid(String),
    #[response(status = 500)] // InternalServerError
    UnknownBackend(String),
    OwnerMissing(String),
}

impl std::error::Error for DIDError {}

impl std::fmt::Display for DIDError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DIDError::ContentConversion(e) => write!(fmt, "Error {}.", e),
            DIDError::NoFileRead(e) => write!(fmt, "Error {}.", e),
            DIDError::NoFileWrite(e) => write!(fmt, "Error {}.", e),
            DIDError::NoFileName(e) => write!(fmt, "Error {}.", e),
            DIDError::DIDExists(e) => write!(fmt, "Error {}.", e),
            DIDError::IllegalCharacter(e) => write!(fmt, "Error {}.", e),
            DIDError::DIDMismatch(e) => write!(fmt, "Error {}.", e),
            DIDError::DIDDocMissing(e) => write!(fmt, "Error {}.", e),
            DIDError::DIDNotFound(e) => write!(fmt, "Error {}.", e),
            DIDError::DIDPortNotAllowed(e) => write!(fmt, "Error {}.", e),
            DIDError::PresentationInvalid(e) => write!(fmt, "Error {}.", e),
            DIDError::UnknownBackend(e) => write!(fmt, "Error {}.", e),
            DIDError::OwnerMissing(e) => write!(fmt, "Error {}.", e),
        }
    }
}
