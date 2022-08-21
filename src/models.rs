use deadpool_postgres::Pool;
use serde::{Serialize, Deserialize};
use slog::Logger;
use tokio_pg_mapper_derive::PostgresMapper;

pub struct AppState  {
    pub db_pool: Pool,
    pub log: Logger,
}

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="to_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="to_item")]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub todo_list_ids: i32,
}

#[derive(Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub success: bool,
}