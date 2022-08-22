use crate::error::DIDError;
use regex::Regex;
use std::fmt;

static URL_SEGMENT_SEPARATOR: &'static str = "/";

#[derive(Debug)]
pub struct DIDWeb {
    host: DIDSegment, // FIXME: accept host names
    port: u16,        // FIXME: allow only valid ports to be stored here, .. maybe?
    id: Vec<DIDSegment>,
}

impl fmt::Display for DIDWeb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut id = "".to_string();
        for e in self.id.iter() {
            id.push_str(&format!("{}", e))
        }
        let host = if self.host.to_string() != "localhost" && self.port == 443 {
            self.host.to_string()
        } else {
            // join host with port host as specified in https://w3c-ccg.github.io/did-method-web/#method-specific-identifier
            format!("{}%3A{}", self.host, self.port)
        };

        // can we collect the string differently?
        write!(
            f,
            "did:{}:{}:{}",
            DIDWeb::name(),
            host,
            id // self.id.iter().collect::<String>()
        )
    }
}

impl DIDWeb {
    pub fn new(host: &str, port: &str, path: &str, id: &str) -> Result<DIDWeb, DIDError> {
        let port = match port.parse::<u16>() {
            Ok(port) => {
                if port == 0 {
                    return Err(DIDError::DIDPortNotAllowed(
                        "Port '0' out of range, expected 1-65535".to_string(),
                    ));
                }
                port
            }
            Err(e) => {
                if port == "" {
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
            if _p != "" {
                _id.push(DIDSegment::from(path)?);
            }
        }
        _id.push(DIDSegment::from(id)?);
        Ok(DIDWeb {
            host: DIDSegment::from(host)?,
            port,
            id: _id,
        })
    }
    pub fn name<'a>() -> &'a str {
        "web"
    }
}

// TODO: wie implementiere ich einen Custom String Typ, der besondere Anforderungen / Traits
// implemeniert / erfuellt?  Eigentlich geht es um die Daten im String, die bestimmten Ansprchen
// genugen mssen
#[derive(Debug)]
struct DIDSegment {
    segment: String,
}

impl fmt::Display for DIDSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.segment)
    }
}

impl DIDSegment {
    fn from(segment: &str) -> Result<DIDSegment, DIDError> {
        // ensure that each segment conforms to the DID Syntax specification: https://w3c.github.io/did-core/#did-syntax
        let re = Regex::new(r"^([a-zA-Z._-]|%[A-F][A-F])+$").unwrap();
        if re.is_match(segment) {
            Ok(DIDSegment {
                segment: segment.to_owned(),
            })
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
