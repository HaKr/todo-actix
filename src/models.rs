use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use slog;
use tokio_pg_mapper_derive::PostgresMapper;

use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::config::Config;


#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub log: slog::Logger,
}

impl Default for AppState {
    fn default() -> Self { 
        dotenv().ok();

        let config = Config::from_env().unwrap();
    
        let pool = config.pg.create_pool(None, NoTls).unwrap();
    
        let log = Config::configure_log();
    
        return AppState {
            pool: pool.clone(),
            log: log.clone(),
        };
    
     }
}

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}
