use crate::{config::AppConfig, mpsc::ChannelReceiver, utils::logger};
use content::templates::{i18n::TaggedContentBuilder, I18N_STATIC_CONTENT};
use error::Error;
use mpsc::TxMessage;
use std::sync::LazyLock;
use std::{
    env,
    sync::Arc,
};
use tokio::sync::Mutex;

pub mod config;
pub mod content;
pub mod error;
pub mod models;
pub mod mpsc;
pub mod server;
pub mod utils;

#[allow(clippy::type_complexity)]
static APP_CONFIG: LazyLock<Mutex<Option<Box<AppConfig>>>> =
    LazyLock::new(|| Mutex::new(Some(Box::default())));

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenv::from_filename(".env.development").ok();

    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "gadget-hub=trace,tower_http=trace,tokio=trace,runtime=trace",
        )
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .init();

    let new_config = config::config().await.expect("Loads config");
    tracing::info!("Config: {:#?}", new_config);
    let arc_config = Arc::new(new_config.clone());

    // Apply configurations to the global state here.
    {
        let config_lock = &mut *APP_CONFIG.lock().await;
        if let Some(config) = config_lock {
            *config = Box::new(new_config);
        }

        let i18n_content = &mut *I18N_STATIC_CONTENT.lock().unwrap();
        if let Some(i18n) = i18n_content {
            let mut builder = TaggedContentBuilder::new();
            let content = builder
                .add("site_name_short", "Nosferatu".to_string())
                .add(
                    "site_description",
                    "Static site with Axum and Askama".to_string(),
                )
                .build();

            i18n.create_language("en");
            i18n.add_to_content("en", content);
        }
    } // This block ensures we drop the lock here.

    // Spin up our API
    let host: &str = &env::var("SERVER_BIND_HOST").unwrap_or("0.0.0.0".to_string());
    let port: u16 = env::var("SERVER_BIND_PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .expect("Unable to parse port!");
    let addr = server::common::NetworkAddr::new(host, port);
    logger::log(
        logger::Level::Info,
        logger::Color(utils::YELLOW),
        logger::Tag("[ OK ]"),
        logger::Text(format!("Listening on {}", &addr.to_string()).as_str()),
    );

    // Setup mpsc
    let (tx, receiver) = tokio::sync::mpsc::channel::<TxMessage>(32);
    let mut rx = ChannelReceiver::new(receiver);

    // let config = config::config().await.expect("Loads config");
    let backend = async move { server::serve(&arc_config, addr, tx).await };

    // single consumer
    tokio::spawn(async move {
        if let Err(err) = rx.run().await {
            let err_message = format!("Error when spawning single consumer! {}", err);
            tracing::error!("{}", err_message);

            return Err(Error::new(err_message));
        }

        Ok(())
    });

    let public_addr = server::common::NetworkAddr::new("0.0.0.0", 9002);
    tokio::join!(
        server::public::serve_barebones(server::public::public_dir(), public_addr),
        backend,
    );

    Ok(())
}
