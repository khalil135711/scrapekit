//! Lokale Web-UI für scrapekit: `cargo run --bin scrapekit-ui`, dann http://127.0.0.1:3000 öffnen.

use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use scrapekit::{extract, fetch};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize)]
struct ScrapeRequest {
    url: String,
    selector: String,
    attr: Option<String>,
}

#[derive(Serialize)]
struct ScrapeResponse {
    results: Vec<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

type ApiError = (StatusCode, Json<ErrorResponse>);

fn bad_request(msg: String) -> ApiError {
    (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: msg }))
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../../ui/index.html"))
}

async fn scrape(Json(req): Json<ScrapeRequest>) -> Result<Json<ScrapeResponse>, ApiError> {
    let base = Url::parse(&req.url).map_err(|e| bad_request(format!("Ungültige URL: {e}")))?;
    if !matches!(base.scheme(), "http" | "https") {
        return Err(bad_request("Nur http und https sind erlaubt".into()));
    }
    let attr = req.attr.filter(|a| !a.trim().is_empty());
    let selector = req.selector;

    // reqwest::blocking darf nicht direkt im async-Kontext laufen.
    let outcome = tokio::task::spawn_blocking(move || -> anyhow::Result<Vec<String>> {
        let html = fetch(base.as_str(), 10)?;
        extract(&html, &selector, attr.as_deref(), Some(&base))
    })
    .await
    .map_err(|e| bad_request(format!("Interner Fehler: {e}")))?;

    match outcome {
        Ok(results) => Ok(Json(ScrapeResponse { results })),
        Err(e) => Err(bad_request(format!("{e:#}"))),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/scrape", post(scrape));

    // Nur lokal erreichbar: die UI lädt beliebige URLs und gehört nicht ins offene Netz.
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Konnte Port 3000 nicht binden (schon belegt?)");
    println!("scrapekit UI läuft auf http://{addr}");
    axum::serve(listener, app).await.expect("Server-Fehler");
}
