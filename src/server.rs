use crate::server::common::return_json;
use crate::utils::{self, logger};
use crate::{error::Error, mpsc::TxMessage, AppConfig, APP_CONFIG};
use axum::{
    extract::{self, DefaultBodyLimit, Extension, Path},
    http::HeaderValue,
    response::{IntoResponse, Response},
    routing::{delete, get, post, Router},
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::{fmt, str::FromStr};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::{AllowOrigin, Any, CorsLayer},
    trace,
    trace::TraceLayer,
};
use tracing::Level;

pub mod common;
pub mod handlers;
pub mod public;

pub async fn serve(
    config: &AppConfig,
    addr: common::NetworkAddr<'_>,
    handle: mpsc::Sender<TxMessage>,
) {
    let mut app = api_router();

    app = allow_cors(app);
    app = add_middleware(config, app, handle);

    let ip: IpAddr = addr.host().parse().unwrap();
    let addr = SocketAddr::from((ip, addr.port()));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn get_middleware(config: &AppConfig, handle: mpsc::Sender<TxMessage>) -> Router {
    let mut app = api_router();
    app = add_middleware(config, app, handle);

    app
}

struct CorsOrigins<'a>(pub(crate) &'a Vec<HeaderValue>);

impl From<CorsOrigins<'_>> for AllowOrigin {
    fn from(value: CorsOrigins<'_>) -> Self {
        AllowOrigin::list(value.0.to_owned())
    }
}

impl<'a> fmt::Display for CorsOrigins<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, origin| {
            result.and_then(|_| writeln!(f, "{:?}", origin))
        })
    }
}

fn allow_cors(router: Router) -> Router {
    let origins = [
        "http://localhost:9001".parse().unwrap(),
        "http://10.2.40.53:9001".parse().unwrap(), // iPhone iOS testing locally
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_headers(Any)
        .allow_methods(Any);

    router.layer(cors)
}

fn add_middleware(config: &AppConfig, router: Router, handle: mpsc::Sender<TxMessage>) -> Router {
    router.layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            )
            .layer(CatchPanicLayer::custom(PanicLayerResponse {}))
            .layer(Extension(config.clone()))
            .layer(Extension(handle))
            .layer(DefaultBodyLimit::max(20971520)),
    )
}

fn api_router() -> Router {
    Router::new()
        .route("/health", get(crate::server::common::handle_health_get))
        .route("/", get(handlers::render_index))
        .route("/about", get(handlers::render_about))
        // .route("/login", get(templates::render_login))
        .route(
            "/panic",
            get(|| async { panic!("panic like it's 1999...") }),
        )
        .fallback(crate::content::templates::error_404_template)
}

#[derive(Clone)]
struct PanicLayerResponse {}

impl tower_http::catch_panic::ResponseForPanic for PanicLayerResponse {
    type ResponseBody = String;

    fn response_for_panic(
        &mut self,
        _err: Box<dyn std::any::Any + Send + 'static>,
    ) -> hyper::Response<Self::ResponseBody> {
        let template = crate::content::templates::panic_error_template();

        let resp = Response::builder()
            // RA block
            .status(StatusCode::OK)
            .body(template.into());

        resp.expect("Unable to unwrap panic template!")
    }
}
