use crate::{error::Error, Address, Result};
use axum::{
    extract::{Path, State},
    response::{Html, Response},
    routing::get,
    Json, Router,
};
use reqwest::StatusCode;
use serde_json::json;
use std::net::SocketAddr;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

const BANNER: &'static str = "<html>
<head>
    <title>cepd - https://github.com/mvrilo/cepd</title>
    <style>*{font-family:courier}</style>
</head>
<body>
    <p>this is a fast and small proxy-server with caching capabilities for fetch CEP (brazilian zipcode) information.</p>
</body>
</html>";

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            // Error::Internal => _,
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[derive(Debug, Clone)]
struct Ctx {
    pub core: crate::Cepd,
}

impl Ctx {
    pub fn new(core: crate::Cepd) -> Self {
        Self { core }
    }
}

async fn handler() -> Html<&'static str> {
    Html(BANNER)
}

async fn query(Path(zipcode): Path<String>, State(state): State<Ctx>) -> Result<Json<Address>> {
    let input = &zipcode.into_bytes().to_vec();
    let addr = state.core.search(input).await?;
    Ok(Json(addr))
}

pub async fn start(addr: SocketAddr, core: crate::Cepd) -> Result<()> {
    let state = Ctx::new(core);
    let app = Router::new()
        .route("/", get(handler))
        .route("/q/:c", get(query))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(
                    DefaultOnResponse::new()
                        .include_headers(true)
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        );
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
