use std::{env, io};

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

pub async fn get1() -> impl Responder {
    format!("hello from get result!")
}

pub async fn get2() -> impl Responder {
    HttpResponse::Ok().body("rendered!")
}

pub async fn get_p() -> impl Responder {
    HttpResponse::Ok().body("get_p!")
}

pub async fn post_p() -> impl Responder {
    HttpResponse::Ok().body("post_p!")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub sys: DateTime<Utc>,
    pub local: DateTime<Local>,
}

pub async fn get_u() -> Result<HttpResponse, Error> {
    let user = User {
        id: 11,
        first_name: String::from("sam"),
        last_name: String::from("sam"),
        email: String::from("someone@example.com"),
        created_at: Utc::now().naive_utc(),
        sys: Utc::now(),
        local: Local::now(),
    };

    Ok(HttpResponse::Ok().json(user))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(get_u))
            .route("/1", web::get().to(get1))
            .route("/2", web::get().to(get2))
            .service(
                web::resource("/p")
                    .route(web::get().to(get_p))
                    .route(web::post().to(post_p)),
            )
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
