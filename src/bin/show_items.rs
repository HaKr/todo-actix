use slog::{error, info, Logger};

use todo_actix::{errors::AppError, handlers::*, models::AppState};

fn log_error(log: Logger, err: AppError) -> AppError {
    error!(log, "Error! {}", err.message());
    err
}

#[actix_rt::main]
async fn main() {
    let state = AppState::default();

    info!(state.log, "Getting to-do items");
    let results = items(state.clone()).await;

    match results {
        Ok(results) => {
            info!(state.log, "Displaying {} unchecked items", results.len());
            for item in results {
                info!(state.log, "{}", item.title);
            }
        }
        Err(err) => {
            log_error(state.log, err);
        }
    }
}
