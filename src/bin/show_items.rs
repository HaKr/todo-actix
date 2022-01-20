extern crate diesel;
extern crate todo_actix;
use slog::info;

use todo_actix::{handlers::*, models::AppState};

#[actix_rt::main]
async fn main() {

    let state = AppState::default();

    info!(state.log, "Getting to-do items");

    if let Ok(results) = items(state.clone()).await {
        info!(state.log, "Displaying {} unchecked items", results.len());
        for item in results {
            info!(state.log, "{}", item.title);
        }
    }
}
