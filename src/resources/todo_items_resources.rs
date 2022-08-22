use crate::{ models::{ AppState, ResultResponse, }, db, errors::AppError, resources::{get_client, log_error}};
use actix_web::{ Responder, HttpResponse, web, get, put };
use deadpool_postgres::{ Client, };
use slog::{o,};

#[get("/todos/{list_id}/items")]
pub async fn index(app_state: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<impl Responder, AppError> {
    let log = app_state.log.new(o!("handler.rs" => "#show '/todos/{list_id}/items'"));
    let client: Client = get_client(app_state.db_pool.clone(), log.clone() ).await?;


    let result = db::get_items(&client, path.0).await;
    result
        .map(|todo_items| HttpResponse::Ok().json(todo_items))
        .map_err(log_error(log))
}

#[put("/todos/{list_id}/items/{item_id}")]
pub async fn check_item(app_state: web::Data<AppState>, path: web::Path<(i32, i32)>) -> Result<impl Responder, AppError> {
    let log = app_state.log.new(o!("handler.rs" => "#check_items '/todos/{list_id}/items/{item_id}'"));
    let client: Client = get_client(app_state.db_pool.clone(), log.clone() ).await?;


    let result = db::check_todo(&client, path.0, path.1).await;
    result
        .map(|updated| HttpResponse::Ok().json( ResultResponse { success: updated }))
        .map_err(log_error(log))
}
