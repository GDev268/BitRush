use actix_web::{web, HttpResponse, Responder};
use rand::Rng;
use tokio_postgres::{Client, Error, NoTls};

use crate::classes::{self, JsonUser};

pub async fn check_id_collision(db: &web::Data<Client>, id: u64) -> bool {
    let query = "SELECT EXISTS(SELECT true FROM bit_user WHERE user_id = $1)";
    let row = db.query_one(query, &[&(id as i64)]).await.unwrap();

    return row.get(0);
}

pub async fn create_user(user: web::Json<JsonUser>, db: web::Data<Client>) -> impl Responder {
    let user_id = generate_user_id(&db).await;
    let json_user = user.into_inner();

    let user = classes::User::create_user(json_user, user_id);

    let query = "INSERT INTO bit_user (user_id,username,password,email) VALUES ($1, $2, $3, $4)";
    db
        .execute(
            query,
            &[
                &(user.user_id as i64), 
                &user.username,
                &user.password,
                &user.email,
            ],
        )
        .await
        .unwrap();

    HttpResponse::Created().json(user)
}

pub async fn generate_user_id(db: &web::Data<Client>) -> u64 {
    let mut rng = rand::thread_rng();
    let mut user_id: u64 = rng.gen_range(0..9223372036854775807);

    while check_id_collision(&db, user_id).await {
        user_id = rng.gen();
    }

    return user_id;
}

/*pub async fn get_user(path: web::Path<(i32)>, db: web::Data<Client>) {
    let user_id = path.into_inner().0;
    let query = "SELECT id, name, email FROM users WHERE id = $1";
    let row = db.query_one(query, &[&user_id]).await.unwrap();

    let user = User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    };

    HttpResponse::Ok().json(user)
}*/
