use std::path::Path;
use std::{cmp::Ordering, fmt};

use chrono::{DateTime, Utc};
use ssi::did::{Document, VerificationRelationship};
use ssi::ldp::Proof;
use ssi::one_or_many::OneOrMany;
use ssi::vc::{Credential, CredentialOrJWT, CredentialSubject, Presentation, VCDateTime};

use crate::error::DIDError;

/// Join a path into a String with separator.
pub fn path_to_string(path: &Path, sep: &str) -> String {
    path.iter()
        .filter_map(|s| s.to_str())
        .collect::<Vec<&str>>()
        .join(sep)
}

/// Retrieve value from an environment variable or if unst return the default value.
pub fn get_env(varname: &str, default: &str) -> String {
    match std::env::var(varname) {
        Ok(value) => value,
        Err(_) => default.to_string(),
    }
}

/// Takes a message and returns a function that takes a variable, prints it with the message and
/// returns the argument.
///
/// # Example:
///
/// ```rust
/// Err("an error")
///   .map_err(log("state of error is"))
///   .map(|x|
///     // do something else
///     x
///   )
/// ```
///
/// Prints: `state of the error is: an error`
pub fn log<T: fmt::Display>(msg: &'static str) -> impl Fn(T) -> T {
    move |o| {
        println!("{}: {}", msg, o);
        o
    }
}

/// Extract DID document from presentation and ensure that it matches a certain DID
pub fn get_did_doc_from_presentation(
    presentation: &'_ Presentation,
    did: String,
) -> Result<(&'_ Credential, CredentialSubject), DIDError> {
    presentation
        .verifiable_credential
        .as_ref()
        .and_then(|vcs| {
            vcs.into_iter()
                .map(|credential| match credential {
                    CredentialOrJWT::Credential(credential) => {
                        credential.credential_subject.clone().into_iter().fold(
                            None,
                            |acc, credential_subject| {
                                let id_equals_proof_parameter_did =
                                    credential_subject.id.as_ref().and_then(|id| {
                                        println!("credential subject: {:?}", id.to_string());
                                        if id.to_string() == did {
                                            Some(true)
                                        } else {
                                            None
                                        }
                                    });
                                if acc.is_none() && id_equals_proof_parameter_did.is_some() {
                                    println!("credential has been issued for DID {}", did);
                                    // TODO: ensure that document is a DID Doc
                                    Some((credential, credential_subject))
                                } else {
                                    acc
                                }
                            },
                        )
                    }
                    CredentialOrJWT::JWT(_) => {
                        println!("credential jwt");
                        // ignore JWT credentials
                        None
                    }
                })
                .fold(
                    None,
                    |acc, credential| if acc.is_none() { credential } else { acc },
                )
        })
        .ok_or_else(|| DIDError::DIDDocMissing("No valid DID Doc credential found".to_string()))
}

/// Ensures that a date is in the correct order to a reference date, e.g. to ensure that the date
/// is not in the future or past.
pub fn compare_date(
    date: &Option<VCDateTime>,
    ordering: Ordering,
    reference: DateTime<Utc>,
) -> Option<Ordering> {
    match date {
        Some(issuance_date) => {
            let issuance_date = issuance_date.clone();
            match DateTime::parse_from_rfc3339(&String::from(issuance_date)) {
                Ok(issuance_date) => {
                    if issuance_date
                        .partial_cmp(&reference)
                        .and_then(|v| if v == ordering { Some(v) } else { None })
                        .is_some()
                    {
                        Some(ordering)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// ensure that at least one of the verification methods from the did document is
/// used in the proofs
///
/// * The bool return value can be ignored, it's always `true` if a matching proof was found
pub fn ensure_proof_matches_verification_method(
    did_doc: &Document,
    verification_relationship: VerificationRelationship,
    proofs: &Option<OneOrMany<Proof>>,
) -> Result<bool, DIDError> {
    // extract the supported verification methods from did document
    let verification_methods_in_did_doc: Vec<String> = did_doc
        .get_verification_method_ids(verification_relationship)
        .unwrap_or_else(|_| Vec::new());
    proofs
        .as_ref()
        .ok_or_else(|| {
            DIDError::PresentationInvalid("Presentation invalid, no proof found".to_string())
        })
        .and_then(|proofs| {
            if proofs.any(|proof| {
                proof
                    .verification_method
                    .as_ref()
                    .and_then(|verification_method| {
                        if verification_methods_in_did_doc.contains(verification_method) {
                            println!("proof found {}", verification_method);
                            Some(verification_method)
                        } else {
                            println!("proof not found");
                            None
                        }
                    })
                    .is_some()
            }) {
                Ok(true)
            } else {
                Err(DIDError::PresentationInvalid(
                    "Presentation invalid, no proof has been signed by expected did".to_string(),
                ))
            }
        })
}
