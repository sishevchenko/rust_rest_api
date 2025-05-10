use axum::{
    extract::State, routing::get, Router
};

use crate::state::AppState;

use super::services::PostService;



pub fn get_post_routers() -> Router<AppState> {
    let router = Router::new()
        .route("/posts", get(posts));
    router
}

async fn posts(State(state): State<AppState>) -> String {
    let services = PostService::new();
    serde_json::to_string(&services.get_posts_list(state).await).unwrap()
}
