#![allow(unused_imports)]

use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool, PoolError};
use dotenv::dotenv;
use slog::{crit, error, o, Logger};
use tokio_postgres::NoTls;

use crate::db::{self, *};
use crate::models::{AppState, Status};

use crate::config::Config;
use crate::errors::AppError;

async fn get_client(log: &Logger, pool: &Pool) -> Result<Client, AppError> {
    pool.get().await.map_err(|err: PoolError| {
        let sublog = log.new(o!("cause" => err.to_string()));
        error!(sublog, "Error creating client");
        AppError::from(err)
    })
}

fn log_error(log: Logger) -> impl Fn(AppError) -> AppError {
    move |err| {
        error!(log, "Error! {}", err.message());
        err
    }
}

pub async fn status() -> Result<impl Responder, AppError> {
    Ok(web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    }))
}

pub async fn items(state: AppState) -> Result<Vec<TodoItem>, AppError> {
    let (log, pool) = (state.log.clone(), state.pool);

    let client: Client = get_client(&log, &pool).await?;

    let result = db::get_items(&client, 1).await;

    result
        // .map(|todos| HttpResponse::Ok().json(todos))
        // .map(|todos| todos)
        .map_err(log_error(log))
}
