use std::{
    error::Error,
    process::exit
};

use axum::Router;

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
    let conn = db::ApiDBConn::default();
    let app_state = state::AppState::new(conf, conn);

    let app = Router::new()
        .merge(urls::routers())
        .with_state(app_state.clone());

    let listener = tokio::net::TcpListener::bind(
        app_state
            .get_conf()
            .url()).await
            .unwrap();
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
