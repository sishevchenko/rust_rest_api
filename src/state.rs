use crate::config::Config;
use crate::db::ApiDBConn;


#[derive(Clone, Debug)]
pub struct AppState {
    conf: Config,
    conn: ApiDBConn,
}


impl AppState {
    pub fn new(conf: Config, conn: ApiDBConn) -> Self {
        Self {conf, conn}
    }
    pub fn get_conf(&self) -> &Config {
        &self.conf
    }
    pub fn get_conn(&self) -> &ApiDBConn {
        &self.conn
    }
}
