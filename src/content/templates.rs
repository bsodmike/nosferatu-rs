use crate::utils;
use crate::utils::logger;
use askama::Template;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use i18n::I18nBundle;
use std::sync::LazyLock;
use std::sync::Mutex;

pub mod i18n;

pub static I18N_STATIC_CONTENT: LazyLock<Mutex<Option<I18nBundle>>> =
    LazyLock::new(|| Mutex::new(Some(I18nBundle::new())));
pub static I18N_LANGUAGE: LazyLock<Mutex<Option<&str>>> = LazyLock::new(|| Mutex::new(Some("en")));

pub(crate) struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Response::builder()
                .body(axum::body::Body::from(html))
                .unwrap(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from(format!(
                    "Failed to render template. Error: {}",
                    err
                )))
                .unwrap(),
        }
    }
}

pub fn translate(key: &str) -> String {
    let i18n_content = &mut *I18N_STATIC_CONTENT.lock().unwrap();

    if let Some(i18n) = i18n_content {
        let i18n_lang = &mut *I18N_LANGUAGE.lock().unwrap();

        if let Some(lang) = i18n_lang {
            let bundle = i18n.fetch_bundle(lang);

            bundle.get(key).unwrap_or(&String::default()).to_string()
        } else {
            translation_err(
                "translation_language_missing",
                "content::templates::translate",
            )
        }
    } else {
        translation_err("translation_missing", "content::templates::translate")
    }
}

fn translation_err(err: &str, tag: &str) -> String {
    logger::log(
        logger::Level::Error,
        logger::Color(utils::RED),
        logger::Tag(tag),
        logger::Text(format!("{}", err).as_str()),
    );

    err.to_string()
}

pub fn render_nav() -> String {
    let nav = NavTemplate {};

    match nav.render() {
        Ok(html) => html,
        Err(err) => format!("Failed to render nav template. Error: {}", err),
    }
}

// Index: Homepage
#[derive(Template, Clone)]
#[template(path = "nav.html", escape = "none")]
pub struct NavTemplate {}

// Index: Homepage
#[derive(Template, Clone)]
#[template(path = "index.html", escape = "none")]
pub struct IndexTemplate {}

// About
#[derive(Template)]
#[template(path = "about.html", escape = "none")]
pub struct AboutTemplate {}

// Panic Error Template
#[derive(Template)]
#[template(path = "panic.html", escape = "none")]
pub(crate) struct PanicErrorTemplate {}

pub fn panic_error_template() -> String {
    let template = PanicErrorTemplate {};

    template.render().unwrap()
}

// 404 Error Template
#[derive(Template)]
#[template(path = "error_404.html", escape = "none")]
pub(crate) struct Error404Template {}

pub async fn error_404_template() -> impl IntoResponse {
    let template = Error404Template {};

    HtmlTemplate(template)
}
