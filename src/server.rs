use crate::{client::Viacep, error::Error, storage::Sled, Address, Result};
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
    <h1>cepd</h1>
    <p>
        cepd is a small and fast caching proxy-server<br>
        for CEP records (brazilian postalcode).
    </p>

    <p>API: <pre>GET /q/:cep</pre></p>

    <p>
    Example:<br>

    <pre>curl -sf http://localhost:3000/q/01311200 | jq
{
  \"postalcode\": \"01311-200\",
  \"address\": \"Avenida Paulista\",
  \"complement\": \"de 1047 a 1865 - lado ímpar\",
  \"neighborhood\": \"Bela Vista\",
  \"city\": \"São Paulo\",
  \"state_initials\": \"SP\"
}
    </pre>
    </p>

    <p><a target='_blank' href='https://github.com/mvrilo/cepd'>source</a></p>
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
    pub core: crate::Cepd<Viacep, Sled>,
}

impl Ctx {
    pub fn new(core: crate::Cepd<Viacep, Sled>) -> Self {
        Self { core }
    }
}

async fn home() -> Html<&'static str> {
    Html(BANNER)
}

async fn query(Path(postalcode): Path<String>, State(state): State<Ctx>) -> Result<Json<Address>> {
    Ok(Json(state.core.search(&postalcode).await?))
}

pub async fn start(addr: SocketAddr, core: crate::Cepd<Viacep, Sled>) -> Result<()> {
    let state = Ctx::new(core);
    let app = Router::new()
        .route("/", get(home))
        .route("/q/:postalcode", get(query))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .include_headers(true)
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        );
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
