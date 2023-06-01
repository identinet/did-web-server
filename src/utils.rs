use std::path::Path;
use std::{cmp::Ordering, fmt};

use chrono::{DateTime, Utc};
use rocket::serde::json::Json;
use ssi::did::VerificationRelationship;
use ssi::did_resolve::SeriesResolver;
use ssi::vc::{
    Credential, CredentialOrJWT, CredentialSubject, LinkedDataProofOptions, Presentation,
    VCDateTime, VerificationResult,
};
use ssi_dids::did_resolve::get_verification_methods_for_all;
use ssi_json_ld::ContextLoader;

use crate::config::Config;
use crate::did::ProofParameters;
use crate::error::DIDError;
#[cfg(test)]
use crate::test_resolver::DIDWebTestResolver;

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
) -> Result<(Credential, CredentialSubject), DIDError> {
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
                                        // println!("credential subject: {:?}", id.to_string());
                                        if id.to_string() == did {
                                            Some(true)
                                        } else {
                                            None
                                        }
                                    });
                                if acc.is_none() && id_equals_proof_parameter_did.is_some() {
                                    // println!("credential has been issued for DID {}", did);
                                    // TODO: ensure that document is a DID Doc
                                    Some((credential.clone(), credential_subject))
                                } else {
                                    acc
                                }
                            },
                        )
                    }
                    CredentialOrJWT::JWT(_) => {
                        // println!("credential jwt");
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

/// verify_issuer ensures that at least one of the verification methods from the issuer's did document is
/// used in the proofs.
///
/// # Arguments
///
/// * `config` - Server configuration
/// * `issuer_did` - DID of the isuser
/// * `verification_relationship` - The verification relationship that's expected to be used
/// * `presentation` - presentation to be inspected
/// * returns the true if a mathing relationship was found otherwise an error
// WARNING: do we need this?
pub async fn verify_issuer(
    config: &rocket::State<Config>,
    issuer_did: &str,
    verification_relationship: VerificationRelationship,
    presentation: &Presentation,
) -> Result<bool, DIDError> {
    // Retrieve all verification methods for the given DID

    // INFO: Not sure how to replace this with a simple function call that integrates the test resolver on demand!
    let mut resolvers: Vec<&dyn ssi::did_resolve::DIDResolver> = vec![];
    #[cfg(test)]
    let test_resolver = DIDWebTestResolver {
        store: Some(&config.store),
        ..DIDWebTestResolver::default()
    };
    #[cfg(test)]
    resolvers.push(&test_resolver);
    let default_resolver = config.reslover_options.get_resolver();
    resolvers.push(&default_resolver);
    let resolver = SeriesResolver { resolvers };

    let vmms: Vec<String> =
        get_verification_methods_for_all(&[issuer_did], verification_relationship, &resolver)
            .await
            .map(|map| map.into_keys().collect())
            .map_err(|_| DIDError::DIDNotFound("Couldn't fully resolve DID".to_string()))?;
    // println!("vmms: {:?}", vmms);
    // extract the supported verification methods from did document
    // let verification_methods_in_did_doc: Vec<String> = issuer_did_doc
    //     .get_verification_method_ids(verification_relationship)
    //     .unwrap_or_else(|_| Vec::new());
    presentation
        .proof
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
                        if vmms.contains(verification_method) {
                            // println!("proof found {}", verification_method);
                            Some(verification_method)
                        } else {
                            // println!("proof not found");
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

// INFO: is provided by get_did_doc_from_presentation. I'll keep it around in case a separate verification is needed
// /// verify_did_in_did_doc verifies that the DID is the subject of the DID Document or returns an error.
// pub fn verify_did_in_did_doc(did: &str, did_doc: CredentialSubject) -> Result<bool, DIDError> {
//     // ensure that the DID is the subject of the DID Document
//     did_doc
//         .id
//         .ok_or(DIDError::DIDMismatch(format!(
//             "DID Doc is missing id with value {}",
//             did
//         )))
//         .and_then(|id| {
//             if id.to_string() == did {
//                 return Ok(true);
//             }
//             Err(DIDError::DIDMismatch(format!(
//                 "DID Doc has been issued for {} instead of {}",
//                 id, did
//             )))
//         })
// }

/// verify_presentation verifies the integrity and authenticity of a presentation and its included credentials.
/// Returns the verification result and the first credential that was issued to the DID specified by id containing the
/// new DID Document.
pub async fn verify_presentation(
    config: &rocket::State<Config>,
    proof_parameters: ProofParameters,
    presentation: Json<Presentation>,
) -> Result<(VerificationResult, Credential, CredentialSubject), DIDError> {
    let opts = LinkedDataProofOptions {
        challenge: proof_parameters.challenge, // fail if challenge is not present
        domain: Some(proof_parameters.domain.to_string()),
        proof_purpose: Some(proof_parameters.proof_purpose),
        // created: xx; // TODO this is set to now_ms, not sure if that's correct .. I guess that is should have be created max a minute ago
        ..LinkedDataProofOptions::default()
    };

    // INFO: Not sure how to replace this with a simple function call that integrates the test resolver on demand!
    let mut resolvers: Vec<&dyn ssi::did_resolve::DIDResolver> = vec![];
    #[cfg(test)]
    let test_resolver = DIDWebTestResolver {
        store: Some(&config.store),
        ..DIDWebTestResolver::default()
    };
    #[cfg(test)]
    resolvers.push(&test_resolver);
    let default_resolver = config.reslover_options.get_resolver();
    resolvers.push(&default_resolver);
    let resolver = SeriesResolver { resolvers };

    let mut context_loader = ContextLoader::default();
    let result = presentation
        .verify(Some(opts), &resolver, &mut context_loader)
        .await;

    // // debug output
    // println!("checks {}", result.checks.len());
    // println!("warn {}", result.warnings.len());
    // println!("errors {}", result.errors.len());
    // println!("errors: {}", result.errors.join(", "));

    if !result.errors.is_empty() {
        return Err(DIDError::PresentationInvalid(
            "Presentation invalid, verification failed".to_string(),
        ));
    }

    let presentation = presentation.into_inner();
    let (vc, new_did_doc) = get_did_doc_from_presentation(&presentation, proof_parameters.did)?;

    // ensure that inssuance_date is not in the future
    compare_date(&vc.issuance_date, Ordering::Less, Utc::now()).ok_or_else(|| {
        DIDError::PresentationInvalid(
            "Presentation invalid, DID Doc credential has been issued in the future or has no issuance date".to_string(),
            ) })?;

    // verify expiration_date as it's not verified by verify() https://github.com/spruceid/ssi/issues/470
    match &vc.expiration_date {
        Some(expiration_date) => compare_date(
            &Some(expiration_date.clone()),
            Ordering::Greater,
            Utc::now(),
        )
        .ok_or_else(|| {
            DIDError::PresentationInvalid(
                "Presentation invalid, DID Doc credential has expired".to_string(),
            )
        }),
        _ => Ok(Ordering::Greater),
    }?;
    // TODO: verify "not before use" date - applies only to JWT claims
    Ok((result, vc, new_did_doc))
}
