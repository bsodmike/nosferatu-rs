use crate::content::templates::{AboutTemplate, HtmlTemplate, IndexTemplate};
use axum::response::IntoResponse;

pub async fn render_index() -> impl IntoResponse {
    let template = IndexTemplate {};

    HtmlTemplate(template)
}

pub async fn render_about() -> impl IntoResponse {
    let template = AboutTemplate {};

    HtmlTemplate(template)
}
