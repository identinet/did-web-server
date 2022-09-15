use crate::util::get_env;
use std::path::PathBuf;

/// Global configuration
///
/// * `hostname` - Hostname for `did:web:<hostname>`. Set via EXTERNAL_HOSTNAME variable, e.g. `example.com`
/// * `port` - , e.g. `example.com`
/// * `path` - Path to the identity `did:web:<domainname>:<path>/<id>. Set via SUBPATH variable, e.g. `users`
/// * `didstore` - Directory to store the DID Documents at, default: `$PWD/did_store`
#[derive(Debug)]
pub struct Config {
    pub external_hostname: String,
    pub external_port: String,
    pub did_method_path: String,
    pub didstore: PathBuf,
    pub did_resolver: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            external_hostname: get_env("EXTERNAL_HOSTNAME", "localhost"),
            external_port: get_env("EXTERNAL_PORT", "8000"),
            did_method_path: get_env("EXTERNAL_PATH", "/"),
            didstore: PathBuf::from(&get_env(
                "DID_STORE",
                // by default store all files in $PWD/did_store/
                &std::env::current_dir()
                    .map(|val| val.join("did_store").to_str().unwrap_or(".").to_string())
                    .unwrap_or_else(|_| ".".to_string()),
            )),
            did_resolver: get_env(
                "DID_RESOLVER_OVERRIDE",
                "http://localhost:8080/1.0/identifiers/",
            ),
        }
    }
}
