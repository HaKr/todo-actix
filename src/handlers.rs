#![allow(unused_imports)]

use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool, PoolError};
use slog::{crit, error, o, Logger};

use crate::db::{self, *};
use crate::models::{AppState, Status};

use crate::errors::AppError;

async fn get_client(log: &Logger, pool: &Pool) -> Result<Client, AppError> {
    pool.get().await.map_err(|err: PoolError| {
        let sublog = log.new(o!("cause" => err.to_string()));
        error!(sublog, "Error creating client");
        AppError::from(err)
    })
}

pub async fn status() -> Result<impl Responder, AppError> {
    Ok(web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    }))
}

pub async fn items(state: AppState) -> Result<Vec<TodoItem>, AppError> {
    let (log, pool) = (state.log.clone(), state.pool);

    let client: Client = get_client(&log, &pool).await?;

    db::get_items(&client, 1).await
}
