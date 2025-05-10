use axum::Router;

use crate::{posts::get_post_routers, state::AppState};


pub fn routers() -> Router<AppState> {
    let routers = Router::new()
        .merge(get_post_routers());
    routers
}
