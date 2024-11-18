#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

use config::{ConfigBuilder, ConfigError, File};

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let builder = ConfigBuilder::<config::builder::DefaultState>::default();

    let settings = builder
        .add_source(File::with_name("configuration"))
        .build()?; // Constrói a configuração

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
