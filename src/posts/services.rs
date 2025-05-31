use sea_orm::*;
use crate::state::AppState;

use super::{
    models as posts,
    PostEntity,
    PostModel,
    PostActiveModel,
};

pub struct PostService {}


impl PostService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_posts_list(
        &self,
        state: AppState,
        page: u64,
        page_size: u64,
    ) -> Vec<PostModel> {
        let db = &state
            .get_db()
            .get_conn().await
            .expect("Database connection failed.");

        let posts = PostEntity::find()
            .order_by_asc(posts::Column::Id)
            .paginate(db, page_size)
            .fetch_page(page - 1).await
            .unwrap();
        posts
    }

    pub async fn create_post(
        &self,
        state: AppState,
        data: PostModel,
    ) -> PostModel {
        let db = &state
            .get_db()
            .get_conn().await
            .expect("Database connection failed.");
        let new_post = PostActiveModel {
            author: Set(data.author.to_owned()),
            content: Set(data.content.to_owned()),
            ..Default::default()
        }.save(db).await
            .inspect_err(|err| { eprintln!("DbErr: {err}") })
            .unwrap()
            .try_into_model()
            .unwrap();
        new_post
    }
}


impl Default for PostService {
    fn default() -> Self {
        Self {}
    }
}
