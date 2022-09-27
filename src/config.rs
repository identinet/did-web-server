use ssi::did_resolve::HTTPDIDResolver;

use crate::error::DIDError;
use crate::resolver::ResolverOptions;
use crate::store::file::FileStore;
use crate::store::{mem::MemStore, DIDWebStore};
use crate::util::get_env;

/// Global configuration
///
/// * `external_hostname` - Hostname for `did:web:<hostname>`. Set via EXTERNAL_HOSTNAME variable, e.g. `example.com`
/// * `external_path` - Path to the identity `did:web:<domainname>:<path>/<id>. Set via SUBPATH variable, e.g. `users`
/// * `external_port` - , e.g. `example.com`
/// * `owner_did` - DID of the server's owner
/// * `reslover_options` - Directory to store the DID Documents at, default: `$PWD/did_store`
/// * `store` - Store for DID Documents
pub struct Config {
    pub external_path: String,
    pub external_hostname: String,
    pub external_port: String,
    pub owner: String,
    pub reslover_options: ResolverOptions,
    pub store: Box<dyn DIDWebStore + Sync + Send>,
}

impl Config {
    pub fn load_env_or_panic(config: Config) -> Config {
        Config {
            external_hostname: get_env("DID_SERVER_EXTERNAL_HOSTNAME", &config.external_hostname),
            external_port: get_env("DID_SERVER_EXTERNAL_PORT", &config.external_port),
            external_path: get_env("DID_SERVER_EXTERNAL_PATH", &config.external_path),
            owner: std::env::var("DID_SERVER_OWNER").unwrap(),
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
                .unwrap(),
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            external_hostname: "localhost".to_string(),
            external_port: "8000".to_string(),
            external_path: "/".to_string(),
            owner: "_did:in:valid".to_string(),
            reslover_options: ResolverOptions {
                did_resolver: None,
                did_resolver_override: None,
            },
            store: Box::new(MemStore::new()),
        }
    }
}
