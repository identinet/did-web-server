use crate::config::Config;
use crate::error::DIDError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha256::digest;
use ssi::did::Document;
use std::{fmt, path::PathBuf};

static URL_SEGMENT_SEPARATOR: &str = "/";

/// ProofParameters are reqiured to compute the Verifiable Presentation for updating the stored DID
/// Dcoument.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProofParameters {
    /// Challenge is the sha256 hash of the current DID Document
    challenge: String,
    domain: String,
    proof_purpose: String,
    did: String,
}

impl ProofParameters {
    /// Create a new ProofParameters struct
    pub fn new(
        config: &rocket::State<Config>,
        doc: &Document,
    ) -> Result<ProofParameters, DIDError> {
        serde_json::to_string(doc)
            .map_err(|e| DIDError::ContentConversion(e.to_string()))
            .map(|s| ProofParameters {
                challenge: digest(s),
                domain: config.hostname.to_string(),
                proof_purpose: "authentication".to_string(),
                did: doc.id.to_string(),
            })
    }
}

impl fmt::Display for ProofParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        serde_json::to_string(&self)
            .map_err(|_| fmt::Error)
            .and_then(|s| write!(f, "{}", s))
    }
}

/// DIDWeb represents a DID of the DID Web method, see https://w3c-ccg.github.io/did-method-web/
#[derive(Debug, Deserialize, Serialize)]
pub struct DIDWeb {
    host: DIDSegment, // FIXME: accept all valid host names and not just DIDSegment
    port: u16, // FIXME: allow only valid ports to be stored here, .. maybe? create a custom type that excludes 0
    id: Vec<DIDSegment>,
}

impl fmt::Display for DIDWeb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let host = if self.host.to_string() != "localhost" && self.port == 443 {
            self.host.to_string()
        } else {
            // join host with port host as specified in https://w3c-ccg.github.io/did-method-web/#method-specific-identifier
            format!("{}%3A{}", self.host, self.port)
        };
        let id = self
            .id
            .iter()
            .map(|s| s.to_string()) // TODO: maybe I can extend DIDSgement to implement the Concat/Join traits to allow a direct join
            .collect::<Vec<String>>()
            .join(":");
        if id.is_empty() {
            write!(f, "did:{}:{}", DIDWeb::name(), host,)
        } else {
            write!(f, "did:{}:{}:{}", DIDWeb::name(), host, id)
        }
    }
}

impl DIDWeb {
    pub fn new(host: &str, port: &str, path: &str, id: &PathBuf) -> Result<DIDWeb, DIDError> {
        let port = match port.parse::<u16>() {
            Ok(port) => {
                if port == 0 {
                    return Err(DIDError::DIDPortNotAllowed(
                        "Port '0' out of range, expected value between 1 and 65535".to_string(),
                    ));
                }
                port
            }
            Err(e) => {
                if port.is_empty() {
                    if host == "localhost" {
                        8080_u16
                    } else {
                        443_u16
                    }
                } else {
                    return Err(DIDError::DIDPortNotAllowed(e.to_string()));
                }
            }
        };
        let mut _id = vec![];
        for p in path.split(URL_SEGMENT_SEPARATOR) {
            let _p = p.trim();
            if !_p.is_empty() {
                _id.push(DIDSegment::from(_p)?);
            }
        }

        // Test if filename is did.json .. otherwise bail out
        if id
            .file_name()
            .and_then(|filename| filename.to_str())
            .and_then(|filename| {
                if filename == "did.json" {
                    Some(filename)
                } else {
                    None
                }
            })
            .is_none()
        {
            return Err(DIDError::DIDMismatch(
                "path segment not allowed".to_string(),
            ));
        }

        if id.is_absolute() && *id != PathBuf::from("/.well-known/did.json")
            || id.is_relative() && *id != PathBuf::from(".well-known/did.json")
        {
            // TODO: I don't understand the clippy suggestion .. does it apply here?
            #[allow(clippy::for_loops_over_fallibles)]
            for segment in id.parent().iter() {
                match segment.to_str() {
                    Some(_segment) => _id.push(DIDSegment::from(_segment)?),
                    None => {
                        return Err(DIDError::DIDMismatch(
                            "path segment not allowed".to_string(),
                        ))
                    }
                }
            }
        }

        Ok(DIDWeb {
            host: DIDSegment::from(host)?,
            port,
            id: _id,
        })
    }

    /// Returns the name of the DID method.
    pub fn name<'a>() -> &'a str {
        "web"
    }

    /// Builds DID from the service configuration and id.
    ///
    /// * `config` - service configuration.
    /// * `id` - requested id.
    pub fn did_from_config(
        config: &rocket::State<Config>,
        id: &PathBuf,
    ) -> Result<DIDWeb, DIDError> {
        DIDWeb::new(&config.hostname, &config.port, &config.did_method_path, id)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct DIDSegment(String);

impl fmt::Display for DIDSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DIDSegment {
    fn from(segment: &str) -> Result<DIDSegment, DIDError> {
        // ensure that each segment conforms to the DID Syntax specification: https://w3c.github.io/did-core/#did-syntax
        let re = Regex::new(r"^([a-zA-Z._-]|%[A-F][A-F])+$").unwrap();
        if re.is_match(segment) {
            Ok(DIDSegment(segment.to_owned()))
        } else {
            Err(DIDError::IllegalCharacter(
                "segment contains illegal character".to_string(),
            ))
        }
    }
}

// // impl std::str::Join<&str> for DIDSegment {
// impl<DIDSegment> [DIDSegment] {
//     fn join<'a>(slice: &'a Self, sep: &'a str) -> &'a str {
//         slice.segment.push_str(sep)
//     }
// }

#[cfg(test)]
mod test {
    use crate::did::*;

    #[test]
    fn test_did_web() {
        let host = "";
        let port = "80";
        let path = "/";
        let id = PathBuf::from("abc");
        let result = DIDWeb::new(host, port, path, &id);
        assert!(
            result.is_err(),
            "When <host> is empty, then error is returned"
        );

        let host = "example.com";
        let port = "80";
        let path = "/";
        let id = PathBuf::from("abc");
        let result = DIDWeb::new(host, port, path, &id);
        assert!(
            result.is_err(),
            "When <id> doesn't end in /did.json, then an error is thrown"
        );

        let host = "example.com";
        let port = "";
        let path = "/";
        let id = PathBuf::from("abc/did.json");
        let result = DIDWeb::new(host, port, path, &id).unwrap();
        assert_eq!(
            result.to_string(),
            "did:web:example.com:abc",
            "When <port> is empty, then the default port is assumed"
        );

        let host = "localhost";
        let port = "";
        let path = "/";
        let id = PathBuf::from("abc/did.json");
        let result = DIDWeb::new(host, port, path, &id).unwrap();
        assert_eq!(
            result.to_string(),
            "did:web:localhost%3A8080:abc",
            "When <port> is empty and host is 'localhost', then the default port 8080 is assumed"
        );

        let host = "example.com";
        let port = "";
        let path = "";
        let id = PathBuf::from("abc/did.json");
        let result = DIDWeb::new(host, port, path, &id).unwrap();
        assert_eq!(
            result.to_string(),
            "did:web:example.com:abc",
            "When <path> is empty, then no path is assumed"
        );

        let host = "example.com";
        let port = "";
        let path = "";
        let id = PathBuf::from("");
        let result = DIDWeb::new(host, port, path, &id);
        assert!(
            result.is_err(),
            "When <id> is empty, then an error is thrown"
        );

        let host = "example.com";
        let port = "8443";
        let path = "a:long/path"; // `:` is the illegal characher
        let id = PathBuf::from("abc/did.json");
        let result = DIDWeb::new(host, port, path, &id);
        assert!(
            result.is_err(),
            "When <path> contains illegal characters, then an error is thrown"
        );

        let host = "example.com";
        let port = "8443";
        let path = "a/long/path";
        let id = PathBuf::from("abc/did.json");
        let result = DIDWeb::new(host, port, path, &id).unwrap();
        assert_eq!(
            result.to_string(),
            "did:web:example.com%3A8443:a:long:path:abc",
            "When a custom <host>, <port>, a long <path> and <id> are provided, then the did:web URL is correctly properly"
        );

        let host = "example.com";
        let port = "8443";
        let path = "";
        let id = PathBuf::from("/.well-known/did.json");
        let result = DIDWeb::new(host, port, path, &id).unwrap();
        assert_eq!(
            result.to_string(),
            "did:web:example.com%3A8443",
            "When /.well-known/did.json is requested, then the computed DID is just did:web::hostname%3A8443 "
        );

        let host = "example.com";
        let port = "8443";
        let path = "";
        let id = PathBuf::from(".well-known/did.json");
        let result = DIDWeb::new(host, port, path, &id).unwrap();
        assert_eq!(
            result.to_string(),
            "did:web:example.com%3A8443",
            "When .well-known/did.json is requested, then the computed DID is just did:web::hostname%3A8443 "
        );
    }
}
