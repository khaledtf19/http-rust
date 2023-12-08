pub use self::error::{Error, Result};
use crate::{log::log_request, model::ModelController};
use axum::{
    extract::{Path, Query},
    http::{uri, Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use ctx::Ctx;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc_state = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routets(mc_state.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api/", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc_state.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let cline_status_error = service_error.map(|ser| ser.client_status_error());

    let error_res = cline_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
            }
            });
            println!("   ->> client_error_body: {client_error_body}");
            (*status_code, Json(client_error_body)).into_response()
        });
    let client_error = cline_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await.unwrap();

    println!();
    error_res.unwrap_or(res)
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
