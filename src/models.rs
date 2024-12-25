use crate::error::Error;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;

pub mod postgres {
    use super::*;

    pub mod config {
        use super::*;
        use std::fmt;

        #[derive(Clone)]
        #[allow(dead_code)]
        pub struct PgConfig {
            pub url: String,
            pub connect_timeout: String,
            pub idle_timeout: String,
            pub max_lifetime: String,
            pub min_connections: String,
            pub max_connections: String,
        }

        impl fmt::Debug for PgConfig {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("PgConfig")
                    .field("url", &crate::config::sanitize_db_url(&self.url))
                    .field("connect_timeout", &self.connect_timeout)
                    .field("idle_timeout", &self.idle_timeout)
                    .field("max_lifetime", &self.max_lifetime)
                    .field("min_connections", &self.min_connections)
                    .field("max_connections", &self.max_connections)
                    .finish()
            }
        }

        pub async fn pg_connection(
            url: &str,
            connect_timeout: &str,
            idle_timeout: &str,
            max_lifetime: &str,
            min_connections: &str,
            max_connections: &str,
        ) -> Result<Result<sqlx::PgPool, sqlx::Error>, Error> {
            let pool = PgPoolOptions::new()
                .acquire_timeout(std::time::Duration::from_secs(
                    connect_timeout.parse().expect("Unable to parse into u64!"),
                ))
                .idle_timeout(std::time::Duration::from_secs(
                    idle_timeout.parse().expect("Unable to parse into u64!"),
                ))
                .max_lifetime(std::time::Duration::from_secs(
                    max_lifetime.parse().expect("Unable to parse into u64!"),
                ))
                .min_connections(min_connections.parse().expect("Unable to parse into u32!"))
                .max_connections(max_connections.parse().expect("Unable to parse into u32!"))
                .connect(url)
                .await;

            tracing::info!(
                "[ OK ]: Opening PostgreSQL DB connection to URL: {}",
                crate::config::sanitize_db_url(url)?
            );

            Ok(pool)
        }
    }
}
