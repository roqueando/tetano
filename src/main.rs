use actix_web::{App, HttpServer};

pub mod handlers;
pub mod config;
pub mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let _conn = config::database::connect_database();

    println!("Server running on port 8000");
    HttpServer::new(|| {
        App::new()
            .configure(router::build)
    })
    .bind("0.0.0.0:8000")?
        .run()
        .await
}
