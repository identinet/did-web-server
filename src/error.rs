use std::fmt;

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
    #[response(status = 399)] // TODO: return a default value instead of an error code, maybe
    NoFileRead(String),
    #[response(status = 500)] // InternalServerError
    NoFileWrite(String),
    #[response(status = 400)] // BadRequest
    NoFileName(String),
    #[response(status = 400)] // BadRequest
    DIDExists(String),
    #[response(status = 400)] // BadRequest
    IllegalCharacter(String),
    #[response(status = 400)] // BadRequest
    DIDMismatch(String),
    #[response(status = 404)] // NotFound
    DIDNotFound(String),
    #[response(status = 400)] // Bad Request
    DIDPortNotAllowed(String),
    #[response(status = 400)] // Bad Request
    PresentationInvalid(String),
    #[response(status = 500)] // InternalServerError
    UnknownBackend(String),
}

impl std::error::Error for DIDError {}

impl fmt::Display for DIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DIDError::ContentConversion(e) => write!(f, "{}", e),
            DIDError::NoFileRead(e) => write!(f, "{}", e),
            DIDError::NoFileWrite(e) => write!(f, "{}", e),
            DIDError::NoFileName(e) => write!(f, "{}", e),
            DIDError::DIDExists(e) => write!(f, "{}", e),
            DIDError::IllegalCharacter(e) => write!(f, "{}", e),
            DIDError::DIDMismatch(e) => write!(f, "{}", e),
            DIDError::DIDNotFound(e) => write!(f, "{}", e),
            DIDError::DIDPortNotAllowed(e) => write!(f, "{}", e),
            DIDError::PresentationInvalid(e) => write!(f, "{}", e),
            DIDError::UnknownBackend(e) => write!(f, "{}", e),
        }
    }
}
