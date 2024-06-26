// SPDX-License-Identifier: AGPL-3.0

use std::process;

use ssi::did_resolve::HTTPDIDResolver;

use crate::error::DIDError;
use crate::resolver::ResolverOptions;
use crate::store::file::FileStore;
use crate::store::{mem::MemStore, DIDWebStore};
use crate::utils::get_env;

/// Global configuration
///
/// * `external_hostname` - Hostname for `did:web:<hostname>`. Set via EXTERNAL_HOSTNAME variable, e.g. `example.com`
/// * `external_path` - Path to the identity `did:web:<domainname>:<path>/<id>. Set via SUBPATH variable, e.g. `users`
/// * `external_port` - , e.g. `8000`
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
    // INFO: I wish I could get this to work somehow
    /// get_resolver Retrieves the resolver from the configuration.
    // pub fn get_resolver<'a>(&self, custom_resolver: Option<&'a dyn DIDResolver>) -> SeriesResolver {
    //     let mut series_resolver = self.reslover_options.get_resolver();
    //     if let Some(resolver) = custom_resolver {
    //         series_resolver.resolvers.push(resolver);
    //     }
    //     series_resolver
    //     // let mut resolvers: Vec<&dyn ssi::did_resolve::DIDResolver> = vec![];
    //     // #[cfg(test)]
    //     // let test_resolver = DIDWebTestResolver {
    //     //     store: Some(&self.store),
    //     //     ..DIDWebTestResolver::default()
    //     // };
    //     // #[cfg(test)]
    //     // resolvers.push(test_resolver.as_ref());
    //     // resolvers.push(&self.reslover_options.get_resolver());
    //     // SeriesResolver { resolvers }
    // }
    // /// get_resolver Retrieves the resolver from the configuration.
    // pub fn get_test_resolver(&self) -> Option<&dyn DIDResolver> {
    //     #[cfg(test)]
    //     return Some(&DIDWebTestResolver {
    //         store: Some(&self.store),
    //         ..DIDWebTestResolver::default()
    //     });
    //     // reason = "only in test cases this code isn't used"
    //     #[allow(unreachable_code)]
    //     return None;
    // }
    pub fn load_env_or_panic(config: Config) -> Config {
        Config {
            external_hostname: get_env("DWS_EXTERNAL_HOSTNAME", &config.external_hostname),
            external_port: get_env("DWS_EXTERNAL_PORT", &config.external_port),
            external_path: get_env("DWS_EXTERNAL_PATH", &config.external_path),
            owner: std::env::var("DWS_OWNER")
                .map_err(|_| DIDError::OwnerMissing("Owner not specified".to_string()))
                .and_then(|owner| {
                    if owner.is_empty() {
                        Err(DIDError::OwnerMissing("Owner not specified".to_string()))
                    } else {
                        Ok(owner)
                    }
                })
                .map_err(|e| {
                    println!("Error: {}", e);
                    process::exit(1)
                })
                .unwrap(),
            reslover_options: ResolverOptions {
                did_resolver: std::env::var("DWS_RESOLVER")
                    .ok()
                    .map(|ref url| HTTPDIDResolver::new(url)),
                did_resolver_override: std::env::var("DWS_RESOLVER_OVERRIDE")
                    .ok()
                    .map(|ref url| HTTPDIDResolver::new(url)),
            },
            store: Ok(get_env("DWS_BACKEND", "mem"))
                .and_then(
                    |backend| -> Result<Box<dyn DIDWebStore + Sync + Send>, DIDError> {
                        match backend.as_str() {
                            "file" => {
                                let directory = get_env(
                                    "DWS_BACKEND_FILE_STORE",
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
                .map_err(|e| {
                    println!("Error: {}", e);
                    process::exit(1)
                })
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
            owner: "<invalidDID>".to_string(),
            reslover_options: ResolverOptions {
                did_resolver: None,
                did_resolver_override: None,
            },
            store: Box::new(MemStore::new()),
        }
    }
}
