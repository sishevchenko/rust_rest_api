use std::{
    error::Error,
    process::exit
};

use axum::Router;
use migration::{Migrator, MigratorTrait};

mod config;
use config::Config;

mod urls;
mod posts;
mod db;
mod state;


#[tokio::main]
async fn start() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let conf = Config::default();
    let mut db = db::ApiDB::default();

    Migrator::up(
        &db.get_conn().await
        .expect("Database connection failed during migration script."), None).await
        .expect("Failed migration script.");

    let app_state = state::AppState::new(conf, db);

    let app = Router::new()
        .merge(urls::routers())
        .with_state(app_state.clone());

    let listener = tokio::net::TcpListener::bind(
        app_state
            .get_conf()
            .url()).await
            .expect("Error in setting up TCP listener.");
    axum::serve(listener, app).await?;
    Ok(())
}

fn main() {
    let result = start();

    match result.err() {
        Some(err) => {
            println!("Error: {err}");
            exit(1);
        },
        _ => (),
    }
    exit(0)
}
