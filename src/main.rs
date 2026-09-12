mod components;
mod delivery;
mod i18n;
mod routes;
mod seo;
mod validation;

use axum::{
    routing::{get, post},
    Router,
};
use std::{env, net::SocketAddr};
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG")
                .unwrap_or_else(|_| "smart_dawn_web=debug,tower_http=info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/en", get(routes::index_en))
        .route("/cz", get(routes::index_cz))
        .route("/de", get(routes::index_de))
        .route("/contact", post(routes::contact))
        .route("/sitemap.xml", get(routes::sitemap))
        .route("/robots.txt", get(routes::robots_txt))
        .route("/healthz", get(routes::healthz))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(get(routes::not_found))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let port = env::var("PORT")
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("Smart Dawn web listening at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    axum::serve(listener, app).await.expect("server error");
}
