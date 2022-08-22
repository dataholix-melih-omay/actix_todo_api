use crate::{ models::{Status, CreateTodoList, AppState, }, db, errors::AppError, resources::{get_client, log_error}};
use actix_web::{ Responder, HttpResponse, web, get, post, };
use deadpool_postgres::{ Client};
use slog::o;


#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
    .json( Status { status: "Ok".to_string() })
}


#[get("/todos")]
pub async fn index(app_state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let log = app_state.log.new(o!("handler.rs" => "index '/todos'"));
    let client: Client = get_client(app_state.db_pool.clone(), log.clone() ).await?;

    let result = db::get_todos(&client).await;

    result
        .map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}

#[post("/todos")]
pub async fn create(app_state: web::Data<AppState>, json: web::Json<CreateTodoList>) -> Result<impl Responder, AppError> {
    let log = app_state.log.new(o!("handler.rs" => "#create '/todos'"));
    let client: Client = get_client(app_state.db_pool.clone(), log.clone() ).await?;


    let result = db::create(&client, json.title.clone()).await;
    result
        .map(|new_todo| HttpResponse::Ok().json(new_todo))
        .map_err(log_error(log))
}


// https://www.youtube.com/watch?v=B0fL3WmJZsc muss ich noch lernen