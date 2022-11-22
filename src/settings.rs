use config::{Config, ConfigError};
use serde::Deserialize;
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use time::{macros::format_description, UtcOffset};

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u32,
    pub ip: String,
}

impl Server {
    pub fn get_ip(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}


#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
}

impl Settings {
    fn new() -> Result<Self, ConfigError> {
        const CURRENT_DIR: &str = "./configs/Settings.toml";

        let s = Config::builder().add_source(config::File::with_name(CURRENT_DIR)).build()?;

        s.try_deserialize()
    }

    pub fn init() -> Settings {
        let s = Settings::new().unwrap();

        use tracing_subscriber::{fmt::time::OffsetTime, EnvFilter};
        let local_time = OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
        );

        tracing_subscriber::fmt()
            .with_timer(local_time)
            .init();
        s
    }
}

