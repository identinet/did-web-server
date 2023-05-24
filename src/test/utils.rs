use chrono::prelude::*;
use ssi::did_resolve::SeriesResolver;

use ssi_json_ld::ContextLoader;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use ssi::ldp::ProofSuiteType;
use ssi::one_or_many::OneOrMany;
use ssi::vc::{
    Context, Contexts, Credential, CredentialOrJWT, LinkedDataProofOptions, Presentation,
    StringOrURI, VCDateTime, URI,
};

pub fn read_file(filename: &str) -> Result<String, String> {
    fs::read(PathBuf::from(filename))
        .map_err(|_| "file access failed".to_string())
        .and_then(|b| String::from_utf8(b).map_err(|_| "conversion failure".to_string()))
}

pub fn json_file_to_attributes_or_panic(
    filename: &str,
) -> HashMap<String, rocket::serde::json::serde_json::Value> {
    let doc = read_file(filename);
    assert!(
        doc.is_ok(),
        "When a fixture is read, then it's returned successfully."
    );
    let doc = doc.unwrap();
    serde_json::from_str::<HashMap<String, rocket::serde::json::serde_json::Value>>(&doc).unwrap()
}

pub async fn create_credential_or_panic(
    issuer: &str,
    id: &str,
    subject_id: &str,
    attributes: Option<HashMap<String, rocket::serde::json::serde_json::Value>>,
    issuance_date: Option<VCDateTime>,
    expiration_date: Option<VCDateTime>,
    resolver: &SeriesResolver<'_>,
    verification_method: &str,
    key: &ssi::jwk::JWK,
) -> Credential {
    let mut credential = Credential {
        context: Contexts::One(Context::URI(URI::String(
            "https://www.w3.org/2018/credentials/v1".to_string(),
        ))),
        id: Some(StringOrURI::String(subject_id.to_string())),
        type_: OneOrMany::One("VerifiableCredential".to_string()),
        credential_subject: OneOrMany::One(ssi::vc::CredentialSubject {
            id: Some(URI::String(id.to_string())),
            property_set: attributes,
        }),
        issuer: Some(ssi::vc::Issuer::URI(URI::String(issuer.to_string()))),
        issuance_date: issuance_date.or_else(|| Some(VCDateTime::from(Utc::now()))),
        proof: None, // added later
        expiration_date,
        credential_status: None,
        terms_of_use: None,
        evidence: None,
        credential_schema: None,
        refresh_service: None,
        property_set: None,
    };
    let mut context_loader = ContextLoader::default();
    let proof = match credential
        .generate_proof(
            &key,
            &LinkedDataProofOptions {
                type_: Some(ProofSuiteType::Ed25519Signature2020),
                proof_purpose: Some(ssi::vc::ProofPurpose::AssertionMethod),
                verification_method: Some(URI::String(verification_method.to_string())),
                ..LinkedDataProofOptions::default()
            },
            resolver,
            &mut context_loader,
        )
        .await
    {
        Ok(proof) => Ok(proof),

        Err(e) => {
            eprintln!("error, {}", e);
            Err(e)
        }
    }
    .unwrap();
    credential.add_proof(proof);
    credential
}

pub async fn create_presentation_or_panic(
    holder: &str,
    credentials: OneOrMany<CredentialOrJWT>,
    proof_options: &LinkedDataProofOptions,
    resolver: &SeriesResolver<'_>,
    key: &ssi::jwk::JWK,
) -> Presentation {
    let mut presentation = Presentation {
        holder: Some(URI::String(holder.to_string())), // holder must be present, otherwise the presentation can't be verified
        verifiable_credential: Some(credentials),
        ..Presentation::default()
    };
    let mut context_loader = ContextLoader::default();
    let proof = match presentation
        .generate_proof(&key, &proof_options, resolver, &mut context_loader)
        .await
    {
        Ok(proof) => Ok(proof),

        Err(e) => {
            eprintln!("error, {}", e);
            Err(e)
        }
    }
    .unwrap();
    presentation.add_proof(proof);
    presentation
}
