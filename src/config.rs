/// Global configuration
///
/// * `domainname` - Domain name for `did:web:<domainame>`. Set via DOMAINNAME variable, e.g. `example.com`
/// * `path` - Path to the identity `did:web:<domainname>:<path>/<id>. Set via SUBPATH variable, e.g. `users`
/// * `didstore` - Directory to store the DID Documents at, default: `$PWD/did_store`
#[derive(Debug)]
pub struct Config {
    pub domainname: String,
    pub did_method_path: String,
    pub didstore: String,
}

impl Config {
    pub fn new(domainname: String, path: String, didstore: String) -> Config {
        Config {
            domainname,
            did_method_path: path,
            didstore,
        }
    }
}
