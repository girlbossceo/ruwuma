//! `POST /_matrix/client/*/room_keys/version`

pub mod v3 {
    //! `/v3/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/v1.4/client-server-api/#post_matrixclientv3room_keysversion

    use ruma_common::{
        api::{request, response, Metadata},
        metadata,
        serde::Raw,
    };

    use crate::backup::BackupAlgorithm;

    const METADATA: Metadata = metadata! {
        description: "Create a new backup version.",
        method: POST,
        name: "create_backup_version",
        rate_limited: true,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/client/unstable/room_keys/version",
            1.1 => "/_matrix/client/v3/room_keys/version",
        }
    };

    #[request(error = crate::Error)]
    pub struct Request {
        /// The algorithm used for storing backups.
        #[ruma_api(body)]
        pub algorithm: Raw<BackupAlgorithm>,
    }

    #[response(error = crate::Error)]
    pub struct Response {
        /// The backup version.
        pub version: String,
    }

    impl Request {
        /// Creates a new `Request` with the given backup algorithm.
        pub fn new(algorithm: Raw<BackupAlgorithm>) -> Self {
            Self { algorithm }
        }
    }

    impl Response {
        /// Creates a new `Response` with the given version.
        pub fn new(version: String) -> Self {
            Self { version }
        }
    }
}
