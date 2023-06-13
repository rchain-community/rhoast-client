use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;
use std::env;

pub mod cli;
mod config;
mod handlers;
pub mod rholang;
pub mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = cli::Args::new();
    match args.select {
        types::Select::CLI => {
            if args.utility.is_none() {
                panic!("select a utility type")
            }
            match args.utility.unwrap() {
                types::Type::SimpleDeploy => args.parse_simple_deploy(),
            }
            Ok(())
        }
        types::Select::HTTP => run_http().await,
    }
}

async fn run_http() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    //set env variables
    let host = env::var("HOST").expect("Please set host in .env");
    let port = env::var("PORT").expect("Please set port in .env");

    println!("running on host {} on port {}", host, port);
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .configure(config::routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
