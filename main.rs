
#[macro_use]
extern crate diesel;


use actix_web::{App, Error, HttpRequest, HttpServer, dev::ServiceRequest, get, http, middleware, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_cors::Cors;

//mod errors;
mod handlers;
mod models;
mod schema;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    // HttpServer::new( || 
        
    //     let cors = Cors::default()
    //     Cors::default()
    //     .allow_any_origin()
    //     .send_wildcard()
    //     .allowed_methods(vec!["POST", "PUT", "PATCH","DELETE","GET"])
    //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    //     .allowed_header(http::header::CONTENT_TYPE)
    //     .max_age(3600);
    HttpServer::new(move || {
        let cors = Cors::default()
              //.allowed_origin("localhost:9000")
              .allow_any_origin()
              .allow_any_method()
              .send_wildcard()
              .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        App::new()
        //.wrap(middleware::Logger::default())
        .data(pool.clone())
        .wrap(cors)
 
       .service(handlers::get_users)
       //       .route("/users/get", web::get().to(handlers::get_users))
              .route("/users/get/{id}", web::get().to(handlers::get_user_by_id))
              .route("/users/post", web::post().to(handlers::add_user))
              .route("/users/{id}", web::delete().to(handlers::delete_user))
            // .service(web::scope("/users").configure(apps::users::routes::init_routes))

       })
    .bind("localhost:9000")?
    .run()
    .await
}