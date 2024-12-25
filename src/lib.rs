mod error;
pub mod utils;

pub mod prelude {
    pub use super::utils::{self, logger};

    #[allow(unused_imports)]
    pub mod axum_prelude {
        pub use axum::{
            body::Body,
            response::{IntoResponse, Response},
            routing::{delete, get, post, Router},
        };
        pub use hyper::StatusCode;
    }
}
