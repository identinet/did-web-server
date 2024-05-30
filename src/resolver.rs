// SPDX-License-Identifier: Apache-2.0
// Copyright 2021, Spruce Systems, Inc. All rights reserved.
// Source: https://github.com/spruceid/didkit/blob/main/cli/src/opts.rs

use did_jwk::DIDJWK;
use did_method_key::DIDKey;
use did_web::DIDWeb;
use lazy_static::lazy_static;
use ssi::did::DIDMethods;
use ssi::did_resolve::{HTTPDIDResolver, SeriesResolver};

#[derive(Debug, Clone, Default)]
pub struct ResolverOptions {
    /// Fallback DID Resolver HTTP(S) endpoint, for non-built-in DID methods.
    pub did_resolver: Option<HTTPDIDResolver>,
    /// Override DID Resolver HTTP(S) endpoint, for all DID methods.
    pub did_resolver_override: Option<HTTPDIDResolver>,
}

lazy_static! {
    static ref DID_METHODS: DIDMethods<'static> = {
        let mut methods = DIDMethods::default();
        methods.insert(Box::new(DIDKey));
        methods.insert(Box::new(DIDJWK));
        methods.insert(Box::new(DIDWeb));
        methods
    };
}

impl ResolverOptions {
    pub fn get_resolver(&self) -> SeriesResolver {
        let mut resolvers = vec![DID_METHODS.to_resolver()];
        if let Some(resolver) = &self.did_resolver {
            resolvers.push(resolver);
        }
        if let Some(resolver) = &self.did_resolver_override {
            resolvers.insert(0, resolver);
        }
        SeriesResolver { resolvers }
    }
}
