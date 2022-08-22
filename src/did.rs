use crate::error::DIDError;
use regex::Regex;
use std::fmt;

static URL_SEGMENT_SEPARATOR: &'static str = "/";

#[derive(Debug)]
pub struct DIDWeb {
    host: DIDSegment,
    id: Vec<DIDSegment>,
}

// TODO: wie implementiere ich einen Custom String Typ, der besondere Anforderungen / Traits
// implemeniert / erfuellt?  Eigentlich geht es um die Daten im String, die bestimmten Ansprchen
// genugen mssen
#[derive(Debug)]
struct DIDSegment {
    segment: String,
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

impl fmt::Display for DIDSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.segment)
    }
}

impl DIDWeb {
    pub fn new(host: &str, path: &str, id: &str) -> Result<DIDWeb, DIDError> {
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
            id: _id,
        })
    }
    pub fn name<'a>() -> &'a str {
        "web"
    }
}

impl fmt::Display for DIDWeb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x = "".to_string();
        for e in self.id.iter() {
            x.push_str(&format!("{}", e))
        }
        // can we collect the string differently?
        write!(
            f,
            "did:{}:{}:{}",
            DIDWeb::name(),
            self.host,
            x // self.id.iter().collect::<String>()
        )
    }
}
