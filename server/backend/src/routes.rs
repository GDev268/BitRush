use actix_web::{web, HttpResponse, HttpServer, Responder};
use tokio_postgres::Client;

use crate::{classes::JsonUser, database};

pub async fn create_user(user: web::Json<JsonUser>, db: web::Data<Client>) -> impl Responder {
    let user_json = user.clone();

    database::create_user(user, db).await;

    HttpResponse::Created().json(user_json)
}
