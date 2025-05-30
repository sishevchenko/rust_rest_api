use crate::config::Config;
use crate::db::ApiDB;


#[derive(Clone, Debug)]
pub struct AppState {
    conf: Config,
    db: ApiDB,
}


impl AppState {
    pub fn new(conf: Config, db: ApiDB) -> Self {
        Self {conf, db}
    }
    pub fn get_conf(&self) -> &Config {
        &self.conf
    }
    pub fn get_db(&self) -> ApiDB {
        self.db.clone()
    }
}
