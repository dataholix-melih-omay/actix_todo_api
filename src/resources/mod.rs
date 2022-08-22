use deadpool_postgres::{Pool, Client};
use slog::{crit, error, Logger, o};
use crate::errors::AppError;

pub mod health_resource;
pub mod todo_resource;
pub mod todo_items_resources;



// Wichtig das hier wird ein closure zurück gegeben, der wenn es soweit ist mit map_err( |err| ...) aufgerufen wird.
pub fn log_error(logger: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move | err|  {
        let sub_log = logger.new(o!("cause" => err.cause.clone()));
        error!(sub_log, "{}", err.message());
        err
    })
}

// DB Pool Connection Management and error handling
async fn get_client(db_pool: Pool, logger: Logger) -> Result<Client, AppError> {
    db_pool.get().await.map_err( |err| {

        let sub_log = logger.new(o!("cause" => err.to_string()));
        crit!(sub_log, "Error creating client");
        AppError::db_error(err)
    })
}
