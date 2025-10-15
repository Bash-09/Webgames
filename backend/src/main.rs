use clap::Parser;
use config::Config;

mod api;
mod config;
mod db;
mod http;
mod site;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenv::dotenv().ok();

    let config = Config::parse();

    let db = db::create_connection(&config)
        .await
        .map(|db| db.expect("Couldn't connect to database."));

    http::serve(config, db)
        .await
        .expect("Couldn't start server.");
}
