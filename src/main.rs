pub use self::error::{Error, Result};
use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use crate::model::ModelController;

mod error;
mod web;
mod model;
mod ctx;

#[tokio::main]
async fn main() -> Result<()> {
    let mc_state = ModelController::new().await?; 

    let routes_apis = web::routes_tickets::routets(mc_state.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api/",routes_apis )
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(mc_state.clone(), web::mw_auth::mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("world!");
    println!("name: => {}", name);
    Html(format!(
        "
        <!doctype html>
        <html>
            <head>
                <title>hello</title>
            </head>
            <body>
                <h1>hello {name}</h1>
            </body>
        </html>
        "
    ))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!(
        "
        <!doctype html>
        <html>
            <head>
                <title>hello</title>
            </head>
            <body>
                <h1>hello {name}</h1>
            </body>
        </html>
        "
    ))
}
