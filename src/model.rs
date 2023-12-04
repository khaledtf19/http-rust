use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    pub tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
    pub async fn create_ticket(&self, tickit_fc: TicketForCreate) -> Result<Ticket>{
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;

        let tickit = Ticket {
            id,
            title: tickit_fc.title
        };
        store.push(Some(tickit.clone()));

        Ok(tickit)

    }
    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }
    pub async fn delete_ticket(&self, id: u64) ->Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeletFailIdNotFound { id })
    }
}

