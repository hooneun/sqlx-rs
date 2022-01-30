use sqlx::{MySqlPool};
use axum::{
    Router,
    routing::{get, post},
    AddExtensionLayer,
};
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer};
use handlers::{
    protected,
    non_protected,
    index,
    authorize,
};

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
        .layer(middleware)
}