use actix_web::http::header::ContentType;
use actix_web::{web};
use actix_web::{delete, get, post, put, Responder};
use actix_web::{web::Data, HttpResponse};
use hyper::Headers;
use hyper::header::{Authorization, Bearer};
use rand::distributions::{Alphanumeric, DistString};
use sqlx::{self};
use pwhash::bcrypt;

use crate::error::MyError;
use crate::model::*;

#[post("/register")]
pub async fn register(state: Data<AppState>, user: web::Json<Users>) -> Result<impl Responder, MyError> {

    let user =user.into_inner();
    let h_pwd =bcrypt::hash(&user.user_password.to_string()).map_err(|_| MyError::InternalError)?;

    let row = sqlx::query_as!(Users,
        "INSERT INTO users (user_name, user_password, user_profession) VALUES ($1, $2, $3) 
        RETURNING user_id, user_name, user_password, user_profession",
        user.user_name, h_pwd, user.user_profession
    )
    .fetch_one(&state.db)
    .await?;

    Ok(actix_web::web::Json(row))
}

#[post("/login")]
async fn login(state: Data<AppState>, user: web::Json<Users>) -> Result<impl Responder, MyError> {
    let user = user.into_inner();

    let table_user = sqlx::query_as!(Users, "select user_id, user_name, user_password, user_profession from users where user_name =$1",
         user.user_name)
    .fetch_one(&state.db).await?;


    if bcrypt::verify(user.user_password.to_owned(), &table_user.user_password ){
        let user_token = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let mut headers = Headers::new();
        headers.set(
            Authorization(
                Bearer {
                   token: user_token.to_owned()
               }
            )
        );

        sqlx::query_as!(Auths,"Insert into auths (user_id, user_token) VALUES ($1, $2)", user.user_id, user_token)
        .fetch_one(&state.db)
        .await?;

        Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("Authorization", user_token))
        .body(serde_json::to_string(&table_user).unwrap()))
                
    } else {
        Err(MyError::UnAuthorized)
    }

}
            
            
#[post("/todo")]
async fn create(state: Data<AppState>, todo: web::Json<Todos>, usr:Users) -> Result<impl Responder, MyError> {

    let b_id = usr.user_id;

    let row = sqlx::query_as!( Todos,
        "INSERT INTO todos (user_id, description, todo_date) VALUES ($1, $2, $3) 
        RETURNING todo_id, user_id, description, todo_date",
        b_id, todo.description, todo.todo_date
    )
    .fetch_one(&state.db)
    .await?;

    Ok(HttpResponse::Ok().json(row))
    
}

#[get("/todouser")]
async fn get_todos_by_user_id(state: Data<AppState>, usr: Users) -> Result<impl Responder, MyError> {

    let b_user_id = usr.user_id;

    let rows = sqlx::query_as!(Todos,"SELECT todo_id, description, todo_date, user_id FROM todos WHERE user_id=$1", b_user_id)
        .fetch_all(&state.db)
        .await?;
    Ok(HttpResponse::Ok().json(rows))
}

#[get("/todo/{id}")]
async fn get_todo_by_todo_id(state: Data<AppState>, id: web::Path<i32>, usr: Users) -> Result<impl Responder, MyError> {

    let id=id.into_inner();
    let b_user_id = usr.user_id;

    let req_row = sqlx::query_as!(Todos, "SELECT todo_id, description, todo_date, user_id FROM todos WHERE todo_id=$1", id)
        .fetch_one(&state.db)
        .await?;

    let req_id = req_row.user_id;

    if req_id==b_user_id {
        Ok(HttpResponse::Ok().json(req_row))
    } else {
        Err(MyError::UnAuthorized)
    }
}

#[put("todo/{id}")]
async fn modify_by_todo_id( state: Data<AppState>, id: web::Path<i32>, todo: web::Json<Todos>, usr: Users) -> Result<impl Responder, MyError> {

    let id=id.into_inner();
    let b_user_id = usr.user_id;
    let todo = todo.into_inner();

    let req_row = sqlx::query_as!(Todos, "SELECT todo_id, description, todo_date, user_id FROM todos WHERE todo_id=$1", id)
        .fetch_one(&state.db)
        .await?;

    let req_id = req_row.user_id;

    if req_id==b_user_id {
        let row = sqlx::query_as!( Todos, "UPDATE todos SET description=$1, todo_date=$2 WHERE
        todo_id=$3 RETURNING todo_id, user_id, description, todo_date", 
        &todo.description, &todo.todo_date, id
        )
        .fetch_one(&state.db)
        .await?;

        Ok(HttpResponse::Ok().json(row))
    }else{
        Err(MyError::UnAuthorized) 
    }
}

#[delete("todo/{id}")]
async fn delete_by_todo_id(state: Data<AppState>, id: web::Path<i32>, usr: Users) -> Result<impl Responder, MyError> {

    let id=id.into_inner();
    let b_user_id = usr.user_id;

    let req_row = sqlx::query_as!(Todos, "SELECT todo_id, description, todo_date, user_id FROM todos WHERE todo_id=$1", id)
        .fetch_one(&state.db)
        .await?;

    let req_id = req_row.user_id;
    (req_id == b_user_id).then(|| true).ok_or(MyError::UnAuthorized)?;
    let row = sqlx::query_as!(Todos,"DELETE FROM todos WHERE todo_id=$1 RETURNING todo_id, user_id, description, todo_date", id)
        .fetch_one(&state.db)
        .await?;
    Ok(actix_web::web::Json(row))
}
