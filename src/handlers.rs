use crate::{ models::{Status, CreateTodoList, ResultResponse, AppState, }, db, errors::AppError};
use actix_web::{ Responder, HttpResponse, web, get, post, put,  };
use deadpool_postgres::{ Client, Pool, };
use slog::{o, crit, Logger, error};


// DB Pool Connection Management and error handling
async fn get_client(db_pool: Pool, logger: Logger) -> Result<Client, AppError> {
    db_pool.get().await.map_err( |err| {

        let sub_log = logger.new(o!("cause" => err.to_string()));
        crit!(sub_log, "Error creating client");
        AppError::db_error(err)
    })
}

// Wichtig das hier wird ein closure zurück gegeben, der wenn es soweit ist mit map_err( |err| ...) aufgerufen wird.
fn log_error(logger: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move | err|  {
        let sub_log = logger.new(o!("cause" => err.cause.clone()));
        error!(sub_log, "{}", err.message());
        err
    })
}



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

#[get("/todos/{list_id}/items")]
pub async fn show(app_state: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<impl Responder, AppError> {
    let log = app_state.log.new(o!("handler.rs" => "#show '/todos/{list_id}/items'"));
    let client: Client = get_client(app_state.db_pool.clone(), log.clone() ).await?;


    let result = db::get_items(&client, path.0).await;
    result
        .map(|todo_items| HttpResponse::Ok().json(todo_items))
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

#[put("/todos/{list_id}/items/{item_id}")]
pub async fn check_item(app_state: web::Data<AppState>, path: web::Path<(i32, i32)>) -> Result<impl Responder, AppError> {
    let log = app_state.log.new(o!("handler.rs" => "#check_items '/todos/{list_id}/items/{item_id}'"));
    let client: Client = get_client(app_state.db_pool.clone(), log.clone() ).await?;


    let result = db::check_todo(&client, path.0, path.1).await;
    result
        .map(|updated| HttpResponse::Ok().json( ResultResponse { success: updated }))
        .map_err(log_error(log))
}

// https://www.youtube.com/watch?v=B0fL3WmJZsc muss ich noch lernen