use crate::config::Config;
use anyhow::Context;
use axum::Router;
use sqlx::PgPool;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

/// Define the error type that will be used for all request handlers.
mod error;
mod types;
mod extractor;
mod api;

pub use error::{Error};

use tower_http::trace::TraceLayer;


pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let api_context = extractor::ApiContext {
        config: Arc::new(config),
        db,
    };

    // create a router using the function below, pass in the api_context to all routes
    let app = api_router(api_context);
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Error starting HTTP server")
}

fn api_router(api_context: extractor::ApiContext) -> Router {
    Router::new()
        .merge(api::videos::router())
        // Enables logging. Use `RUST_LOG=tower_http=debug`
        .layer(TraceLayer::new_for_http())
        .with_state(api_context)
}