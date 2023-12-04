
use axum::{
    extract::{Path, State, FromRef},
    routing::{delete, post},
    Json, Router,
};

use crate::{
    model::{ModelController, Ticket, TicketForCreate},
    Result,
};

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController 
}

pub fn routets(mc: ModelController) -> Router {
    let app_state = AppState{mc};
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(app_state)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}

