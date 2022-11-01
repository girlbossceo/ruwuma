//! `GET /_matrix/federation/*/openid/userinfo`
//!
//! Endpdoint for retrieving OpenID userinfo.

pub mod v1 {
    //! `/v1/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/v1.4/server-server-api/#get_matrixfederationv1openiduserinfo

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, OwnedUserId,
    };

    const METADATA: Metadata = metadata! {
        description: "Exchanges an OpenID access token for information about the user who generated the token.",
        method: GET,
        name: "get_openid_userinfo",
        rate_limited: false,
        authentication: None,
        history: {
            1.0 => "/_matrix/federation/v1/openid/userinfo",
        }
    };

    #[request]
    pub struct Request<'a> {
        /// The OpenID access token to get information about the owner for.
        #[ruma_api(query)]
        pub access_token: &'a str,
    }

    #[response]
    pub struct Response {
        /// The Matrix User ID who generated the token.
        pub sub: OwnedUserId,
    }

    impl<'a> Request<'a> {
        /// Creates a new `Request` with the given access token.
        pub fn new(access_token: &'a str) -> Self {
            Self { access_token }
        }
    }

    impl Response {
        /// Creates a new `Response` with the given user id.
        pub fn new(sub: OwnedUserId) -> Self {
            Self { sub }
        }
    }
}
