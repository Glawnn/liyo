pub mod models;
pub mod routes;

pub fn router() -> axum::Router {
    axum::Router::new().merge(routes::health::router())
}
