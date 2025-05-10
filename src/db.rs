use sea_orm::{
    Database,
    DatabaseConnection, DbErr,
};


#[derive(Debug)]
enum SupportsDBType {
    PostgreSql=5432,
    MySql=3306,
    Sqlite,
}

impl From<&String> for SupportsDBType {
    fn from(host: &String) -> Self {
        if host.starts_with("mysql://") {
            return Self::MySql;
        } else if host.starts_with("postgres://") || host.starts_with("postgresql://") {
            return Self::PostgreSql;
        } else if host.starts_with("sqlite://") {
            return Self::Sqlite;
        } else {
            let msg = format!("Unsupport DataBase type for host: {}", &host);
            panic!("{}", msg)
        }
    }
}


#[derive(Clone, Debug)]
pub struct ApiDBConn {
    host: String,
    pool: Option<DatabaseConnection>,
    port: Option<u16>,
}


impl ApiDBConn {
    pub fn new(host: String, port: Option<u16>) -> Self {
        Self {
            host,
            port,
            pool: None,
        }
    }

    pub async fn get_conn(&mut self) -> Result<DatabaseConnection, DbErr> {
        if !self.pool.is_none() {
            return Ok(self.pool.clone().unwrap());
        }
        let pool = match SupportsDBType::from(&self.host) {
            SupportsDBType::PostgreSql => {
                let port = self.port.clone().unwrap_or(SupportsDBType::PostgreSql as u16);
                let url = format!("{}:{}", &self.host, port);
                Database::connect(url).await?
            },
            SupportsDBType::MySql => {
                let port = self.port.clone().unwrap_or(SupportsDBType::MySql as u16);
                let url = format!("{}:{}", &self.host, port);
                Database::connect(url).await?
            },
            SupportsDBType::Sqlite => {
                Database::connect(&self.host).await?
            },
        };
        self.pool = Some(pool.clone());
        Ok(pool)
    }
}


impl Default for ApiDBConn {
    fn default() -> Self {
        Self {
            host: "sqlite://data.sqlite?mode=rwc".to_string(),
            pool: Default::default(),
            port: Default::default(),
        }
    }
}
