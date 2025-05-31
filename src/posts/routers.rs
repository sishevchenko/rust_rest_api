use axum::{
    extract::{Json, Query, State},
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
        .route("/posts", get(posts_list))
        .route("/post", post(create_post));
    router
}


async fn posts_list(
    State(state): State<AppState>,
    Query(page): Query<u64>,
    Query(page_size): Query<u64>,
) -> String {
    let services = PostService::new();
    serde_json::to_string(
        &services.get_posts_list(state, page, page_size).await
    ).unwrap_or("Something went wrong.".to_owned())
}


async fn create_post(
    State(state): State<AppState>,
    Json(data): Json<PostModel>,
) -> String {
    let services = PostService::new();
    serde_json::to_string(&services.create_post(state, data).await)
        .unwrap_or("Something went wrong.".to_owned())
}
