#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use actix_web::web::Data;
use dotenv::dotenv;
use std::collections::btree_map::Values;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::StatusCode;

use self::models::Post;
use self::schema::posts;
use self::schema::posts::dsl::*;

pub type dbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(pool: web::Data<dbPool>) -> impl Responder {
    let mut conn = pool.get().expect("No se puede no hay tortillas");

    match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => {

            return HttpResponse::Ok().body(format!("{:?}", data));

        },
        Err(err) => HttpResponse::Ok().body("Hubo un error"),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Variable de entorno 'DATABASE_URL' no encontrada");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder().build(connection).expect("No hay coneccion");

    HttpServer::new(move || {
        App::new().service(index).data(pool.clone())
    })
    .bind("0.0.0.0:9900")?
    .run()
    .await
}
