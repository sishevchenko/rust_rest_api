use axum::{
    extract::{State, Json},
    routing::{get, post},
    Router,
};

use crate::state::AppState;

use super::{
    PostModel,
    PostService,
};


pub fn get_post_routers() -> Router<AppState> {
    let router = Router::new()
        .route("/posts", get(posts))
        .route("/post", post(create_post));
    router
}

async fn posts(State(state): State<AppState>) -> String {
    let services = PostService::new();
    serde_json::to_string(&services.get_posts_list(state).await)
        .unwrap_or("Something went wrong.".to_owned())
}

async fn create_post(State(state): State<AppState>, Json(data): Json<PostModel>) -> String {
    let services = PostService::new();
    serde_json::to_string(&services.create_post(state, data).await)
        .unwrap_or("Something went wrong.".to_owned())
}
