use std::path::PathBuf;

/// Global configuration
///
/// * `hostname` - Hostname for `did:web:<hostname>`. Set via EXTERNAL_HOSTNAME variable, e.g. `example.com`
/// * `port` - , e.g. `example.com`
/// * `path` - Path to the identity `did:web:<domainname>:<path>/<id>. Set via SUBPATH variable, e.g. `users`
/// * `didstore` - Directory to store the DID Documents at, default: `$PWD/did_store`
#[derive(Debug)]
pub struct Config {
    pub hostname: String,
    pub port: String,
    pub did_method_path: String,
    pub didstore: PathBuf,
}

impl Config {
    pub fn new(hostname: String, port: String, path: String, didstore: PathBuf) -> Config {
        Config {
            hostname,
            port,
            did_method_path: path,
            didstore,
        }
    }
}
