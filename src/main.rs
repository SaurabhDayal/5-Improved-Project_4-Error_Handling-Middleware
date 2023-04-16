use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{self};
use sqlx::{postgres::PgPoolOptions};

use model::*;
use service::*;
use error::*;

mod model;
mod service;
mod error;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = Data::new(
        AppState {  
            db:
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Error building a connection pool")}
    );
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(register)
            .service(login)
            .service(create)
            .service(get_todos_by_user_id)
            .service(get_todo_by_todo_id)
            .service(modify_by_todo_id)
            .service(delete_by_todo_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
