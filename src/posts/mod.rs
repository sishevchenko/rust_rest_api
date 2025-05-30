mod models;
pub use models::{
    Model as PostModel,
    Entity as PostEntity,
    ActiveModel as PostActiveModel,
};

mod routers;
pub use routers::*;

mod services;
pub use services::PostService;
