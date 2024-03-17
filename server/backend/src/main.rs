use actix_web::{
    web, App, FromRequest, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
use classes::JsonUser;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, NoTls, Row};

mod classes;
mod database;
mod routes;

// Define a User struct to represent the data model
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// Define handlers for CRUD operations
/*async fn create_user(user: web::Json<User>, db: web::Data<Client>) -> impl Responder {
    let user = user.into_inner();
    let query = "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email";
    let row = db
        .query_one(query, &[&user.name, &user.email])
        .await
        .unwrap();

    let new_user = User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    };

    HttpResponse::Created().json(new_user)
}


async fn get_user(path: web::Path<(i32,)>, db: web::Data<Client>) -> impl Responder {
    let user_id = path.into_inner().0;
    let query = "SELECT id, name, email FROM users WHERE id = $1";
    let row = db.query_one(query, &[&user_id]).await.unwrap();

    let user = User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    };

    HttpResponse::Ok().json(user)
}

async fn update_user(
    path: web::Path<(i32,)>,
    user: web::Json<User>,
    db: web::Data<Client>,
) -> impl Responder {
    let user_id = path.into_inner().0;
    let user = user.into_inner();
    let query = "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email";
    let row = db
        .query_one(query, &[&user.name, &user.email, &user_id])
        .await
        .unwrap();

    let updated_user = User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    };

    HttpResponse::Ok().json(updated_user)
}

async fn delete_user(path: web::Path<(i32,)>, db: web::Data<Client>) -> impl Responder {
    let user_id = path.into_inner().0;
    let query = "DELETE FROM users WHERE id = $1 RETURNING id, name, email";
    let row = db.query_one(query, &[&user_id]).await.unwrap();

    let deleted_user = User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    };

    HttpResponse::Ok().json(deleted_user)
}*/

async fn test_rest_api(db: web::Data<Client>) -> impl Responder {
    let query = "SELECT * FROM bit_user";
    let rows = db.query(query, &[]).await.unwrap();

    let value = rows.len();

    HttpResponse::Ok().json(value)
}

async fn create_user_handler(user: web::Json<JsonUser>, db: web::Data<Client>) -> impl Responder {
    //let user = extract_user_from_request(req).await.expect("Failed to extract user from request");
    HttpResponse::Ok().json(1)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=root dbname=bitrush password=root",
        NoTls,
    )
    .await.expect("Failed to initialize database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("error connecting to database: {}", e);
        }
    });

    let client_data = web::Data::new(client);

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(client_data.clone())
            .route("/test", web::get().to(test_rest_api))
            .route(
                "/create",
                web::post().to(routes::create_user),
            )
    })
    .bind("localhost:9090")?
    .run()
    .await
}
