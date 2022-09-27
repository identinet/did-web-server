use ssi::did_resolve::HTTPDIDResolver;

use crate::error::DIDError;
use crate::resolver::ResolverOptions;
use crate::store::file::FileStore;
use crate::store::{mem::MemStore, DIDWebStore};
use crate::util::get_env;

/// Global configuration
///
/// * `hostname` - Hostname for `did:web:<hostname>`. Set via EXTERNAL_HOSTNAME variable, e.g. `example.com`
/// * `port` - , e.g. `example.com`
/// * `path` - Path to the identity `did:web:<domainname>:<path>/<id>. Set via SUBPATH variable, e.g. `users`
/// * `didstore` - Directory to store the DID Documents at, default: `$PWD/did_store`
pub struct Config {
    pub external_hostname: String,
    pub external_port: String,
    pub did_method_path: String,
    pub store: Box<dyn DIDWebStore + Sync + Send>,
    pub reslover_options: ResolverOptions,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            external_hostname: get_env("DID_SERVER_EXTERNAL_HOSTNAME", "localhost"),
            external_port: get_env("DID_SERVER_EXTERNAL_PORT", "8000"),
            did_method_path: get_env("DID_SERVER_EXTERNAL_PATH", "/"),
            reslover_options: ResolverOptions {
                did_resolver: std::env::var("DID_SERVER_RESOLVER")
                    .ok()
                    .map(|ref url| HTTPDIDResolver::new(url)),
                did_resolver_override: std::env::var("DID_SERVER_RESOLVER_OVERRIDE")
                    .ok()
                    .map(|ref url| HTTPDIDResolver::new(url)),
            },
            store: Ok(get_env("DID_SERVER_BACKEND", "mem"))
                .and_then(
                    |backend| -> Result<Box<dyn DIDWebStore + Sync + Send>, DIDError> {
                        match backend.as_str() {
                            "file" => {
                                let directory = get_env(
                                    "DID_SERVER_BACKEND_FILE_STORE",
                                    // by default store all files in $PWD/did_store/
                                    &std::env::current_dir()
                                        .map(|val| {
                                            val.join("did_store")
                                                .to_str()
                                                .unwrap_or(".")
                                                .to_string()
                                        })
                                        .unwrap_or_else(|_| ".".to_string()),
                                );
                                Ok(Box::new(FileStore::new(directory)))
                            }
                            "mem" => Ok(Box::new(MemStore::new())),
                            _ => Err(DIDError::UnknownBackend(format!(
                                "Backend is unknown: {}",
                                backend
                            ))),
                        }
                    },
                )
                .unwrap(), // WARNING: panic if backend is incorrect
        }
    }
}
