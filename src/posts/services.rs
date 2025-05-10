use crate::state::AppState;

use super::models::Model as Post;

pub struct PostService {}


impl PostService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_posts_list(&self, state: AppState) -> Vec<Post> {
        let mut posts = Vec::new();
        for i in 0..10 {
            let author = format!("Author {}", i);
            let content = format!("Author {}", i);
            let post = Post {
                id: i,
                author,
                content,
            };
            posts.push(post);
        }
        posts
    }
}


impl Default for PostService {
    fn default() -> Self {
        Self {}
    }
}
