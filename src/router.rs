use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};
use handlers::{authorize, index, non_protected, protected, unauth_protected};
use sqlx::MySqlPool;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::handlers;

pub fn router(pg_pool: MySqlPool) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(pg_pool.to_owned()))
        .into_inner();

    Router::new()
        .route("/", get(index))
        .route("/authorize", post(authorize))
        .route("/protected", get(protected))
        .route("/non_protected", get(non_protected))
        .route("/unauth_protected", get(unauth_protected))
        .layer(middleware)
}
