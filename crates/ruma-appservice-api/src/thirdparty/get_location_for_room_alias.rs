//! `GET /_matrix/app/*/thirdparty/location`
//!
//! Endpoint to retrieve an array of third party network locations from a Matrix room alias.

pub mod v1 {
    //! `/v1/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/v1.4/application-service-api/#get_matrixappv1thirdpartylocation

    use ruma_common::{
        api::{request, response, Metadata},
        metadata,
        thirdparty::Location,
        RoomAliasId,
    };

    const METADATA: Metadata = metadata! {
        description: "Retrieve an array of third party network locations from a Matrix room alias.",
        method: GET,
        name: "get_location_for_room_alias",
        rate_limited: false,
        authentication: AccessToken,
        history: {
            1.0 => "/_matrix/app/v1/thirdparty/location",
        }
    };

    #[request]
    pub struct Request<'a> {
        /// The Matrix room alias to look up.
        #[ruma_api(query)]
        pub alias: &'a RoomAliasId,
    }

    #[response]
    pub struct Response {
        /// List of matched third party locations.
        #[ruma_api(body)]
        pub locations: Vec<Location>,
    }

    impl<'a> Request<'a> {
        /// Creates a new `Request` with the given room alias id.
        pub fn new(alias: &'a RoomAliasId) -> Self {
            Self { alias }
        }
    }

    impl Response {
        /// Creates a new `Response` with the given locations.
        pub fn new(locations: Vec<Location>) -> Self {
            Self { locations }
        }
    }
}
