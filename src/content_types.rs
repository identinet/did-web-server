// SPDX-License-Identifier: AGPL-3.0

use rocket::http::{ContentType, MediaType};

pub struct DIDContentTypes;

impl DIDContentTypes {
    pub const DID_LD_JSON: ContentType = ContentType(MediaType::const_new(
        "application",
        "did+ld+json",
        &[("", "")],
    ));
    // pub const DID_JSON: ContentType =
    //     ContentType(MediaType::const_new("application", "did+json", &[("", "")]));
}
