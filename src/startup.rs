use std::sync::Arc;

use axum::{routing::{get, post}, serve::Serve, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, pool: PgPool) -> Serve<Router, Router> {
    let pool = Arc::new(pool);
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/subscriptions", post(subscribe))
        .route("/health_check", get(health_check))
        .with_state(pool.clone());

    // run our app with hyper, listening globally on port 3000
    axum::serve(listener, app)
}
