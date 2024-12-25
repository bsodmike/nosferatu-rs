use super::common;
use axum::handler::HandlerWithoutStateExt;
use nosferatu::prelude::axum_prelude::*;
use nosferatu::prelude::*;
use std::net::IpAddr;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub fn public_dir() -> Router {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }
    let service_404 = handle_404.into_service();

    async fn handle_400() -> (StatusCode, &'static str) {
        (StatusCode::BAD_REQUEST, "")
    }

    let path = std::env::current_dir().expect("Unable to get current dir!");
    let public_dir = path.join("public/");

    let serve_dir = ServeDir::new(public_dir).not_found_service(service_404);

    Router::new()
        .route("/health", get(common::handle_health_get))
        .nest_service("/public", serve_dir)
        .fallback_service(handle_400.into_service())
}

pub async fn serve_barebones(app: Router, addr: common::NetworkAddr<'_>) {
    let ip: IpAddr = addr.host().parse().unwrap();
    let addr = SocketAddr::from((ip, addr.port()));
    logger::log(
        logger::Level::Info,
        logger::Color(utils::YELLOW),
        logger::Tag("[ OK ]"),
        logger::Text(format!("Listening on {:?} exposing ./public", &addr).as_str()),
    );

    axum_server::bind(addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}
