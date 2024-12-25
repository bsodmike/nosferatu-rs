use crate::error::Error;
use crate::models::postgres::config::PgConfig;
use anyhow::anyhow;
use regex::Captures;
use regex::Regex;
use std::env;

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    // pub aws_creds: AwsCredentials,
    // pub aws_region: Region,
    pub pg_pool: Option<sqlx::PgPool>,
    pub pg_config: Option<PgConfig>,
}

pub async fn config() -> Result<AppConfig, Error> {
    let pg_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing!");
    let pg_connect_timeout =
        env::var("POSTGRES_CONNECT_TIMEOUT").expect("POSTGRES_CONNECT_TIMEOUT is missing!");
    let pg_idle_timeout =
        env::var("POSTGRES_IDLE_TIMEOUT").expect("POSTGRES_IDLE_TIMEOUT is missing!");
    let pg_max_lifetime =
        env::var("POSTGRES_MAX_LIFETIME").expect("POSTGRES_MAX_LIFETIME is missing!");
    let pg_min_connections =
        env::var("POSTGRES_MIN_CONNECTIONS").expect("POSTGRES_MIN_CONNECTIONS is missing!");
    let pg_max_connections =
        env::var("POSTGRES_MAX_CONNECTIONS").expect("POSTGRES_MAX_CONNECTIONS is missing!");

    let pg_pool: sqlx::PgPool = match crate::models::postgres::config::pg_connection(
        &pg_url,
        &pg_connect_timeout,
        &pg_idle_timeout,
        &pg_max_lifetime,
        &pg_min_connections,
        &pg_max_connections,
    )
    .await?
    {
        Ok(value) => value,
        Err(err) => {
            return Err(anyhow::anyhow!(err)
                .context(format!(
                    "Unable to establish a connection to Postgres at URL: {}",
                    &pg_url
                ))
                .into())
        }
    };

    Ok(AppConfig {
        pg_config: Some(PgConfig {
            url: pg_url,
            connect_timeout: pg_connect_timeout,
            idle_timeout: pg_idle_timeout,
            max_lifetime: pg_max_lifetime,
            min_connections: pg_min_connections,
            max_connections: pg_max_connections,
        }),
        pg_pool: Some(pg_pool),
    })
}

pub fn sanitize_db_url(url: &str) -> Result<String, Error> {
    let re = Regex::new(r"^(postgres://[a-zA-Z\d\-\S]+):([a-zA-Z\d\-\S]*)@")?;
    let result = re
        .replace(url, |caps: &Captures<'_>| {
            format!("{}:<PASSWORD_REDACTED>@", &caps[1])
        })
        .to_string();

    Ok(result)
}
